
(import ../interpreter :as I)
(import ../reader :as R)
(import ../data)

(defn <reader> :builtin {:o [data/Type]} [i] [R/Reader])

(defn reader-empty :builtin {:o [R/Reader]} [i] [(R/reader)])

(defn reader-check :builtin {:i [R/Reader] :o [:any]} [i r]
  (let [res (R/check r :raise false)]
    [(if (nil? res) true (data/set-error res))]))

(defn reader-read-string :builtin
  {:i [R/Reader data/String] :o [R/Reader data/List]} [i r s]
  (let [res (data/new-list (R/read r s))]
    [r res]))

(defn read-string->list :builtin
  {:i [data/String] :o [:any]} [i s]
  (let [r (R/reader)
        res (data/new-list (R/read r s))
        check (R/check r :raise false)]
    [(if (nil? check) res (data/set-error check))]))

