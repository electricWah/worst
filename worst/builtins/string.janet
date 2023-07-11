
(import ../interpreter :as I)
(import ../data)

(defn string-append :builtin
  {:i [data/String data/String] :o [data/String]} [i a b] [(string/join [a b])])

(defn string-equal :builtin
  {:i [data/String data/String] :o [data/Bool]} [i a b] [(= a b)])
