
(import ./data)
(import ./interpreter :as I)
(import ./builtins :prefix "")
(import ./reader :as R)

(setdyn *pretty-format* "%m")

(def all-builtins
  (apply require-builtins
    (map (fn [s] (string "./builtins/" s))
         '[core type
           def
           fs
           value])))

(def loading-files
  ["lib/base/prelude.w"
   "lib/base/attribute.w"])

(def init-defs
  (let [loader (I/new :defs all-builtins)]
    (loop [f :in loading-files]
      (I/run loader :body (R/read-file f)))
    (I/current-defenv loader)))

(defn main [& args]
  (let [i (I/new :defs init-defs
                 :body '[define egg ["hello" println] egg])]
        # res (protect (I/run i))]
    # (pp [res (data/unwrap* (I/stack i))])))
    # (try
      (I/run i)))
      # ([err fib]
      #  (eprintf "Error: %q" err)
      #  (eprintf "Stack: %q" (data/unwrap* (I/stack i)))
      #  (eprintf "Call stack:")
      #  (loop [s :in (debug/stack fib)]
      #    (eprintf "%s@%q:%q (%s)"
      #             (s :source)
      #             (s :source-line)
      #             (s :source-column)
      #             (s :name)))
      #  (os/exit 1)))))

