
use std::io::Read;

use crate::base::*;
use crate::list::*;
use crate::reader;
use crate::interpreter::{Builder, Paused, Handle, DefSet};

fn eval_module(m: List, defs: DefSet) -> Result<DefSet, Paused> {
    let mut ib = Builder::default();
    for (name, def) in defs.iter() {
        ib.define(name, def.clone());
    }

    let exports_orig = Place::wrap(List::default());
    let exports_final = exports_orig.clone();
    ib.define("%exports", move |mut i: Handle| {
        let e = exports_orig.clone();
        async move {
            i.stack_push(e.clone()).await;
        }
    });

    ib.define("export", |mut i: Handle| async move {
        i.call("%exports").await;
        let mut exports = i.stack_pop::<Place>().await;
        if let Some(q) = i.quote().await {
            if let Some(&b) = q.downcast_ref::<bool>() {
                if b {
                    exports.set(true);
                } else {
                    dbg!("not sure how to export #f");
                }
            } else if q.is::<Symbol>() {
                match exports.get().downcast::<List>() {
                    Ok(mut l) => {
                        l.push(q);
                        exports.set(l);
                    },
                    Err(oe) => {
                        dbg!("export symbol failed", &q, &oe);
                    },
                }
            } else {
                match q.downcast::<List>() {
                    Ok(coll) => {
                        match exports.get().downcast::<List>() {
                            Ok(mut l) => {
                                for v in coll {
                                    l.push(v);
                                }
                                exports.set(l);
                            },
                            Err(oe) => {
                                dbg!("export list failed", &oe);
                            },
                        }
                    },
                    Err(e) => {
                        todo!("export this thing {:?}", e);
                    },
                }
            }
        } else {
            i.stack_push("quote-nothing".to_symbol()).await;
            return i.pause().await;
        }
    });

    let mut i = ib.eval(Val::from(List::from(m)));
    while !i.run() {
        return Err(i);
    }

    i.definition_remove("%exports");
    // TODO make sure it's not an overridden 'export' somehow
    i.definition_remove("export");

    let all_defs = i.all_definitions();
    let mut exmap = DefSet::default();
    let exportsion = exports_final.get();
    if let Some(&true) = exportsion.downcast_ref::<bool>() {
        exmap = all_defs;
    } else {
        match exportsion.downcast::<List>() {
            Ok(l) => {
                for ex in l.into_iter() {
                    let name = ex.downcast::<Symbol>().unwrap().into();
                    if let Some(def) = all_defs.get(&name) {
                        exmap.insert(name, def.clone());
                    } else {
                        dbg!("coudldn't see def", name);
                    }
                }
            },
            Err(e) => todo!("exporting failure {:?}", e)
        }
    }
    Ok(exmap)
}

fn resolve_module(path: String, libpath: &Vec<String>) -> Option<List> {
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

pub fn install(mut i: Builder) -> Builder {
    i.define("WORST_LIBPATH", |mut i: Handle| async move {
        let s =
            if let Ok(s) = std::env::var("WORST_LIBPATH") { s }
            else { String::new() };
        i.stack_push(List::from_iter(s.split(':').map(String::from))).await;
    });
    i.define("import", |mut i: Handle| async move {
        let imports =
            if let Some(q) = i.quote().await {
                if q.is::<Symbol>() {
                    List::from(vec![q])
                } else {
                    match q.downcast::<List>() {
                        Ok(l) => l,
                        Err(_e) => {
                            i.stack_push("expected list or symbol").await;
                            return i.pause().await;
                        },
                    }
                }
            } else {
                return i.stack_push("quote-nothing".to_symbol()).await;
            };
        i.call("WORST_LIBPATH").await;
        let libpath = {
            let lp = i.stack_pop::<List>().await;
            let mut v = vec![];
            for l in lp {
                if let Ok(s) = l.downcast::<String>() {
                    v.push(s);
                } else {
                    i.stack_push("WORST_LIBPATH contained a not-string").await;
                    return i.pause().await;
                }
            }
            v
        };
        
        for import in imports {
            if let Ok(s) = import.downcast::<Symbol>() {
                let modname = s.clone();
                if let Some(r) = resolve_module(s.into(), &libpath) {
                    match eval_module(r, i.all_definitions().await) {
                        Ok(defs) => {
                            for (name, def) in defs.iter() {
                                i.define(name, def.clone()).await;
                            }
                        },
                        Err(p) => {
                            dbg!(modname, p.stack_ref());
                            i.stack_push("error in eval_module").await;
                            return i.pause().await;
                        },
                    }
                } else {
                    i.stack_push("couldn't resolve module").await;
                    return i.pause().await;
                }
            } else {
                i.stack_push("expected symbol in import").await;
                return i.pause().await;
            }
        }
    });
    i
}


