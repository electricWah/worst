
use std::sync::mpsc::{ self, Sender };

use wasm_bindgen::prelude::*;
use js_sys;
use web_sys;

use crate::base::*;
use crate::interpreter;

use crate::wasm::conv;

#[derive(Clone)]
enum Event {
    StackPop,
    StackPush(Val),
    Complete
}

#[wasm_bindgen]
pub struct Handle(Sender<(Event, js_sys::Function)>);

#[wasm_bindgen]
impl Handle {
    fn send(&self, ev: Event) -> js_sys::Promise {
        js_sys::Promise::new(&mut |res, rej: js_sys::Function| {
            match self.0.send((ev.clone(), res)) {
                Ok(()) => {},
                Err(e) => {
                    // assume Promise.reject will not error
                    let _ = rej.call1(&JsValue::UNDEFINED, &format!("send error: {e:?}").into());
                }
            }
        })
    }

    pub fn stack_push(&self, j: JsValue) -> js_sys::Promise {
        self.send(Event::StackPush(conv::from_jsvalue(j)))
    }

    pub fn stack_pop(&self) -> js_sys::Promise {
        self.send(Event::StackPop)
    }
    pub fn complete(&self) -> js_sys::Promise {
        self.send(Event::Complete)
    }
}

pub async fn call(def: js_sys::Function, mut i: interpreter::Handle) {
    let (s, r) = mpsc::channel();
    // should be a Promise so the outer run loop will wait on this
    match def.clone().call1(&JsValue::UNDEFINED, &Handle(s).into()) {
        Ok(p) => i.pause(p).await,
        Err(e) => i.error(e).await,
    }
    'waiter: loop {
        match r.try_recv() {
            Ok((Event::StackPush(v), res)) => {
                i.stack_push(v).await;
                // ignore promise resolve
                let _ = res.call0(&JsValue::UNDEFINED);
                i.pause(JsValue::from(js_sys::Promise::resolve(&JsValue::UNDEFINED))).await;
            },
            Ok((Event::StackPop, res)) => {
                let v = i.stack_pop_val().await.into();
                // ignore promise resolve
                let _ = res.call1(&JsValue::UNDEFINED, &v);
                i.pause(JsValue::from(js_sys::Promise::resolve(&JsValue::UNDEFINED))).await;
            },
            Ok((Event::Complete, res)) => {
                // ignore promise resolve
                let _ = res.call0(&JsValue::UNDEFINED);
                break 'waiter;
            },
            Err(mpsc::TryRecvError::Empty) => {
                web_sys::console::error_1(&"empty(??)".into());
                break 'waiter;
            },
            Err(e) => {
                i.error(format!("{e:?}")).await;
            },
        }
    }
}

