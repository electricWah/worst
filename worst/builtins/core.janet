
(import ../data)
(import ../interpreter :as I)

(defn dump :builtin [i] (pp (I/stack i)))

(defn drop :builtin [i] (I/stack-pop i))

(defn swap :builtin [i]
  (let [b (I/stack-pop i)
        a (I/stack-pop i)]
    (I/stack-push i b)
    (I/stack-push i a)))

(defn clone :builtin [i]
  (let [a (I/stack-pop i)]
    (I/stack-push i a)
    (I/stack-push i a)))

(defn dig :builtin [i]
  (let [a (I/stack-pop i)
        b (I/stack-pop i)
        c (I/stack-pop i)]
    (I/stack-push i b)
    (I/stack-push i a)
    (I/stack-push i c)))

(defn bury :builtin [i]
  (let [a (I/stack-pop i)
        b (I/stack-pop i)
        c (I/stack-pop i)]
    (I/stack-push i a)
    (I/stack-push i c)
    (I/stack-push i b)))

(defn not :builtin [i]
  (let [a (I/stack-pop i)]
    (I/stack-push i (not a))))

(defn eval :builtin [i]
  (let [a (I/stack-pop i)]
    (I/eval-next i a)))

(defn eval-if :builtin [i]
  (let [a (I/stack-pop i)
        b (I/stack-pop i)]
    (when b
      (I/eval-next i a))))

(defn uplevel :builtin [i]
  (I/enter-parent-frame i)
  (eval i))

(defn pause :builtin :pause [i]
  (let [a (I/stack-pop i)]
    a))

(defn stack-get :builtin [i]
  (let [s (I/stack i)]
    (I/stack-push i s)))

(defn stack-set :builtin [i]
  (let [s (I/stack-pop i)]
    (I/stack-set i s)))

(defn code-next :builtin [i]
  (let [c (I/code-next i)]
    (I/stack-push i c)))

(defn code-peek :builtin [i]
  (let [c (I/code-peek i)]
    (I/stack-push i c)))

(defn quote :builtin [i] (code-next i))

(defn upquote :builtin [i]
  (I/enter-parent-frame i)
  (code-next i))

#    i.add_builtin("value-insert-meta-entry", |i: &mut Interpreter| {
#        let mv = i.stack_pop_val()?;
#        let u = i.stack_pop::<Unique>()?.into_inner();
#        let mut v = i.stack_pop_val()?;
#        v.meta_mut().insert_val(u, mv);
#        i.stack_push(v);
#        Ok(())
#    });

#/// Install all these functions.
#pub fn install(i: &mut Interpreter) {
#    i.add_builtin("quote", quote);
#    i.add_builtin("upquote", upquote);
#    util::add_const_type_builtin::<IsError>(i, "<is-error>");
#    util::add_const_type_builtin::<bool>(i, "<bool>");
#    i.add_builtin("bool-equal", util::equality::<bool>);
#    i.add_builtin("bool-hash", util::value_hash::<bool>);
#    util::add_const_type_builtin::<Symbol>(i, "<symbol>");
#    i.add_builtin("symbol-equal", util::equality::<Symbol>);
#    i.add_builtin("symbol-hash", util::value_hash::<Symbol>);

#    util::add_const_type_builtin::<Unique>(i, "<unique>");
#    i.add_builtin("unique-equal", util::equality::<Unique>);
#    i.add_builtin("unique-hash", util::value_hash::<Unique>);
#    i.add_builtin("make-unique", |i: &mut Interpreter| {
#        let u = i.uniques_mut().create();
#        i.stack_push(u);
#        Ok(())
#    });

#    util::add_const_type_builtin::<Builtin>(i, "<builtin>");

#    util::add_const_type_builtin::<TypeId>(i, "<type-id>");
#    i.add_builtin("type-id-equal", util::equality::<TypeId>);
#    i.add_builtin("type-id-hash", util::value_hash::<TypeId>);
#    i.add_builtin("value-type-id", |i: &mut Interpreter| {
#        let v = i.stack_pop_val()?;
#        i.stack_push(v.val_type_id());
#        Ok(())
#    });
#    i.add_builtin("type-id->unique", |i: &mut Interpreter| {
#        let v = i.stack_pop::<TypeId>()?.into_inner();
#        let u = i.uniques_mut().get_type_id(v);
#        i.stack_push(u);
#        Ok(())
#    });
#    i.add_builtin("unique-type-id?", |i: &mut Interpreter| {
#        let is = i.stack_top::<Unique>()?.as_ref().is_type();
#        i.stack_push(is);
#        Ok(())
#    });

#    i.add_builtin("value-meta-copy", |i: &mut Interpreter| {
#        let mut dest = i.stack_pop_val()?;
#        let src = i.stack_pop_val()?;
#        *dest.meta_mut() = src.meta_ref().clone();
#        i.stack_push(dest);
#        Ok(())
#    });

#    i.add_builtin("value-meta-entry", |i: &mut Interpreter| {
#        let u = i.stack_pop::<Unique>()?;
#        let v = i.stack_pop_val()?;
#        i.stack_push_option(v.meta_ref().get_val(u.as_ref()));
#        Ok(())
#    });

#    i.add_builtin("value-take-meta-entry", |i: &mut Interpreter| {
#        let u = i.stack_pop::<Unique>()?;
#        let mut v = i.stack_pop_val()?;
#        let entry = v.meta_mut().take_val(u.as_ref());
#        i.stack_push(v);
#        i.stack_push_option(entry);
#        Ok(())
#    });

#    i.add_builtin("current-frame-meta-entry", |i: &mut Interpreter| {
#        let u = i.stack_pop::<Unique>()?;
#        let entry = i.frame_meta_ref().get_val(u.as_ref());
#        i.stack_push_option(entry);
#        Ok(())
#    });

#    let enabled_features = List::from_iter(vec![
#        #[cfg(feature = "enable_os")] "os".to_symbol(),
#        #[cfg(feature = "enable_stdio")] "stdio".to_symbol(),
#        #[cfg(feature = "enable_fs_os")] "fs-os".to_symbol(),
#        #[cfg(feature = "enable_fs_embed")] "fs-embed".to_symbol(),
#        #[cfg(feature = "enable_fs_zip")] "fs-zip".to_symbol(),
#        #[cfg(feature = "enable_process")] "process".to_symbol(),
#        #[cfg(feature = "wasm")] "wasm".to_symbol(),
#    ]);
#    i.add_builtin("features-enabled", move |i: &mut Interpreter| {
#        i.stack_push(enabled_features.clone());
#        Ok(())
#    });

#}



