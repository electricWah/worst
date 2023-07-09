
(import ../interpreter :as I)
(import ../data)

(defn <string> :builtin {:o [data/Type]} [i] [data/String])
(defn <symbol> :builtin {:o [data/Type]} [i] [data/Symbol])
(defn <bool> :builtin {:o [data/Type]} [i] [data/Bool])
(defn <builtin> :builtin {:o [data/Type]} [i] [data/Builtin])

(defn <list> :builtin {:o [data/Type]} [i] [data/List])
(defn <lookup> :builtin {:o [data/Type]} [i] [data/Lookup])
(defn <bytevector> :builtin {:o [data/Type]} [i] [data/Bytevector])

(defn <type> :builtin {:o [data/Type]} [i] [data/Type])
(defn <unique> :builtin {:o [data/Type]} [i] [data/Unique])

(defn make-unique :builtin [i] (I/stack-push i (data/unique)))

(defn dump :builtin [i] (pp (I/stack i)))
(defn dump1 :builtin [i] (pp (I/stack-pop i)))

(defn drop :builtin [i] (I/stack-pop i))

(defn clone :builtin {:i 1 :o 2} [i v] [v v])
(defn swap :builtin {:i 2 :o 2} [i a b] [b a])
(defn dig :builtin {:i 3 :o 3} [i a b c] [b c a])
(defn bury :builtin {:i 3 :o 3} [i a b c] [c a b])

(defn bool-equal :builtin {:i [data/Bool data/Bool] :o [data/Bool]} [i a b]
  [(= a b)])

(def- real-not not)
(defn not :builtin {:i 1 :o 1} [i v] [(real-not (data/unwrap v))])

(defn eval :builtin {:i 1} [i v] (I/eval-next i v))
(defn eval-if :builtin {:i 2} [i b a]
  (when (data/unwrap b)
    (I/eval-next i a)))
(defn uplevel :builtin {:i 1} [i v]
  (I/enter-parent-frame i)
  (I/eval-next i v))

(defn pause :builtin :pause {:i 1} [i a] a)

(defn stack-get :builtin {:o [data/List]} [i] [(I/stack i)])
(defn stack-set :builtin {:i [data/List]} [i s] (I/stack-set i s))

(defn code-next :builtin {:o 1} [i] [(I/code-next i)])
(defn code-peek :builtin {:o 1} [i] [(I/code-peek i)])
(defn quote :builtin {:o 1} [i] [(I/code-next i)])
(defn upquote :builtin {:o 1} [i]
  (I/enter-parent-frame i)
  [(I/code-next i)])

(defn value->constant :builtin {:i 1 :o 1} [i a]
  [(fn const [i] (I/stack-push i a))])

(defn value-meta-entry :builtin
  {:i [:val data/Unique] :o 1} [i v u]
  [(or (data/meta-get v u) false)])

(defn value-insert-meta-entry :builtin
  {:i [:val data/Unique :val] :o 1} [i v u mv]
  [(data/meta-set v {u mv})])

(defn features-enabled :builtin {:o [data/List]} [i]
  # 'os 'stdio 'fs-os 'fs-embed 'fs-zip 'process 'wasm
  [(data/val @[])])

#    util::add_const_type_builtin::<IsError>(i, "<is-error>");
#    i.add_builtin("bool-equal", util::equality::<bool>);
#    i.add_builtin("bool-hash", util::value_hash::<bool>);
#    i.add_builtin("symbol-equal", util::equality::<Symbol>);
#    i.add_builtin("symbol-hash", util::value_hash::<Symbol>);

#    i.add_builtin("unique-equal", util::equality::<Unique>);
#    i.add_builtin("unique-hash", util::value_hash::<Unique>);
#    i.add_builtin("make-unique", |i: &mut Interpreter| {
#        let u = i.uniques_mut().create();
#        i.stack_push(u);
#        Ok(())
#    });

#    util::add_const_type_builtin::<Builtin>(i, "<builtin>");

#    i.add_builtin("value-meta-copy", |i: &mut Interpreter| {
#        let mut dest = i.stack_pop_val()?;
#        let src = i.stack_pop_val()?;
#        *dest.meta_mut() = src.meta_ref().clone();
#        i.stack_push(dest);
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



