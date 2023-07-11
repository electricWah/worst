
(import ./data)
(import ./interpreter :as I)
(import ./builtins :prefix "")
(import ./reader :as R)

(defn runner [i & args]
  (try
    (I/run i ;args)
    ([err fib]
     (eprintf "Error: %q" err)
     (eprintf "Stack: %q" (I/stack i))
     # (eprintf "?? %q" (I/definition-resolve i 'define))
     (eprintf "Call stack:")
     (loop [s :in (debug/stack fib)]
       (eprintf "%s@%q:%q (%s)"
                (s :source)
                (s :source-line)
                (s :source-column)
                (or (s :name) "???")))
     (os/exit 1))))

(def all-builtins
  (apply require-builtins
    (map (fn [s] (string "./builtins/" s))
         '[core type def
           string list lookup numeric bytevector
           place port fs sys
           interpreter reader])))

(def preloads '[init attribute dispatch ops
                ansi repl])

(def init-defs
  (let [loader (I/new :defs all-builtins)]
    (loop [preload :in preloads
           :let [f (string "./worst/preload/" preload ".w")]]
      (runner loader :body (R/read-file f)))
    (I/current-defenv loader)))

(def progmain (data/val (R/read-file "./worst/main.w")))

(defn main [& args]
  (let [i (I/new :defs init-defs
                 :body progmain)]
        # res (protect (I/run i))]
    # (pp [res (data/unwrap* (I/stack i))])))
      (runner i)))

