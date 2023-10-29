
open Worst_base

module DefSet = struct
    module M = Map.Make(String)
    type t' = Val.t M.t
    module T = Base.MakeValType(struct
        type t = t'
        let type_name = "defset"
    end)
    type t = t'
    include Base.ValShow.Add(struct
        include T
        let pp out _v = Format.fprintf out "<defset>"
    end)

    let empty: t = M.empty
    let add: string -> Val.t -> t -> t = M.add
    let find_opt: string -> t -> Val.t option = M.find_opt
    let merge: t -> t -> t =
        M.merge (fun _k av bv -> match bv with None -> av | bv -> bv)
end

module DefBody = struct
    type defbody = {
        body: Val.t list;
        defs: DefSet.t;
    }
    module T = Base.MakeValType(struct
        type t = defbody
        let type_name = "defbody"
    end)
    include T
    include Base.ValShow.Add(struct
        include T
        let pp out v = Format.fprintf out "[@[<hov>%a@]]" Type.List.pp v.body
    end)

    let make ~body ?(defs=DefSet.empty) () = { body; defs; }
end

type t = {
    frame: frame;
    parents: frame list;
    stack: Val.t list;
}

and frame = {
    childs: child_frame list;
    body: Val.t list;
    ambient: DefSet.t;
    local: DefSet.t;
}

and child_frame =
    | Frame of frame
    | Builtin of builtin

and builtin = t -> t

module BuiltinVal = struct
    module T = Base.MakeValType(struct
        type t = builtin
        let type_name = "builtin"
    end)
    include T
    include Base.ValShow.Add(struct
        include T
        let pp out _v = Format.fprintf out "<builtin>"
    end)
end

let frame_empty = {
    childs = [];
    body = [];
    ambient = DefSet.empty;
    local = DefSet.empty;
}
let empty = { frame = frame_empty; parents = []; stack = [] }

let define ?(ambient=false) name body i =
    let frame =
        if ambient
        then { i.frame with ambient = DefSet.add name body i.frame.ambient }
        else { i.frame with local   = DefSet.add name body i.frame.local }
    in
    { i with frame }

let make ?(body=[]) () =
    { empty with frame = { frame_empty with body } }

let update ?body i =
    let i = Option.map (fun body -> { i with frame = { i.frame with body } }) body in
    i

exception Stack_empty
exception Undefined of string
exception Root_uplevel
exception Quote_nothing

let stack_top i = List.nth_opt i.stack 0
let stack_pop_val_exn i =
    match i.stack with
    | [] -> raise Stack_empty
    | v::stack -> { i with stack }, v
let stack_push_val v i = { i with stack = v :: i.stack }

let body_next = function
    | { frame = { body = b :: body; _ }; _ } as i ->
        Some ({ i with frame = { i.frame with body } }, b)
    | _ -> None
let body_next_exn i =
    match body_next i with Some v -> v | None -> raise Quote_nothing

let eval_defbody_next i (defbody: DefBody.t) =
    let { DefBody.body; defs; } = defbody in
    let child = { frame_empty with body; ambient = defs; } in
    let frame = { i.frame with childs = Frame child :: i.frame.childs } in
    { i with frame }

let eval_next_resolve i s =
    let def =
        match DefSet.find_opt s i.frame.local with
        | Some def -> def
        | None ->
        match DefSet.find_opt s i.frame.ambient with
        | Some def -> def
        | None -> raise (Undefined s)
    in
    match BuiltinVal.of_val def with
    | Some b -> b i
    | None ->
    match DefBody.of_val def with
    | Some body -> eval_defbody_next i body
    | None -> { i with stack = def :: i.stack }

let eval_next_val v i =
    match Symbol.of_val v with
    | Some s -> eval_next_resolve i s
    | None ->
    match BuiltinVal.of_val v with
    | Some b ->
        { i with frame = { i.frame with childs = Builtin b :: i.frame.childs } }
    | None ->
    match DefBody.of_val v with
    | Some body -> eval_defbody_next i body
    | None ->
        stack_push_val v i

let into_parent_frame = function
    | { parents = f :: parents; frame; _ } as i ->
        let frame = { f with childs = Frame frame :: f.childs } in
        { i with parents; frame; }
    | { parents = []; _ } -> raise Root_uplevel

let rec run = function
    | { frame = { childs = (Builtin b) :: childs; _ }; _ } as i ->
        run (b { i with frame = { i.frame with childs } })
    | { parents; frame = { childs = (Frame frame) :: childs; _ }; _ } as i ->
        run { i with parents = { i.frame with childs } :: parents; frame; }
    | { frame = { childs = []; body = b :: body; _ }; _ } as i -> begin
        let i = { i with frame = { i.frame with body } } in
        match Symbol.of_val b with
        | Some s -> run (eval_next_resolve i s)
        | _ -> run { i with stack = b :: i.stack }
    end
    | { parents = frame :: parents; _ } as i ->
        run { i with frame; parents }
    | i -> i

