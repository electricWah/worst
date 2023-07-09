
(import ../interpreter :as I)
(import ../data)

(defn command-line-arguments :builtin {:o [data/List]} [i] [(data/val (dyn *args*))])

