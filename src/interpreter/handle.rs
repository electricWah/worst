
use std::cell::Cell;
use std::rc::Rc;

use crate::base::*;
use crate::list::List;

use super::base::*;

// not sure if these all have to be mut
impl Handle {
    /// See [pause](Self::pause), but the value is given [IsError] metadata
    /// so `error?` will return true.
    pub async fn error(&self, v: impl Into<Val>) {
        self.co.yield_(FrameYield::Pause(IsError::add(v))).await;
    }
    /// Pause evaluation and return the given value through [Interpreter::run].
    pub async fn pause(&self, v: impl Into<Val>) {
        self.co.yield_(FrameYield::Pause(v.into())).await;
    }
    /// Evaluate a list or function.
    pub async fn eval(&mut self, f: impl EvalOnce) {
        self.co.yield_(FrameYield::Eval(f.into_eval_once())).await;
    }
    /// Evaluate `child` followed by `body`, but `child` is evaluated
    /// inside `body` as a new stack frame so it can add definitions
    /// without affecting the stack frame that called this function.
    pub async fn eval_child(&mut self, body: List, child: impl EvalOnce) {
        self.co.yield_(FrameYield::EvalPre(child.into_eval_once(), body)).await;
    }
    /// Look up a definition and evaluate it.
    pub async fn call(&mut self, s: impl Into<Symbol>) {
        self.co.yield_(FrameYield::Call(s.into())).await;
    }

    // non-mutable so stack_top doesn't have to be mutable
    async fn inner_stack_push(&self, v: impl Into<Val>) {
        self.co.yield_(FrameYield::StackPush(v.into())).await;
    }

    async fn inner_try_stack_pop_val(&self) -> Option<Val> {
        let r = Rc::new(Cell::new(None));
        self.co.yield_(FrameYield::StackPop(Rc::clone(&r))).await;
        r.take()
    }

    async fn inner_stack_pop_val(&self) -> Val {
        loop {
            if let Some(v) = self.inner_try_stack_pop_val().await {
                return v;
            } else {
                self.error("stack-empty".to_symbol()).await;
            }
        }
    }

    /// Take the top value off the stack.
    /// The resulting value will be of the type requested.
    /// If the stack is empty, the interpreter will pause.
    async fn inner_stack_pop<T: Value>(&self) -> Vals<T> {
        loop {
            match self.inner_stack_pop_val().await.try_into() {
                Ok(v) => return v,
                Err(v) => {
                    self.error(List::from(vec![
                        "wrong-type".to_symbol().into(),
                            v, std::any::type_name::<T>().to_string().into(),
                    ])).await;
                },
            }
        }
    }

    /// Put a value on top of the stack.
    pub async fn stack_push(&mut self, v: impl Into<Val>) {
        self.inner_stack_push(v).await
    }

    /// Take the top value off the stack, or `None` if the stack is empty.
    pub async fn try_stack_pop_val(&mut self) -> Option<Val> {
        self.inner_try_stack_pop_val().await
    }

    /// Take the top value off the stack, or error with `stack-empty`.
    /// Calling code may put a value on the stack and resume the interpreter.
    pub async fn stack_pop_val(&mut self) -> Val {
        self.inner_stack_pop_val().await
    }

    /// Take the top value off the stack.
    /// The resulting value will be of the type requested.
    /// If the stack is empty, the interpreter will pause.
    pub async fn stack_pop<T: Value>(&mut self) -> Vals<T> {
        self.inner_stack_pop().await
    }

    /// Get a copy of the top value on the stack without removing it.
    pub async fn stack_top_val(&self) -> Val {
        let v = self.inner_stack_pop_val().await;
        self.inner_stack_push(v.clone()).await;
        v
    }

    /// Get a copy of the top value of the stack without removing it
    /// (i.e. `stack_nth(0)`).
    /// See [stack_pop](Self::stack_pop).
    pub async fn stack_top<T: Value>(&self) -> Vals<T> {
        let v = self.inner_stack_pop::<T>().await;
        self.inner_stack_push(v.get_val()).await;
        v
    }

    /// The current state of the stack, as a list (cloned).
    pub async fn stack_get(&self) -> List {
        let r = Rc::new(Cell::new(None));
        self.co.yield_(FrameYield::StackGetAll(Rc::clone(&r))).await;
        r.take().unwrap()
    }

