
(import ../interpreter :as I)
(import ../data)

# (defn <string> :builtin {:o [data/Type]} [i] [data/String])

(defn stdout-port :builtin {:o [data/Port]} [i] [(data/new-port stdout)])

(defn stdout-port-write-string :builtin
  {:i [data/Port data/String] :o [data/Port]} [i p s]
  (file/write (p :port) s)
  [p])

(defn stdout-port-flush :builtin
  {:i [data/Port] :o [data/Port :any]} [i p]
  (file/flush (p :port))
  [p true])

(defn stdin-port-read-line :builtin {:o [data/String]} [i]
  [(string (file/read stdin :line))])

(defn port-read->string :builtin {:i [data/Port] :o [data/String]} [i p]
  [(string (file/read (p :port) :all))])

