
(import ../interpreter :as I)
(import ../data)

(defn string-append :builtin
  {:i [data/String data/String] :o [data/String]} [i a b] [(string/join [a b])])

(defn string-equal :builtin
  {:i [data/String data/String] :o [data/Bool]} [i a b] [(= a b)])
(defn string-compare :builtin
  {:i [data/String data/String] :o [data/I64]} [i a b] [(int/s64 (compare a b))])

(defn string-split-bytes->list :builtin
  {:i [data/String] :o [data/List]} [i s]
  [(data/new-list (seq [c :in s] (string/from-bytes c)))])

(defn string-split :builtin
  {:i [data/String data/String] :o [data/List]} [i s p]
  [(data/new-list (string/split p s))])

