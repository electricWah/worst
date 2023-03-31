
//! WebAssembly functions and such: wrappers for wasm_bindgen, etc.

use wasm_bindgen::prelude::*;
use js_sys;
// use web_sys;

use crate::base::*;
use crate::builtins;
use crate::interpreter::*;
use crate::reader::*;

#[wasm_bindgen]
struct JsVal(Val);
impl Value for JsVal {}

impl From<JsValue> for Val {
    fn from(j: JsValue) -> Val {
        if j.is_falsy() {
            false.into()
        } else if let Some(v) = j.as_bool() {
            v.into()
        } else if let Some(v) = j.as_string() {
            v.into()
        } else if let Some(v) = j.as_f64() {
            v.into()
        } else if j.is_symbol() {
            Symbol::from(js_sys::Symbol::from(j).as_string().expect("Symbol is not utf8!")).into()
        } else if js_sys::Array::is_array(&j) {
            List::from(js_sys::Array::from(&j).iter().map(Val::from).collect::<Vec<_>>()).into()
                // } else if let Ok(Sym { symbol }) = j.into_serde() {
                //     // js ["test"] will successfully read as Symbol("test")
                //     // so careful with multiple serde values in this else/if chain
                //     Symbol::from(symbol).into()
        // } else if j.is_function() {
        //     j.into()
        } else {
            web_sys::console::warn_2(&"unknown jsvalue".into(), &j);
            j.into()
        }
    }
}

impl From<Val> for JsValue {
    fn from(v: Val) -> JsValue {
        if v.is::<bool>() {
            JsValue::from(v.try_downcast::<bool>().ok().unwrap().into_inner())
        } else if v.is::<String>() {
            JsValue::from(v.try_downcast::<String>().ok().unwrap().into_inner())
        } else if v.is::<Symbol>() {
            JsValue::from(v.try_downcast::<Symbol>().ok().unwrap().into_inner())
        } else if v.is::<f64>() {
            JsValue::from(v.try_downcast::<f64>().ok().unwrap().into_inner())
        } else if v.is::<List>() {
            JsValue::from(v.try_downcast::<List>().ok().unwrap().into_inner())
        } else {
            web_sys::console::warn_1(&"no ToJsValue for value".into());
            JsVal(v.into()).into()
        }
    }
}

impl From<List> for JsValue {
    fn from(l: List) -> JsValue {
        let a = js_sys::Array::new();
        for v in l.into_iter() {
            a.push(&v.into());
        }
        a.into()
    }
}

impl From<Symbol> for JsValue {
    fn from(s: Symbol) -> JsValue {
        JsValue::symbol(Some(s.as_ref()))
    }
}

#[wasm_bindgen]
impl JsVal {
    #[wasm_bindgen(js_name = unwrap)]
    pub fn js_unwrap(self) -> JsValue { self.0.into() }
}

impl TryFrom<JsValue> for JsVal {
    type Error = JsError;
    fn try_from(_v: JsValue) -> Result<Self, Self::Error> {
        Err(JsError::new("TODO JsVal try_from"))
    }
}

impl From<Val> for JsVal {
    fn from(v: Val) -> JsVal { JsVal(v) }
}

#[wasm_bindgen]
impl Interpreter {
    /// Create a new, empty Interpreter.
    #[wasm_bindgen(constructor)]
    pub fn js_constructor() -> Interpreter {
        console_error_panic_hook::set_once();
        let mut i = Interpreter::default();
        builtins::install(&mut i);
        i
    }

    // pub fn debug(&self) {
    //     let defs = self.0.all_definitions();
    //     web_sys::console::debug_2(&defs.len().into(), &"definitions".into());
    //     web_sys::console::debug_1(&self.0.stack_ref().clone().into());
    // }

    /// Run until the next pause or error, or to completion.
    #[wasm_bindgen(js_name = run)]
    pub fn js_run(&mut self) -> Result<(), JsValue> {
        self.run().map_err(Val::into)
    }

    /// Make the interpreter stop doing things,
    /// but leave its toplevel definitions intact.
    #[wasm_bindgen(js_name = reset)]
    pub fn js_reset(&mut self) { self.reset() }

    /// Add a function, array, or [Val] as a new definition.
    #[wasm_bindgen(js_name = define)]
    pub fn js_define(&mut self, name: String, def: JsValue) -> Result<(), JsError> {
        if let Some(f) = def.dyn_ref::<js_sys::Function>() {
            let def = f.clone();
            self.add_builtin(name, move |i: &mut Interpreter| {
                let res = {
                    let ii: *const Interpreter = i;
                    def.clone().call1(&JsValue::UNDEFINED, &JsValue::from(ii))
                };
                if let Err(e) = res {
                    i.error(Val::from(e))?;
                }
                Ok(())
            });
            Ok(())
        } else if let Some(_a) = def.dyn_ref::<js_sys::Array>() {
            Err(JsError::new("todo array def"))
        } else if let Ok(_v) = JsVal::try_from(def) {
            Err(JsError::new("todo val def"))
        } else {
            Err(JsError::new("todo this thing"))
        }
    }

    /// Pop and return the value on top of the stack.
    // TODO pass `true` to keep it as a Val with metadata but with extra unwrap() needed
    #[wasm_bindgen(js_name = stackPop)]
    pub fn js_stack_pop(&mut self) -> JsValue {
        self.stack_pop_val().ok()
            .map(JsValue::from)
            .unwrap_or(JsValue::UNDEFINED)
    }

    /// Push something on top of the stack.
    #[wasm_bindgen(js_name = stackPush)]
    pub fn js_stack_push(&mut self, v: JsValue) {
        self.stack_push(v)
    }
}

#[wasm_bindgen]
impl Reader {
    /// Create a new, empty reader.
    #[wasm_bindgen(constructor)]
    pub fn js_constructor() -> Reader { Reader::default() }

    /// Read a string into a list.
    #[wasm_bindgen(js_name = read)]
    pub fn js_read_string(&mut self, s: String) -> Result<JsValue, JsError> {
        let mut v = vec![];
        self.read_into(s.chars(), &mut v)
            .map_err(|e| JsError::new(&format!("{:?}", e)))?;
        Ok(JsVal(List::from(v).into()).into())
    }

    /// Tell the reader there's nothing left to read
    /// and return any half-complete values or errors (e.g. unclosed lists).
    /// Create a new reader to start reading values again.
    #[wasm_bindgen(js_name = complete)]
    pub fn js_complete(self) -> Result<JsValue, JsError> {
        self.complete().map(|v|
                            v.map(JsVal::from).map(JsValue::from)
                            .unwrap_or(JsValue::UNDEFINED))
            .map_err(|e| JsError::new(&format!("{:?}", e)))
    }
}

