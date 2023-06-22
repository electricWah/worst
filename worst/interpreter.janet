
(import ./data)

(def DefEnv (data/mkty {} :defenv))
(def defenv? (data/predicate DefEnv))
(def- mkdefenv (data/ctor DefEnv))
(def- defenv-unique (data/type->unique DefEnv))

(defn- defenv [defs]
  (if (defenv? defs) defs
    (mkdefenv :ambient (merge defs)
              :local @{})))

(defn- defenv-clone [de]
  (mkdefenv :ambient (table/clone (de :ambient))
            :local (table/clone (de :local))))

(defn- defenv-insert [defs ty name val]
  (put-in defs [ty name] val))

(defn- defenv-resolve [defs name]
  (or ((defs :local) name) ((defs :ambient) name)))

(defn defenv-new-locals [d]
  (data/the defenv? d)
  (mkdefenv :ambient (merge-into (d :ambient) (d :local))
            :local @{}))

(def- Frame (data/mkty @{} :frame))
(def- frame? (data/predicate Frame))

(defn- frame-empty? [f]
  (and (empty? (f :childs))
       (data/list-empty? (f :body))))

(defn- new-frame [&named defs body]
  (default defs {})
  (default body [])
  # TODO if val, get meta
  (table/setproto
    @{:defs (defenv defs)
      :childs @[]
      :body (data/list body)}
    Frame))

(defn- frame-resolve [f name]
  (if-let [def (defenv-resolve (f :defs) name)]
    def
    (errorf "undefined: %q" name)))

(def Interpreter (data/mkty @{} :interpreter))
(def interpreter? (data/predicate Interpreter))
(def- mkinterpreter (data/ctor Interpreter))

(defn new [&named defs body stack]
  (default defs {})
  (default body [])
  (default stack [])
  (mkinterpreter
    :frame (new-frame :defs defs :body body)
    :parents @[]
    :stack (array/slice stack)))

(defn eval-next [i v]
  (let [frame (i :frame)
        defs (data/unwrap (data/meta-get v defenv-unique))
        iv (data/unwrap v)]
    (cond
      (symbol? iv) (eval-next i (frame-resolve frame iv))
      (function? iv) (array/push (frame :childs) v)
      (or (indexed? iv)
          (data/list? iv)) (array/push (frame :childs)
                                       (new-frame :defs defs
                                                  :body v))
      (errorf "unknown %q" iv))))

(defn enter-parent-frame [i]
  (data/the interpreter? i)
  (if (not (empty? (i :parents)))
    (let [p (array/pop (i :parents))
          f (i :frame)]
      (unless (frame-empty? f)
        (array/push (p :childs) f))
      (put i :frame p))
    (error "root-uplevel")))

(defn stack-pop [i &named type]
  (let [v (array/pop (i :stack))]
    (cond
      (nil? v) (error "stack-empty")
      type (data/the (data/valof type) v)
      v)))

(defn stack-push [i v &named nil->]
  (array/push (i :stack)
              (cond
                (data/val? v) v
                (nil? v) (if (nil? nil->)
                           (error "stack-push nil")
                           (data/val (nil-> v)))
                (data/val v))))
(defn stack [i] (data/list (i :stack)))
(defn stack-set [i s] (put i :stack (data/list->array s)))

(defn code-next [i] (data/list-pop ((i :frame) :body)))
(defn code-peek [i] (data/list-peek ((i :frame) :body)))

(defn current-defenv [i] (defenv-clone ((i :frame) :defs)))

(defn definition-add [i name d]
  (defenv-insert ((i :frame) :defs) :local
    (data/the :symbol (data/unwrap name)) d))

(defn definition-resolve [i name]
  (defenv-resolve ((i :frame) :defs)
    (data/the :symbol (data/unwrap name))))

(defn run [i &named body]
  (when body
    (put (i :frame) :body (data/list body)))
  (var ret nil)
  (while (nil? ret)
    (let [frame (i :frame)]
      (cond
        # try into child
        (not (empty? (frame :childs)))
        (let [c (array/pop (frame :childs))
              ci (data/unwrap c)]
          (cond
            (frame? c)
            (do
              (array/push (i :parents) frame)
              (put i :frame c))
            (function? ci)
            (let [r (ci i)]
              (when (data/meta-get c :pause)
                (set ret r)))
            (errorf "unknown child %q" c)))

        # try next body
        (not (data/list-empty? (frame :body)))
        (let [v (data/list-pop (frame :body))
              iv (data/unwrap v)]
          (if (symbol? iv)
            (eval-next i (frame-resolve frame iv))
            (stack-push i v)))

        # try into parent
        (not (empty? (i :parents)))
        (put i :frame (array/pop (i :parents)))

        (break))))
  ret)


