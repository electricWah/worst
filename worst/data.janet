
(def- T :type) # (gensym))
# meta
(def- M :meta) # (gensym))

# key for TypeId for type equality
(def- TypeId :typeid) # (gensym))

(defn construct [ty fields &opt proto-fields]
  (cond
    (table? fields)
    (let [proto (if proto-fields (put proto-fields T ty) @{T ty})]
      (table/setproto fields proto))
    (struct? fields)
    (let [proto (if proto-fields (struct T ty ;(kvs proto-fields)) {T ty})]
          (struct/with-proto proto ;(kvs fields)))
    (errorf "new: must be @{} or {}: %q" ty)))

# typeof Type = Type
(def Type
  (let [ty @{:name :type
             TypeId (gensym)
             :equal (fn [a b] (= (get a TypeId) (get b TypeId)))}]
    (table/setproto ty @{T ty})))

(defn new-type [fields]
  (-> (construct Type fields)
      (put TypeId (gensym))))

(def raw-types (tabseq [k :in [:symbol :string :boolean
                               :number :core/s64 :function]]
                       k true))
(def Symbol :symbol)
(def String :string)
(def Bool :boolean)
(def F64 :number)
(def I64 :core/s64)
(def Builtin :function)

(def Val (new-type @{:name :val}))
(defn val? [v]
  (and
    (or (table? v) (struct? v))
    (= (get (getproto v) T) Val)))

(defn typeof [v]
  (let [ty (type v)]
    (cond
      (val? v) (typeof (v :v))
      (or (table? v) (struct? v)) (get (getproto v) T)
      (raw-types v) Type
      ty)))
(assert (= Type (typeof Type)))
(assert (= Type (typeof :string)))
(assert (= :string (typeof "test")))

(defn is? [v t]
  (or (= t (typeof v))
      (and (= t Type) (raw-types v))))
(assert (is? Type Type))
(assert (is? Symbol Type))
(assert (is? "test" :string))

# wrap arrays, in reverse
(def List (new-type @{:name :list
                      :clone (fn [l] @{:l (array/slice (l :l))})}))
(defn new-list [arr &named rev]
  (default rev true)
  (cond
    (val? arr) (new-list (arr :v) :rev rev)
    (array? arr) (construct List @{:l (if rev (reverse arr) (array/slice arr))})
    (is? arr List) (construct List @{:l (array/slice (arr :l))})))

# wrap table
(def Lookup (new-type @{:name :lookup
                        :clone (fn [l] (table/clone l))}))
(defn new-lookup [tbl] (construct Lookup tbl))

(def Bytevector (new-type @{:name :bytevector
                            :clone (fn [b] @{:b (buffer/slice (b :b))})}))
# (defn new-bytevector

(defn- new-val [v meta]
  (construct Val {:v v} {M meta}))
(defn- val-metatable [v] (when (val? v) (get (getproto v) M)))

(defn- val-clone [v]
  (let [t (typeof v)
        meta (table/clone (get v M))]
    (if (is? t Type)
      (let [cl (get t :clone)
            vi (v :v)
            vv (if (nil? cl) vi (construct t (cl vi)))]
        (new-val vv meta))
      (new-val (v :v) meta))))

(defn val [v]
  (cond
    (val? v) (val-clone v)
    (raw-types (type v)) (new-val v @{})
    (array? v) (val (new-list v))
    # (buffer? v) (val (new-bytevector l))
    (nil? (typeof v)) (val (new-lookup v))
    (= (typeof (typeof v)) Type) (new-val v @{})
    (errorf "can't val this %q %q" (typeof v) v)))

(defn unwrap [v]
  (if (val? v) (v :v) v))

(assert (= 'test (unwrap (val 'test))))
(assert (is? (val 'test) :symbol))
(assert (= :string (typeof (val "test"))))

(def Unique (new-type @{:name :unique}))
(defn unique [&opt name] (construct Unique {:u (gensym) :name name}))
(assert (is? (val (unique)) Unique))

(defn type->unique [t]
  (let [raw (raw-types t)
        name (if raw t (t :name))
        t (if raw t (get (typeof t) TypeId))]
    (construct Unique {:u t :name name})))
(assert (= (type->unique Unique) (type->unique Unique)))
(assert (= (type->unique String) (type->unique String)))
(assert (= (type->unique Type) (type->unique Type)))
(assert (not (= (type->unique Unique) (type->unique List))))

(def Place (new-type @{:name :place}))
(defn place [ival] (construct Place @{:v ival}))

(defn meta-get [v key] (get (val-metatable v) key))

(defn meta-set [v kv]
  (let [v (val v)]
    (merge-into (val-metatable v) kv)
    v))

(defn set-error [v] (meta-set v {:error true}))
(def *false-error* (set-error false))
(defn nil->err [v &opt err]
  (cond
    (not (nil? v)) v
    (not (nil? err)) (set-error err)
    *false-error*))

(defn list-pop [l] (array/pop (get l :l)))
(defn list-peek [l] (array/peek (get l :l)))
(defn list-push [l v] (array/push (get l :l) v))
(defn list-append [a b] (new-list (array/concat @[] (a :l) (b :l)) :rev false))
(defn list-length [l] (int/s64 (length (l :l))))
(defn list-empty? [l] (empty? (l :l)))
(defn list->array [l] (reverse (l :l)))
(defn list-get [l i] (get (l :l) (int/to-number (- (length (l :l)) 1 i))))

(defn lookup-insert [l k v] (put l (unwrap k) v))

