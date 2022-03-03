
use std::io::Read;

use crate::base::*;
use crate::list::*;
use crate::reader;
use crate::interpreter::{Builder, Handle};

fn resolve_module(path: String, libpath: &Vec<String>) -> Option<List<Val>> {
    let mut resolve_errors = vec![];
    for base in libpath.iter() {
        let filepath = format!("{}/{}.w", base, path);
        match std::fs::File::open(&filepath) {
            Ok(mut f) => {
                let mut s = String::new();
                match f.read_to_string(&mut s) {
                    Ok(_) => {
                        match reader::read_all(&mut s.chars()) {
                            Ok(data) => return Some(data.into()),
                            Err(e) => todo!("{:?}", e),
                        }
                    },
                    Err(e) => resolve_errors.push((filepath, e)),
                }
            },
            Err(e) => resolve_errors.push((filepath, e)),
        }
    }
    dbg!(resolve_errors);
    None // TODO Ok(List) | Err([name + resolve_error]) | Err(read_error)
}

pub fn install(i: Builder) -> Builder {
    i
    .define("WORST_LIBPATH", |mut i: Handle| async move {
        let s =
            if let Ok(s) = std::env::var("WORST_LIBPATH") { s }
            else { String::new() };
        i.stack_push(List::from_vals(s.split(':').map(String::from))).await;
    })
    .define("import", |mut i: Handle| async move {
        let imports =
            if let Some(q) = i.quote().await {
                match q.downcast::<List<Val>>() {
                    Ok(l) => *l,
                    Err(qq) =>
                        match qq.downcast::<Symbol>() {
                            Ok(s) => List::from(vec![(*s).into()]),
                            Err(_) => {
                                i.stack_push("expected list or symbol").await;
                                return i.pause().await;
                            }
                        },
                }
            } else {
                return i.stack_push("quote-nothing".to_symbol()).await;
            };
        i.call("WORST_LIBPATH").await;
        let libpath =
            match i.stack_pop::<List<Val>>().await {
                Some(lp) => {
                    let mut v = vec![];
                    for l in lp {
                        if let Ok(s) = l.downcast::<String>() {
                            v.push(*s);
                        } else {
                            i.stack_push("WORST_LIBPATH contained a not-string").await;
                            return i.pause().await;
                        }
                    }
                    v
                },
                None => {
                    i.stack_push("WORST_LIBPATH was not a list").await;
                    return i.pause().await;
                },
            };
        
        for import in imports {
            if let Ok(s) = import.downcast::<Symbol>() {
                if let Some(r) = resolve_module((*s).into(), &libpath) {
                    i.stack_push(r).await;
                } else {
                    i.stack_push("couldn't resolve module").await;
                    return i.pause().await;
                }
            } else {
                i.stack_push("expected symbol in import").await;
                return i.pause().await;
            }
        }
    })
}


