
(import ../interpreter :as I)
(import ../data)

(defn type-equal :builtin {:i [data/Type data/Type] :o [data/Bool]} [i a b]
  [(compare= a b)])
(defn type-hash :builtin {:i [data/Type] :o [data/I64]} [i t] [(int/s64 (hash t))])

(defn type->unique :builtin {:i [data/Type] :o [data/Unique]} [i t]
  [(data/type->unique t)])

(defn value-type :builtin {:i [:any] :o 1} [i v] [(data/typeof v)])

#    i.add_builtin("unique-type-id?", |i: &mut Interpreter| {
#        let is = i.stack_top::<Unique>()?.as_ref().is_type();
#        i.stack_push(is);
#        Ok(())
#    });

