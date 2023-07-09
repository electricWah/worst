
(import ../interpreter :as I)
(import ../data)

(defn <defenv> :builtin {:o [data/Type]} [i] [I/DefEnv])

(defn defenv-empty :builtin {:o [I/DefEnv]} [i] [(I/defenv {})])

(defn current-defenv :builtin [i]
  (I/stack-push i (I/current-defenv i)))

(defn defenv-new-locals :builtin {:i [I/DefEnv] :o [I/DefEnv]} [i d]
  [(I/defenv-new-locals d)])

(defn defenv-insert-local :builtin {:i [I/DefEnv :symbol :val] :o [I/DefEnv]} [i d name def]
  [(I/defenv-insert d :local name def)])


# (defn defenv-insert-local :builtin [i]
#   (let [d (I/stack-pop i)
#         name (data/unwrap (I/stack-pop i data/Type :symbol))
#         defs (data/unwrap (I/stack-pop i data/Type I/defenv?))]
#     # (
    #     defs.as_mut().insert_local(name.to_string(), def);
    #     i.stack_push(defs);
    #     Ok(())
    # (I/stack-push i (data/val-map d I/defenv-new-locals))))

(defn definition-add :builtin {:i [:val data/Symbol]} [i def name]
  (I/definition-add i name def))

(defn definition-resolve :builtin {:i [data/Symbol] :o [:any]} [i name]
  [(data/nil->err (I/definition-resolve i name))])

