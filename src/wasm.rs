
//! WebAssembly bindings: Interpreter and Reader wrapper.

mod conv;
// mod handle;

use wasm_bindgen::prelude::*;
use js_sys;
// use web_sys;

use crate::base::*;
use crate::builtins;
use crate::interpreter;
use crate::reader;

#[wasm_bindgen]
pub struct Interpreter(interpreter::Interpreter);

#[wasm_bindgen]
pub struct Reader(reader::Reader);

#[wasm_bindgen]
impl Interpreter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Interpreter {
        console_error_panic_hook::set_once();
        let mut i = interpreter::Interpreter::default();
        builtins::install(&mut i);
        Interpreter(i)
    }

    // pub fn debug(&self) {
    //     let defs = self.0.all_definitions();
    //     web_sys::console::debug_2(&defs.len().into(), &"definitions".into());
    //     web_sys::console::debug_1(&self.0.stack_ref().clone().into());
    // }

    pub fn run(&mut self) -> Result<(), JsValue> {
        self.0.run().map_err(JsValue::from)
    }

    pub fn reset(&mut self) {
        self.0.reset()
    }

    // pub fn eval_next_from(&mut self, r: &mut Reader) -> Result<(), JsValue> {
    //     let mut body = vec![];
    //     'read: loop {
    //         match r.0.read_next() {
    //             Ok(Some(v)) => body.push(v),
    //             Ok(None) => break 'read,
    //             Err(e) => return Err(Val::from(e).into()),
    //         }
    //     }
    //     self.0.eval_next(Val::from(List::from(body)));
    //     Ok(())
    // }

    pub fn js_define(&mut self, name: String, def: js_sys::Function) {
        self.0.add_builtin(name, |i: &mut interpreter::Interpreter| {
            // match def.clone().call1(&JsValue::UNDEFINED, &Handle(s).into()) {
            // }
            Ok(())
        });
    }

    pub fn stack_pop(&mut self) -> Result<JsValue, JsValue> {
        self.0.stack_pop_val().map(JsValue::from).map_err(JsValue::from)
    }

    pub fn stack_push(&mut self, v: JsValue) {
        self.0.stack_push(conv::from_jsvalue(v));
    }
}

#[wasm_bindgen]
impl Reader {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Reader {
        Reader(reader::Reader::default())
    }

    pub fn read_string(&mut self, s: String) -> Result<JsValue, JsValue> {
        let mut v = vec![];
        self.0.read_into(s.chars(), &mut v)
            .map_err(|e| JsValue::from(format!("{:?}", e)))?;
        Ok(List::from(v).into())
    }

    pub fn complete(self) -> Result<JsValue, JsValue> {
        self.0.complete().map(JsValue::from)
            .map_err(|e| JsValue::from(format!("{:?}", e)))
    }
}

