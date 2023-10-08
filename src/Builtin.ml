
let registry = ref []

let define name body = registry := (name, body) :: !registry

let install_all i =
    List.fold_left (fun i (name, def) -> Interpreter.define name (Interpreter.BuiltinVal.to_val def) i) i !registry

module I = Interpreter
module V = Ty.V
let ( let$ ) (v: I.builtin) (f: unit -> I.builtin): I.builtin = fun i -> f () (v i)
(* let ( let? ) (v: I.t -> (I.t * 'a option)) (f: 'a -> I.builtin) = fun i -> f (v i) *)

let ( let* ) (expr: I.t -> I.t * 'a) (f: 'a -> I.builtin) (i: I.t) =
    let i', v = expr i in f v i'

let ( >> ) (a: I.builtin) (b: I.builtin) i = b (a i)

let ok i = i

(* let (let*?) (i: I.t) (f: I.t -> 'a option) -> 'a *) 

exception Wrong_type of string * Ty.Val.t

module Stack = functor(T: sig
        include Ty.TypeName.T
    end) -> struct

    let pop (i: I.t) =
        let i', v = I.stack_pop_val_exn i in
        match T.of_valobj v.v with
        | Some v -> i', v
        | None -> raise (Wrong_type (T.type_name, v))

    let push (v: T.t) (i: I.t) =
        I.stack_push_val (Ty.Val.of_valobj (T.to_valobj v)) i

end

module S = struct
    module Symbol = Stack(V.Symbol)
    module String = Stack(V.String)
    module List = Stack(V.List)
    module Bool = Stack(V.Bool)

    let pop = I.stack_pop_val_exn
    let push = I.stack_push_val

end

