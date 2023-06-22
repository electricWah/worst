
(import ../data)
(import ../interpreter :as I)

(def <defenv> :builtin :const I/DefEnv)

(defn current-defenv :builtin [i]
  (I/stack-push i (I/current-defenv i)))

(defn defenv-new-locals :builtin [i]
  (let [d (I/stack-pop i :type I/defenv?)]
    (I/stack-push i (data/val-map d I/defenv-new-locals))))

(defn definition-add :builtin [i]
  (let [name (I/stack-pop i :type :symbol)
        def (I/stack-pop i)]
    (I/definition-add i name def)))

(defn definition-resolve :builtin [i]
  (let [name (I/stack-pop i :type :symbol)
        res (I/definition-resolve i name)]
    (I/stack-push res :nil-> false)))

