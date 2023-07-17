
(import ./interpreter :as I)
(import ./data)

(defn- stack-pred [p]
  (match p
    :any (fn [v] (data/unwrap v))
    :val (fn [v] v)
    (t symbol?) (let [t (eval t)] (fn [v] (let [u (data/unwrap v)]
                                            (when (data/is? u t) u))))
    _ (errorf "can't stack-pred %q" p)))

(defn- stack-preds [v]
  (cond
    (number? v) (array/new-filled v (fn [v] v))
    (indexed? v) (map stack-pred v)))

(defn pred-wrapped [f ins]
  (let [preds (stack-preds ins)]
    (fn [i]
      (if-let [s (I/stack-popn i (length preds))]
        (let [okvs (map (fn [pred v] (pred v)) preds s)]
          (if (nil? (find-index nil? okvs))
            (f i ;okvs)
            (errorf "%q: wrong types: expected %q, got %q" f ins s)))
        (errorf "stack empty, expected %q" ins)))))

(defn out-wrapped [f outs]
  (let [preds (stack-preds outs)]
    (fn [i]
      (let [os (f i)
            okvs (map (fn <out> [pred v] (pred v)) preds os)]
        (if (nil? (find-index nil? okvs))
          (loop [v :in os]
            (I/stack-push i v))
          (errorf "%q: wrong return types: expected %q, got %q %q" f outs os okvs))))))

(defn- stackfn [f i o]
  (let [fins (if (nil? i) f (pred-wrapped f i))
        fouts (if (nil? o) fins (out-wrapped fins o))]
    fouts))

(defn- fn-entry->builtin [f]
  (data/meta-set
    (data/val (stackfn (f :value) (f :i) (f :o)))
    {:name (f :name)
     :builtin true
     :pause (f :pause)}))

(defn- require-builtins-1 [path]
  (->> (require path)
       pairs
       (keep (fn [[name v]] (and (table? v)
                                 (v :builtin)
                                 [(or (v :name) name)
                                  (fn-entry->builtin v)])))
       from-pairs))

(defn require-builtins [& paths]
  (apply merge (array/concat (map require-builtins-1 paths))))



