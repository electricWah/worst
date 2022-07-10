
mod conv;
mod handle;

use wasm_bindgen::prelude::*;
use js_sys;
use web_sys;

use crate::base::*;
use crate::list::*;
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
        conv::setup();
        let mut i = interpreter::Interpreter::default();
        builtins::install(&mut i);
        Interpreter(i)
    }

    pub fn debug(&self) {
        let defs = self.0.all_definitions();
        web_sys::console::debug_2(&defs.len().into(), &"definitions".into());
        web_sys::console::debug_1(&self.0.stack_ref().clone().into());
    }

    pub fn run(&mut self) -> JsValue {
        self.0.run().into()
    }

    pub fn reset(&mut self) {
        self.0.reset()
    }

    pub fn eval_next_from(&mut self, r: &mut Reader) -> Result<(), JsValue> {
        let mut body = vec![];
        'read: loop {
            match r.0.read_next() {
                Ok(Some(v)) => body.push(v),
                Ok(None) => break 'read,
                Err(e) => return Err(Val::from(e).into()),
            }
        }
        self.0.eval_next(Val::from(List::from(body)));
        Ok(())
    }

    pub fn js_define(&mut self, name: String, def: js_sys::Function) {
        self.0.define(name.clone(), move |i: interpreter::Handle| {
            let def = def.clone();
            async move {
                handle::call(def.clone(), i).await;
            }
        });
    }

    pub fn stack_pop(&mut self) -> JsValue {
        if let Some(v) = self.0.stack_pop_val() {
            v.into()
        } else { JsValue::null() }
    }

    pub fn stack_push(&mut self, v: JsValue) {
        self.0.stack_push(conv::from_jsvalue(v));
    }
}

#[wasm_bindgen]
impl Reader {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Reader {
        Reader(reader::Reader::new())
    }

    pub fn reset(&mut self) {
        std::mem::swap(&mut self.0, &mut reader::Reader::new());
    }

    pub fn write(&mut self, s: String) {
        self.0.write(&mut s.chars());
    }
}

