
(import ./data)

(def DefEnv (data/new-type
                      @{:name :defenv
                        :clone (fn [d] @{:ambient (table/clone (d :ambient))
                                         :local (table/clone (d :local))})}))
(def- defenv-unique (data/type->unique DefEnv))
(assert (= defenv-unique (data/type->unique DefEnv)))

(defn defenv [defs]
  (if (data/is? defs DefEnv) defs
    (data/construct DefEnv @{:ambient (merge defs)
                             :local @{}})))
(assert (= DefEnv (data/typeof (defenv {}))))

(defn defenv-insert [defs ty name val]
  (put-in defs [ty name] val))

(defn- defenv-resolve [defs name]
  (or ((defs :local) name) ((defs :ambient) name)))

(defn defenv-new-locals [d]
  (defenv (merge (d :ambient) (d :local))))

(defn defenv-extend-locals [d locs]
  (data/construct DefEnv @{:ambient (merge (d :ambient))
                           :local (merge (d :local) (locs :local))}))

(def- Frame (data/new-type @{:name :frame}))

(defn- frame-empty? [f]
  (and (empty? (f :childs))
       (data/list-empty? (f :body))))

(defn- new-frame [&named defs body]
  (default defs {})
  (default body @[])
  # TODO if val, get meta
  (let [meta (if (data/val? body) (data/val-metatable body) @{})]
    (data/construct Frame @{:defs (defenv defs)
                            :childs @[]
                            :meta meta
                            :body (data/new-list body)})))

(defn- frame-resolve [f name]
  (if-let [def (defenv-resolve (f :defs) name)]
    def
    (do
      # (pp (keys ((f :defs) :local)))
      (errorf "undefined: %q" name))))

(def Interpreter (data/new-type @{:name :interpreter}))

(defn new [&named defs body stack]
  (default defs {})
  (default body @[])
  (default stack @[])
  (data/construct Interpreter
    @{:frame (new-frame :defs defs :body body)
      :parents @[]
      :stack (array/slice stack)}))

(defn interpreter-complete? [i]
  (and (empty? (i :parents))
       (frame-empty? (i :frame))))

(defn stack-push [i v] (array/push (i :stack) (data/val v)))
(defn stack [i] (data/val (i :stack)))
(defn stack-popn [i n]
  (if (> n (length (i :stack))) nil
    (reverse (seq [_ :range [0 n]] (array/pop (i :stack))))))
(defn stack-set [i s] (put i :stack (data/list->array s)))

(defn eval-next [i v &named inherit]
  (let [frame (i :frame)
        defs (defenv-new-locals
               (or (data/unwrap (data/meta-get v defenv-unique))
                   (frame :defs)))
        iv (data/unwrap v)]
    (cond
      (symbol? iv) (eval-next i (frame-resolve frame iv))
      (function? iv) (array/push (frame :childs) v)
      (data/is? iv data/List) (array/push (frame :childs)
                                          (new-frame :defs defs
                                                     :body v))
      (stack-push i v))))

(defn enter-parent-frame [i]
  (if (not (empty? (i :parents)))
    (let [p (array/pop (i :parents))
          f (i :frame)]
      (unless (frame-empty? f)
        (array/push (p :childs) f))
      (put i :frame p))
    (error "root-uplevel")))

(defn interpreter-reset [i]
  (while (not (empty? (i :parents)))
    (enter-parent-frame i))
  (put (i :frame) :childs @[])
  (put (i :frame) :body (data/new-list @[]))
  i)

(defn stack-pop [i]
  (let [v (array/pop (i :stack))]
    (if (nil? v) (error "stack-empty")
      v)))

(defn code-next [i] (data/list-pop! ((i :frame) :body)))
(defn code-peek [i] (data/list-peek! ((i :frame) :body)))

(defn current-defenv [i] ((i :frame) :defs))
(defn set-defenv [i d] (put (i :frame) :defs (defenv d)))

(defn definition-add [i name d]
  (defenv-insert ((i :frame) :defs) :local name d))

(defn definition-resolve [i name]
  (defenv-resolve ((i :frame) :defs) name))

(defn body-prepend [i body]
  (data/list-prepend! ((i :frame) :body) (data/new-list body)))

(defn current-frame-meta-entry [i k]
  (get ((i :frame) :meta) (data/unwrap k)))

(defn run [i &named body]
  (when body
    (put (i :frame) :body (data/new-list body)))
  (var ret nil)
  (while (nil? ret)
    (let [frame (i :frame)]
      (cond
        # try into child
        (not (empty? (frame :childs)))
        (let [c (array/pop (frame :childs))
              ci (data/unwrap c)]
          (cond
            (data/is? ci Frame)
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
        (let [v (data/list-pop! (frame :body))
              iv (data/unwrap v)]
          (if (symbol? iv)
            (eval-next i (frame-resolve frame iv))
            (stack-push i v)))

        # try into parent
        (not (empty? (i :parents)))
        (put i :frame (array/pop (i :parents)))

        (break))))
  ret)


