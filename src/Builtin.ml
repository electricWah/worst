
open Worst_base

let registry = ref []

let define name body = registry := (name, body) :: !registry

let define_type (module M: Base.ValType) =
    let name = Format.asprintf "%a" Base.ValShow.pp M.type_val in
    define name (Interpreter.stack_push_val M.type_val)

let install_all i =
    List.fold_left begin fun i (name, def) ->
        let def' = Interpreter.BuiltinVal.to_val def in
        Interpreter.define ~ambient:true name def' i
    end i !registry

module I = Interpreter
module V = Type
let ( let$ ) (v: I.builtin) (f: unit -> I.builtin): I.builtin = fun i -> f () (v i)
(* let ( let? ) (v: I.t -> (I.t * 'a option)) (f: 'a -> I.builtin) = fun i -> f (v i) *)

let ( let* ) (expr: I.t -> I.t * 'a) (f: 'a -> I.builtin) (i: I.t) =
    let i', v = expr i in f v i'

let ( >> ) (a: I.builtin) (b: I.builtin) i = b (a i)

let ok i = i

exception Wrong_type of string * Val.t

module StackPop = functor(T: sig
        type t
        val type_name: string
        val of_val: Val.t -> t option
    end) -> struct

    let pop (i: I.t) =
        let i', v = I.stack_pop_val_exn i in
        match T.of_val v with
        | Some v -> i', v
        | None -> raise (Wrong_type (T.type_name, v))
end

module StackPush = functor(T: sig
        type t
        val to_val: t -> Val.t
    end) -> struct

    let push (v: T.t) (i: I.t) =
        I.stack_push_val (T.to_val v) i
end

module Stack = functor(T: Base.TypeName.T) -> struct
    include StackPop(T)
    include StackPush(T)
end

module S = struct
    module Symbol = Stack(V.Symbol)
    module String = Stack(V.String)
    module Int = Stack(V.Int)
    module List = Stack(V.List)
    module Bool = Stack(V.Bool)

    module In_channel = Stack(Type.In_channel)
    module Out_channel = Stack(Type.Out_channel)
    module InputPort = StackPop(Base.ValInputPort)
    module OutputPort = StackPop(Base.ValOutputPort)

    let pop = I.stack_pop_val_exn
    let push = I.stack_push_val

end

