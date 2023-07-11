
(import ../interpreter :as I)
(import ../data)

(defn <interpreter> :builtin {:o [data/Type]} [i] [I/Interpreter])

(defn interpreter-empty :builtin {:o [I/Interpreter]} [i] [(I/new)])

(defn interpreter-run
  :builtin {:i [I/Interpreter] :o [I/Interpreter :any]} [i ii]
  (let [res (I/run ii)
        rr (if (nil? res) true res)]
    [ii rr]))

(defn interpreter-complete?
  :builtin {:i [I/Interpreter] :o [I/Interpreter data/Bool]} [i ii]
  [ii (I/interpreter-complete? ii)])

(defn interpreter-reset 
  :builtin {:i [I/Interpreter] :o [I/Interpreter]} [i ii]
  [(I/interpreter-reset ii)])

(defn interpreter-stack-get
  :builtin {:i [I/Interpreter] :o [I/Interpreter data/List]} [i ii]
  [ii (I/stack ii)])

(defn interpreter-defenv-set
  :builtin {:i [I/Interpreter I/DefEnv] :o [I/Interpreter]} [i ii d]
  (I/set-defenv ii d)
  [ii])

(defn interpreter-body-prepend
  :builtin {:i [I/Interpreter data/List] :o [I/Interpreter]} [i ii body]
  (I/body-prepend ii body)
  [ii])