    /// Whether the stack is empty.
    pub async fn stack_empty(&self) -> bool {
        self.stack_get().await.is_empty()
    }
    /// Get the size of the stack.
    pub async fn stack_len(&self) -> usize {
        self.stack_get().await.len()
    }
    /// Quote the next value in the current body.
    /// If there is none, the interpreter will error with "quote-nothing".
    pub async fn quote_val(&mut self) -> Val {
        let r = Rc::new(Cell::new(None));
        loop {
            self.co.yield_(FrameYield::Quote(Rc::clone(&r))).await;
            if let Some(q) = r.take() { return q; }
        }
    }
    /// Evaluate the given thingy in the parent stack frame.
    /// The interpreter will pause, likely indefinitely,
    /// if there is no parent stack frame.
    pub async fn uplevel(&mut self, f: impl EvalOnce) {
        self.co.yield_(FrameYield::Uplevel(f.into_eval_once())).await;
    }
    /// Add a definition in the current stack frame.
    /// It will likely be a [List] or a Rust function.
    pub async fn define(&mut self, name: impl Into<String>, def: impl Eval) {
        self.co.yield_(FrameYield::Define {
            name: name.into(),
            scope: DefScope::Static,
            def: def.into_val(),
        }).await;
    }
    /// Get all definitions defined in the current stack frame.
    /// See [define_closure](Self::define_closure).
    pub async fn local_definitions(&mut self) -> DefSet {
        self.get_defs(DefScope::Static, false).await
    }
    /// Get all available definitions.
    /// See [define_closure](Self::define_closure).
    pub async fn all_definitions(&mut self) -> DefSet {
        self.get_defs(DefScope::Static, true).await
    }
    /// Look for a definition by the given name.
    pub async fn resolve_definition(&self, name: impl Into<String>) -> Option<Val> {
        self.get_def(name.into(), DefScope::Static, true).await
    }
    
    /// Define a dynamic value,
    /// which is a species of value that lives in stack frames, and thus
    /// lacks the lexical scope gene that other values and definitions carry.
    pub async fn define_dynamic(&mut self, name: impl Into<String>, def: impl Into<Val>) {
        self.co.yield_(FrameYield::Define {
            name: name.into(),
            scope: DefScope::Dynamic,
            def: def.into(),
        }).await;
    }

    /// Get the dynamic value of the given name,
    /// searching first in the current stack frame
    /// and then in parent stack frames if it cannot be found.
    pub async fn resolve_dynamic(&self, name: impl Into<String>) -> Option<Val> {
        self.get_def(name.into(), DefScope::Dynamic, true).await
    }

    /// Get all dynamic values in the current stack frame.
    pub async fn get_dynamics(&self) -> DefSet {
        self.get_defs(DefScope::Dynamic, false).await
    }

    /// Remove a dynamic value of the given name from the current stack frame.
    /// Returns it on success.
    pub async fn remove_dynamic(&mut self, name: impl Into<String>) -> Option<Val> {
        self.remove_def(name.into(), DefScope::Dynamic).await
    }

    // dynamic_depth    - Find out how many stack frames up a dynamic is set
    // all_dynamics     - Get all dynamic names and defs

    /// Query the current call stack.
    /// Child frames (with uplevel) are not given.
    /// Each stack frame may have a name;
    /// if so, it is the name of the definition.
    pub async fn call_stack_names(&self) -> Vec<Option<String>> {
        let r = Rc::new(Cell::new(None));
        self.co.yield_(FrameYield::GetCallStack(Rc::clone(&r))).await;
        r.take().unwrap()
    }


    async fn get_defs(&self, scope: DefScope, all: bool) -> DefSet {
        let r = Rc::new(Cell::new(None));
        self.co.yield_(FrameYield::Definitions {
            scope, all, ret: Rc::clone(&r),
        }).await;
        r.take().unwrap()
    }

    async fn get_def(&self, name: String, scope: DefScope, resolve: bool) -> Option<Val> {
        let r = Rc::new(Cell::new(None));
        self.co.yield_(FrameYield::GetDefinition {
            name, scope, resolve, ret: Rc::clone(&r),
        }).await;
        r.take()
    }

    async fn remove_def(&self, name: String, scope: DefScope) -> Option<Val> {
        let r = Rc::new(Cell::new(None));
        self.co.yield_(FrameYield::RemoveDefinition {
            name, scope, ret: Rc::clone(&r),
        }).await;
        r.take()
    }

}


