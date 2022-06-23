
use crate::base::*;
use crate::list::*;
use crate::reader;
use crate::builtins::file;
use crate::interpreter::{Interpreter, Handle, DefSet};

fn eval_module(m: List, defs: DefSet) -> Result<DefSet, (Val, Interpreter)> {
    let mut i = Interpreter::default();
    for (name, def) in defs.iter() {
        i.define(name, def.clone());
    }

    let exports_orig = Place::wrap(List::default());
    let exports_final = exports_orig.clone();
    i.define("%exports", move |mut i: Handle| {
        let e = exports_orig.clone();
        async move {
            i.stack_push(e.clone()).await;
        }
    });

    i.define("export", |mut i: Handle| async move {
        i.call("%exports").await;
        let mut exports = i.stack_pop::<Place>().await;
        let q = i.quote_val().await;
        let qqerr = q.clone();
        if let Some(&b) = q.downcast_ref::<bool>() {
            if b {
                exports.set(true);
            } else {
                dbg!("not sure how to export #f");
            }
        } else if q.is::<Symbol>() {
            let exp = exports.get();
            if let Some(mut l) = exp.clone().downcast::<List>() {
                l.push(q);
                exports.set(l);
            } else {
                dbg!("export symbol failed", &q, &exp);
            }
        } else if let Some(coll) = q.downcast::<List>() {
            if let Some(mut l) = exports.get().downcast::<List>() {
                for v in coll {
                    l.push(v);
                }
                exports.set(l);
            } else {
                dbg!("export list failed", exports.get());
            }
        } else {
            todo!("export this thing {:?}", qqerr);
        }
    });

    i.eval_next(Val::from(m));
    if let Some(ret) = i.run() {
        return Err((ret, i));
    }

    i.definition_remove("%exports");
    // TODO make sure it's not an overridden 'export' somehow
    i.definition_remove("export");

    let all_defs = i.all_definitions();
    let mut exmap = DefSet::default();
    let exportsion = exports_final.get();
    if let Some(&true) = exportsion.downcast_ref::<bool>() {
        exmap = all_defs;
    } else if let Some(l) = exportsion.downcast::<List>() {
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
    i.define("module-resolve-port", |mut i: Handle| async move {
        let module_path = i.stack_pop::<String>().await;

        #[cfg(feature = "enable_fs")] {
            i.call("WORST_LIBPATH").await;
            let libpath = i.stack_pop::<List>().await;
            for lpx in libpath {
                if let Some(lp) = lpx.downcast_ref::<String>() {
                    match file::fs::open_read(format!("{lp}/{module_path}.w")) {
                        Ok(f) => {
                            i.stack_push(f).await;
                            return;
                        },
                        Err(e) => if e.kind() != std::io::ErrorKind::NotFound {
                            i.stack_push(format!("{e:?}")).await;
                            i.stack_push(false).await;
                            return;
                        },
                    }
                } else {
                    eprintln!("Ignored {lpx:?} in WORST_LIBPATH");
                }
            }
        }

        match file::open_bundled_read(format!("{module_path}.w")) {
            Some(p) => i.stack_push(p).await,
            None => i.stack_push(false).await,
        }
    });
    i.define("import", |mut i: Handle| async move {
        let imports = {
            let q = i.quote_val().await;
            if q.is::<Symbol>() {
                List::from(vec![q])
            } else if let Some(l) = q.downcast::<List>() {
                l
            } else {
                return i.error("expected list or symbol".to_string()).await;
            }
        };
        
        for import in imports {
            if let Some(s) = import.downcast::<Symbol>() {
                let modname = s.clone();
                i.stack_push(String::from(s)).await;
                i.call("module-resolve-port").await;
                let m = i.stack_pop_val().await;
                if let Some(mut read) = ReadValue::try_read(m) {
                    match read_module(&mut read) {
                        Ok(r) => match eval_module(r, i.all_definitions().await) {
                            Ok(defs) => {
                                for (name, def) in defs.iter() {
                                    i.define(name, def.clone()).await;
                                }
                            },
                            Err((v, p)) => {
                                return i.error(dbg!(List::from(vec![
                                    "error in eval_module".to_string().into(),
                                    modname.into(),
                                    v,
                                    p.stack_ref().clone().into(),
                                ]))).await;
                            },
                        },
                        Err(e) => return i.error(e).await,
                    }
                } else {
                    return i.error("error resolving module".to_string()).await;
                }
            } else {
                return i.error("expected symbol in import".to_string()).await;
            }
        }
    });
}


