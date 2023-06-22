
(defmacro defbuiltin [name fnarg f]
  ~(upscope
     (defn ,name ,fnarg ,f)
     (setdyn ',name @{:builtin true})))

