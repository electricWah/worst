
//! `import` and `export`

use crate::base::*;
use crate::list::*;
use crate::reader;
use crate::builtins::file;
use crate::interpreter::{Interpreter, Handle, DefSet};

fn eval_module(m: List, mut defs: DefSet) -> Result<DefSet, (Val, Interpreter)> {

    let exports = Place::wrap(List::default());
    let exports_inner = exports.clone();

    // define here so it's not in local_definitions
    defs.define("export".into(), move |mut i: Handle| {
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
                if let Some(mut l) = exp.downcast::<List>() {
                    l.push(q);
                    exports.set(l);
                } else {
                    dbg!("export symbol failed");
                }
            } else if let Some(coll) = q.downcast::<List>() {
                if let Some(mut l) = exports.get().downcast::<List>() {
                    for v in coll {
                        l.push(v);
                    }
                    exports.set(l);
                } else {
                    dbg!("export list failed"); //, exports.get());
                }
            } else {
                todo!("export this thing");
            }
        }
    });

    let mut i = Interpreter::default();
    i.add_definitions(&defs);

    i.eval_next(m);
    if let Some(ret) = i.run() {
        return Err((ret, i));
    }

    let all_defs = i.local_definitions();

    let mut exmap = DefSet::default();
    let exports = exports.get();
    if let Some(&true) = exports.downcast_ref::<bool>() {
        exmap = all_defs;
    } else if let Some(l) = exports.downcast::<List>() {
        for ex in l {
            let name = ex.downcast::<Symbol>().unwrap().into();
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
    #[cfg(feature = "enable_fs")] {
        if v.is::<String>() {
            let s = v.downcast::<String>().unwrap();
            if let Ok(f) = file::fs::open_read(s) {
                return Some(Box::new(f));
            } else {
                // maybe interp.error no file?
                return None;
            }
        }
    }

    if !v.is::<Symbol>() { return None; }
    let module_path = v.downcast::<Symbol>().unwrap().to_string();

    #[cfg(feature = "enable_fs")] {
        i.call("WORST_LIBPATH").await;
        let libpath = i.stack_pop::<List>().await.into_inner();
        for lpx in libpath {
            if let Some(lp) = lpx.downcast_ref::<String>() {
                match file::fs::open_read(format!("{lp}/{module_path}.w")) {
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

    let mod_file = format!("{module_path}.w");

    #[cfg(feature = "bundled_fs_embed")]
    if let Some(f) = file::embedded::open_read(&mod_file) {
        return Some(Box::new(f));
    }
    // TODO bundled zip feature

    None
}

/// Install all these functions.
pub fn install(i: &mut Interpreter) {
    // No point having a libpath if the filesystem isn't accessible
    #[cfg(feature = "enable_fs")]
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
            } else if let Some(l) = q.downcast::<List>() {
                l
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
                return i.error("error resolving module".to_string()).await;
            }
        }
    });
}


