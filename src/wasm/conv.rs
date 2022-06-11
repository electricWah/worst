
use std::sync::Once;

use wasm_bindgen::prelude::*;
use js_sys::{ self, Array };
use web_sys;

use crate::impl_value;
use crate::base::*;
use crate::list::List;

impl_value!(JsValue);

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
        List::from_iter(Array::from(&j).iter().map(from_jsvalue)).into()
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
impl_value!(ToJsValue);

// For values that don't implement ToJsValue
#[wasm_bindgen]
pub struct InternalVal(Val);

impl From<Val> for JsValue {
    fn from(v: Val) -> JsValue {
        if let Some(jsv) = v.type_meta().first::<ToJsValue>() {
            let vv = jsv.0(&v);
            // web_sys::console::warn_3(&"into jsvalue".into(), &format!("{:?}", &v).into(), &vv);
            vv
        } else {
            web_sys::console::warn_2(&"no ToJsValue for this".into(),
                &format!("{:?}", &v).into());
            InternalVal(v).into()
        }
    }
}

pub fn value_tojsvalue<T: ImplValue + Value + Clone + Into<JsValue>>() -> impl Value {
    ToJsValue(Box::new(|v: &Val| {
        v.downcast_ref::<T>().unwrap().clone().into()
    }))
}

fn to_jsvalue<T: ImplValue + Value + Clone + Into<JsValue>>() {
    T::install_meta(value_tojsvalue::<T>());
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

// install ToJsValue on supported types
static TOJSVALUE: Once = Once::new();

pub fn setup() {
    TOJSVALUE.call_once(|| {
        to_jsvalue::<bool>();
        to_jsvalue::<String>();
        to_jsvalue::<Symbol>();
        to_jsvalue::<i32>();
        to_jsvalue::<f64>();
        to_jsvalue::<List>();
        to_jsvalue::<JsValue>();
    });
}


