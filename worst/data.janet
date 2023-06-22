
(def- type-unique (gensym))

(defn mkty [tabstr name]
  (let [uid (gensym)
        mk (if (table? tabstr) table struct)]
    (mk :typeid uid
        :type-unique type-unique
        :name name)))

(defn type? [t]
  (cond
    (table? t) (= (table/rawget t :type-unique) type-unique)
    (struct? t) (and (nil? (struct/getproto t)) (= (t :type-unique) type-unique))
    false))

(defn the [ty expr]
  (let [ok (if (keyword? ty) (= ty (type expr)) (ty expr))]
    (if ok expr (errorf "wrong type, expected %q: %q" ty expr))))

(defn predicate [t]
  (cond
    (table? t) (fn [v] (and (table? v) (= (table/getproto v) t)))
    (struct? t) (fn [v] (and (struct? v) (= (struct/getproto v) t)))))

(defn ctor [t]
  (cond
    (table? t) (fn [& kv]
                 (let [v (table ;kv)]
                   (table/setproto v t)
                   v))
    (struct? t) (fn [& kv]
                  (struct/with-proto t ;kv))))

(def Unique (mkty {} :unique))
(def unique? (predicate Unique))
(def- mkunique (ctor Unique))

(defn type->unique [t]
  (mkunique :u (t :typeid)
            :name (t :name)))

(defn unique [& name]
  (mkunique :u (gensym)
            :name name))

(def Val (mkty {} :val))
(def val? (predicate Val))
(def- mkval (ctor Val))

(defn unwrap [v] (if (val? v) (v :v) v))

(defn valof [ty] (fn [v] (the ty ((the val? v) :v)) v))
(defn thevalof [ty expr] (the (valof ty) expr))
(defn nonnil [v] (not (nil? v)))

(defn val-map [v f]
  (mkval :v (f (v :v))
         :meta (v :meta)))

(defn meta-get [v key] ((v :meta) key))
(defn meta-set [v & kv]
  (let [unwrap-ks (seq [[i kv] :pairs kv]
                    (if (even? i) (unwrap kv) kv))]
    (mkval :v (unwrap v)
           :meta (table/to-struct (merge (or (v :meta) {})
                                         (apply struct unwrap-ks))))))

(def List (mkty {} :list))
(def list? (predicate List))
(def- mklist (ctor List))

(defn list [l]
  (let [l (unwrap l)
        contents (if (list? l)
                   (array/slice (l :l))
                   (reverse l))]
    (mklist :l contents)))

(defn val [v]
  (cond
    (val? v) v
    (indexed? v) (mkval :v (list v))
    (nil? v) (error "nil")
    (mkval :v v)))

(defn list-pop [l] (val (array/pop (l :l))))
(defn list-peek [l] (val (array/peek (l :l))))
(defn list-push [l v] (array/push (l :l) (val v)))
(defn list-length [l] (length (l :l)))
(defn list-empty? [l] (empty? (l :l)))
(defn list->array [l] (reverse (l :l)))

(defn unwrap* [v]
  (let [v (unwrap v)]
    (if (list? v)
      (map unwrap* (list->array v))
      v)))

