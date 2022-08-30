
use std::cell::Cell;
use std::rc::Rc;

use crate::base::*;
use crate::list::List;

use super::base::*;

// not sure if these all have to be mut
impl Handle {
    /// See [pause](Self::pause), but the value is given [IsError] metadata
    /// so `error?` will return true.
    pub async fn error(&self, v: impl Value) {
        self.co.yield_(FrameYield::Pause(IsError::add(v))).await;
    }
    /// Pause evaluation and return the given value through [Interpreter::run].
    pub async fn pause(&self, v: impl Value) {
        self.co.yield_(FrameYield::Pause(v.into())).await;
    }
    /// Evaluate a list or function.
    pub async fn eval(&mut self, f: impl EvalOnce) {
        self.co.yield_(FrameYield::Eval(f.into())).await;
    }
    /// Evaluate `child` followed by `body`, but `child` is evaluated
    /// inside `body` as a new stack frame so it can add definitions
    /// without affecting the stack frame that called this function.
    pub async fn eval_child(&mut self, body: List, child: impl EvalOnce) {
        let mut frame = ListFrame::new_body(body);
        frame.childs.push(child.into());
        self.co.yield_(FrameYield::Eval(ChildFrame::ListFrame(frame))).await;
    }
    /// Look up a definition and evaluate it.
    pub async fn call(&mut self, s: impl Into<Symbol>) {
        self.co.yield_(FrameYield::Call(s.into())).await;
    }
    /// Put a value on top of the stack.
    pub async fn stack_push(&mut self, v: impl Value) {
        self.co.yield_(FrameYield::StackPush(v.into())).await;
    }

    async fn stack_op(&self, pop: Option<usize>, reqs: Vec<StackGetRequest>) -> Vec<Val> {
        let mut op = StackGetOp::from_request(pop, reqs);
        loop {
            self.co.yield_(FrameYield::StackGetOp(op.clone())).await;
            if let Some(v) = op.maybe_resolved() {
                return v;
            }
        }
    }

    /// Take the top value off the stack.
    /// No type is assumed or requested.
    pub async fn stack_pop_val(&mut self) -> Val {
        self.stack_op(None, vec![StackGetRequest::Any]).await.pop().unwrap()
    }
    /// Take the top value off the stack.
    /// The resulting value will be of the type requested.
    /// If the stack is empty, the interpreter will pause.
    pub async fn stack_pop<T: Value + ImplValue>(&mut self) -> Vals<T> {
        self.stack_op(None, vec![StackGetRequest::of_type::<T>()]).await.pop().unwrap().try_into().unwrap()
    }
    
    /// Get a copy of the nth-from-top value on the stack without removing it.
    /// See [stack_pop](Self::stack_pop).
    pub async fn stack_nth_val(&self, nth: usize) -> Val {
        self.stack_op(Some(nth), vec![StackGetRequest::Any]).await.pop().unwrap()
    }

    /// Get a copy of nth value of the stack without removing it.
    pub async fn stack_nth<T: Value + ImplValue>(&self, nth: usize) -> Vals<T> {
        self.stack_op(Some(nth), vec![StackGetRequest::of_type::<T>()]).await.pop().unwrap().try_into().unwrap()
    }
    
    /// Get a copy of the top value on the stack without removing it.
    /// See [stack_pop_val](Self::stack_pop_val)
    /// and [stack_nth_val](Self::stack_nth_val).
    pub async fn stack_top_val(&self) -> Val {
        self.stack_nth_val(0).await
    }

    /// Get a copy of the top value of the stack without removing it
    /// (i.e. `stack_nth(0)`).
    /// See [stack_pop](Self::stack_pop).
    pub async fn stack_top<T: Value + ImplValue>(&self) -> Vals<T> {
        self.stack_nth(0).await
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
        self.co.yield_(FrameYield::Uplevel(f.into())).await;
    }
    /// Add a definition in the current stack frame.
    /// It will likely be a [List] or a Rust function.
    pub async fn define(&mut self, name: impl Into<String>, def: impl Eval) {
        self.co.yield_(FrameYield::Define(name.into(), def.into_val())).await;
    }
    /// Add a definition in the current stack frame,
    /// with an associated environment of definitions.
    /// See [local_definitions](Self::local_definitions)
    /// and [all_definitions](Self::all_definitions).
    pub async fn define_closure(&mut self, name: impl Into<String>,
                                body: impl Value, env: DefSet) {
        let v = body.into().with_meta(|m| m.push(ClosureEnv(env)));
        self.co.yield_(FrameYield::Define(name.into(), v)).await;
    }
    async fn get_definitions(&mut self, global: bool) -> DefSet {
        let r = Rc::new(Cell::new(None));
        self.co.yield_(FrameYield::Definitions(global, Rc::clone(&r))).await;
        r.take().unwrap()
    }
    /// Get all definitions defined in the current stack frame.
    /// See [define_closure](Self::define_closure).
    pub async fn local_definitions(&mut self) -> DefSet {
        self.get_definitions(false).await
    }
    /// Get all available definitions.
    /// See [define_closure](Self::define_closure).
    pub async fn all_definitions(&mut self) -> DefSet {
        self.get_definitions(true).await
    }
    /// Look for a definition by the given name.
    pub async fn resolve_definition(&self, name: impl Into<String>) -> Option<Val> {
        let r = Rc::new(Cell::new(None));
        self.co.yield_(FrameYield::ResolveDefinition(name.into(), Rc::clone(&r))).await;
        r.take()
    }
    /// Query the current call stack.
    /// Child frames (with uplevel) are not given.
    /// Each stack frame may have a name;
    /// if so, it is the name of the definition.
    pub async fn call_stack_names(&self) -> Vec<Option<String>> {
        let r = Rc::new(Cell::new(None));
        self.co.yield_(FrameYield::GetCallStack(Rc::clone(&r))).await;
        r.take().unwrap()
    }
}


