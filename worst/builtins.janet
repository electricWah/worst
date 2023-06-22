
(import ./interpreter :as I)
(import ./data)

(defn- fn-entry->builtin [f]
  (let [inner (cond
                (f :const) (fn constant [i] (I/stack-push i (f :value)))
                (f :value))]
    (data/meta-set
      (data/val inner)
      :name (f :name)
      :builtin true
      :pause (f :pause))))

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


