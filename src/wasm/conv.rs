
use wasm_bindgen::prelude::*;
use js_sys::{ self, Array };
use web_sys;

use crate::base::*;

impl Value for JsValue {}

// Never fails since any failure is just a JsValue itself
pub fn from_jsvalue(j: JsValue) -> Val {
    // web_sys::console::warn_2(&"jsvalue".into(), &j);
    if j.is_falsy() {
        false.into()
    } else if let Some(v) = j.as_bool() {
        v.into()
    } else if let Some(v) = j.as_string() {
        v.into()
    } else if let Some(v) = j.as_f64() {
        v.into()
    } else if Array::is_array(&j) {
        List::from(Array::from(&j).iter().map(from_jsvalue).collect::<Vec<_>>()).into()
    // } else if let Ok(Sym { symbol }) = j.into_serde() {
    //     // js ["test"] will successfully read as Symbol("test")
    //     // so careful with multiple serde values in this else/if chain
    //     Symbol::from(symbol).into()
    } else if j.is_function() {
        j.into()
    } else {
        web_sys::console::warn_2(&"unknown jsvalue".into(), &j);
        j.into()
    }
}

struct ToJsValue(Box<dyn Fn(&Val) -> JsValue>);
impl Value for ToJsValue {}

// For values that don't implement ToJsValue
#[wasm_bindgen]
pub struct InternalVal(Val);

impl From<Val> for JsValue {
    fn from(v: Val) -> JsValue {
        if v.is::<bool>() {
            JsValue::from(v.try_downcast::<bool>().ok().unwrap().into_inner())
        } else if v.is::<String>() {
            JsValue::from(v.try_downcast::<String>().ok().unwrap().into_inner())
        // } else if v.is::<Symbol>() {
        //     JsValue::from(v.try_downcast::<Symbol>().unwrap().into_inner())
        } else if v.is::<f64>() {
            JsValue::from(v.try_downcast::<f64>().ok().unwrap().into_inner())
        } else if v.is::<List>() {
            JsValue::from(v.try_downcast::<List>().ok().unwrap().into_inner())
        } else {
            web_sys::console::warn_1(&"no ToJsValue for value".into());
            InternalVal(v).into()
        }
    }
}

impl From<List> for JsValue {
    fn from(l: List) -> JsValue {
        let a = Array::new();
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

