
(import ../interpreter :as I)
(import ../data)

(defn type->unique :builtin [i]
  (let [t (data/unwrap (I/stack-pop i :type data/type?))]
    (I/stack-push i (data/type->unique t))))

