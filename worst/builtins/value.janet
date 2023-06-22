
(import ../data)
(import ../interpreter :as I)

(defn value->constant :builtin [i]
  (let [a (I/stack-pop i)]
    (I/stack-push i (fn const [i] (I/stack-push i a)))))

(defn value-insert-meta-entry :builtin [i]
  (let [mv (I/stack-pop i)
        u (I/stack-pop i :type data/unique?)
        v (I/stack-pop i)]
    (I/stack-push i (data/meta-set v u mv))))

