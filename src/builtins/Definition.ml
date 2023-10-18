
Builtin.define_type (module Interpreter.DefSet.T);;

module DefSetS = Builtin.Stack(Interpreter.DefSet.T);;
module DefBodyS = Builtin.Stack(Interpreter.DefBody);;

Builtin.(define "current-ambient-defset" @@ begin fun i ->
    DefSetS.push i.frame.ambient i
end);;

Builtin.(define "current-local-defset" @@ begin fun i ->
    DefSetS.push i.frame.local i
end);;

Builtin.(define "defset-merge" @@ begin
    let* b = DefSetS.pop in
    let* a = DefSetS.pop in
    DefSetS.push (I.DefSet.merge a b)
end);;

Builtin.(define "make-defbody" @@ begin
    let* defs = DefSetS.pop in
    let* body = S.List.pop in
    DefBodyS.push (I.DefBody.make ~body ~defs ())
end);;

Builtin.(define "definition-add" @@ begin
    let* name = S.Symbol.pop in
    let* def = S.pop in
    I.define name def
end);;



