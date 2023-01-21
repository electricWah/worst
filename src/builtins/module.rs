
//! `import` and `export`

use crate::base::*;
use crate::reader;
use crate::builtins::fs;
use crate::interpreter::{Builtin, Interpreter, Handle, DefSet};

fn eval_module(m: List, mut defs: DefSet) -> Result<DefSet, (Val, Interpreter)> {

    let exports = Place::wrap(List::default());
    let exports_inner = exports.clone();

    // define here so it's not in local_definitions
    defs.insert("export", Builtin::from(move |mut i: Handle| {
        let exports = exports_inner.clone();
        async move {
            let mut exports = exports.clone();
            let q = i.quote_val().await;
            if let Some(&b) = q.downcast_ref::<bool>() {
                if b {
                    exports.set(true);
                } else {
                    dbg!("not sure how to export #f");
                }
            } else if q.is::<Symbol>() {
                let exp = exports.get();
                if let Ok(mut l) = exp.try_downcast::<List>() {
                    l.as_mut().push(q);
                    exports.set(l);
                } else {
                    dbg!("export symbol failed");
                }
            } else if let Ok(coll) = q.try_downcast::<List>() {
                if let Ok(mut l) = exports.get().try_downcast::<List>() {
                    for v in coll.into_inner() {
                        l.as_mut().push(v);
                    }
                    exports.set(l);
                } else {
                    dbg!("export list failed"); //, exports.get());
                }
            } else {
                todo!("export this thing");
            }
        }
    }));

    let mut i = Interpreter::default();
    i.defenv_mut().append(&defs);

    i.eval_list_next(m.into());
    if let Some(ret) = i.run() {
        return Err((ret, i));
    }

    let all_defs = i.local_definitions();

    let mut exmap = DefSet::default();
    let exports = exports.get();
    if let Some(&true) = exports.downcast_ref::<bool>() {
        exmap = all_defs;
    } else if let Ok(l) = exports.try_downcast::<List>() {
        for ex in l.into_inner() {
            let name = ex.try_downcast::<Symbol>().ok().unwrap().into_inner();
            if let Some(def) = all_defs.get(&name) {
                exmap.insert(name, def.clone());
            } else {
                dbg!("coudldn't see def", name);
            }
        }
    } else {
        todo!("exporting failure");
    }
    Ok(exmap)
}

fn read_module(read: &mut dyn std::io::Read) -> Result<List, String> {
    let mut s = String::new();
    match read.read_to_string(&mut s) {
        Ok(_) => {
            match reader::read_all(&mut s.chars()) {
                Ok(data) => Ok(data.into()),
                Err(e) => Err(format!("{:?}", e)),
            }
        },
        Err(e) => Err(format!("{}", e)),
    }
}

// TODO should be Result?
async fn resolve_import(i: &mut Handle, v: Val) -> Option<Box<dyn std::io::Read>> {

    // if it's a string, load the file
    #[cfg(feature = "enable_fs_os")] {
        if v.is::<String>() {
            let s = v.try_downcast::<String>().ok().unwrap().into_inner();
            if let Ok(f) = fs::os::open_read(s) {
                return Some(Box::new(f));
            } else {
                // maybe interp.error no file?
                return None;
            }
        }
    }

    if !v.is::<Symbol>() { return None; }
    let module_path = v.try_downcast::<Symbol>().ok().unwrap().into_inner().to_string();

    #[cfg(feature = "enable_fs_os")] {
        i.call("WORST_LIBPATH").await;
        let libpath = i.stack_pop::<List>().await.into_inner();
        for lpx in libpath {
            if let Some(lp) = lpx.downcast_ref::<String>() {
                match fs::os::open_read(format!("{lp}/{module_path}.w")) {
                    Ok(f) => {
                        return Some(Box::new(f));
                    },
                    Err(e) => if e.kind() != std::io::ErrorKind::NotFound {
                        // TODO for now - should just interp.error
                        dbg!(e);
                        return None;
                    },
                }
            } else {
                todo!("Ignored in WORST_LIBPATH");
            }
        }
    }


    #[cfg(feature = "enable_fs_embed")] {
        let mod_file = format!("{module_path}.w");
        if let Some(f) = fs::embed::open_read(&mod_file) {
            return Some(Box::new(f));
        }
    }
    // TODO bundled zip feature

    None
}

/// Install all these functions.
pub fn install(i: &mut Interpreter) {
    // No point having a libpath if the filesystem isn't accessible
    #[cfg(feature = "enable_fs_os")]
    i.define("WORST_LIBPATH", |mut i: Handle| async move {
        if let Ok(s) = std::env::var("WORST_LIBPATH") {
            i.stack_push(List::from_iter(s.split(':').map(String::from))).await;
        } else {
            i.stack_push(List::default()).await;
        }
    });
    i.define("import", |mut i: Handle| async move {
        let imports = {
            let q = i.quote_val().await;
            if q.is::<Symbol>() || q.is::<String>() {
                List::from(vec![q])
            } else if let Ok(l) = q.try_downcast::<List>() {
                l.into_inner()
            } else {
                return i.error("expected list, symbol or string".to_string()).await;
            }
        };
        
        for import in imports {
            let import_name = import.clone();
            if let Some(mut read) = resolve_import(&mut i, import).await {
                match read_module(&mut read) {
                    Ok(r) => match eval_module(r, i.all_definitions().await) {
                        Ok(defs) => {
                            for (name, def) in defs.iter() {
                                i.add_definition(name, def.clone()).await;
                            }
                        },
                        Err((v, p)) => {
                            return i.error(List::from(vec![
                                "error in eval_module".to_string().into(),
                                import_name,
                                v,
                                p.stack_ref().clone().into(),
                            ])).await;
                        },
                    },
                    Err(e) => return i.error(e).await,
                }
            } else {
                return i.error(List::from(vec![
                    "error resolving module".to_string().into(),
                    import_name,
                ])).await;
            }
        }
    });
}


