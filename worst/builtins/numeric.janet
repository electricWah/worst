
(import ../interpreter :as I)
(import ../data)

(defn <i64> :builtin {:o [data/Type]} [i] [data/I64])
(defn <f64> :builtin {:o [data/Type]} [i] [data/F64])

(defn i64->string :builtin {:i [data/I64] :o [data/String]} [i v] [(string v)])

(defn i64-add :builtin {:i [data/I64 data/I64] :o [data/I64]} [i a b] [(int/s64 (+ a b))])
(defn i64-div :builtin {:i [data/I64 data/I64] :o [data/I64]} [i a b] [(int/s64 (/ a b))])

(defn i64-compare :builtin {:i [data/I64 data/I64] :o [data/I64]} [i a b]
  [(int/s64 (compare a b))])
(defn i64-equal :builtin {:i [data/I64 data/I64] :o [data/Bool]} [i a b]
  [(= a b)])

