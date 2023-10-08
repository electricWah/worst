
open Ty

module DefSet = struct
    module M = Map.Make(String)
    type t' = Val.t M.t
    module T = MakeValType(struct
        type t = t'
        let type_name = "defset"
    end)
    type t = t'
    include ValShow.Add(struct
        include T
        let pp out _v = Format.fprintf out "<defset>"
    end)

    let empty: t = M.empty
    let add: string -> Val.t -> t -> t = M.add
    let find_opt: string -> t -> Val.t option = M.find_opt
    let merge: t -> t -> t =
        M.merge (fun _k av bv -> match bv with None -> av | bv -> bv)
end

type t = {
    frame: frame;
    parents: frame list;
    stack: Val.t list;
}

and frame = {
    childs: child_frame list;
    body: Val.t list;
    meta: Meta.t;
    ambient: DefSet.t;
    locals: DefSet.t;
}

and child_frame =
    | Frame of frame
    | Builtin of builtin

and builtin = t -> t

module BuiltinVal = struct
    module T = MakeValType(struct
        type t = builtin
        let type_name = "builtin"
    end)
    include T
    include ValShow.Add(struct
        include T
        let pp out _v = Format.fprintf out "<builtin>"
    end)
end

let frame_empty = {
    childs = [];
    body = [];
    meta = Meta.empty;
    ambient = DefSet.empty;
    locals = DefSet.empty;
}
let empty = { frame = frame_empty; parents = []; stack = [] }

let define name body i =
    { i with frame = { i.frame with locals = DefSet.add name body i.frame.locals } }

let make ?body () =
    let body = Option.value body ~default:[] in
    { empty with frame = { frame_empty with body } }

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

let eval_list_next ?(inherit_defs=true) i body meta =
    let ambient =
        match DefSet.T.type_meta_entry meta with
        | Some ds -> ds
        | None ->
            if inherit_defs
            then DefSet.merge i.frame.ambient i.frame.locals
            else DefSet.empty
    in
    let child = { frame_empty with body; meta; ambient; } in
    let frame = { i.frame with childs = Frame child :: i.frame.childs } in
    { i with frame }

let eval_next_resolve i s =
    match DefSet.find_opt s i.frame.locals with
    | None -> raise (Undefined s)
    | Some def ->
    match BuiltinVal.of_val def with
    | Some b -> b i
    | None ->
    match V.List.of_val def with
    | Some body ->
        { i with
            parents = i.frame :: i.parents;
            frame = { frame_empty with body } }
    | None -> { i with stack = def :: i.stack }

let eval_next_val v i =
    match V.Symbol.of_val v with
    | Some s -> eval_next_resolve i s
    | None ->
    match BuiltinVal.of_val v with
    | Some b ->
        { i with frame = { i.frame with childs = Builtin b :: i.frame.childs } }
    | None ->
    match V.List.of_val v with
    | Some l -> eval_list_next ~inherit_defs:false i l v.meta
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
        match V.Symbol.of_val b with
        | Some s -> run (eval_next_resolve i s)
        | _ -> run { i with stack = b :: i.stack }
    end
    | { parents = frame :: parents; _ } as i ->
        run { i with frame; parents }
    | i -> i

