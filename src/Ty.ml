
let gen_type_id =
    let id = ref 1 in
    fun () -> begin
        let id' = !id in
        id := id' + 1;
        id'
    end

module ValObj = struct
    type t = {
        type_id: int;
        v: Obj.t;
    }
end

module type ValObjType = sig
    type t
    val type_id: int
    val to_valobj: t -> ValObj.t
    val of_valobj: ValObj.t -> t option
end

module MakeValObjType = functor(T: sig type t end) -> struct
    type t = T.t
    let type_id = gen_type_id()
    let to_valobj (v: t) = { ValObj.type_id; v = Obj.repr v }
    let of_valobj (v: ValObj.t) : T.t option =
        if v.type_id = type_id then Some (Obj.obj v.v) else None
end

module MakeTypeData = functor(T: sig type tdata end) -> struct
    module Lookup = Hashtbl.Make(struct include Int let hash = Hashtbl.hash end)
    let class_data: T.tdata Lookup.t = Lookup.create 30

    let find_type (module M: ValObjType) = Lookup.find_opt class_data M.type_id
    let find_valobj (v: ValObj.t) = Lookup.find_opt class_data v.type_id

    let install (module M: ValObjType) (data: T.tdata) =
        Lookup.add class_data M.type_id data

end

module ValCompare = struct
    module type T = sig
        include ValObjType
        val compare: t -> t -> int
    end
    module TC = MakeTypeData(struct type tdata = (module T) end)
    module Add = functor(M: T) -> struct
        let () = TC.install (module M) (module M)
    end

    let compare_valobj (a: ValObj.t) (b: ValObj.t) =
        match Stdlib.compare a.type_id b.type_id with
        | 0 -> begin
            match TC.find_valobj a with
            | Some td -> begin
                let module M = (val td: T) in
                let a' = Option.get (M.of_valobj a) in
                let b' = Option.get (M.of_valobj b) in
                M.compare a' b'
            end
            | None -> compare a.v b.v
        end
        | x -> x

end

module MetaMap = Map.Make(struct
    include ValObj
    let compare = ValCompare.compare_valobj
end)

module Val = struct
    type t = {
        v: ValObj.t;
        meta: t MetaMap.t;
    }

    let of_valobj v = { v; meta = MetaMap.empty; }
end

module Meta = struct
    type t = Val.t MetaMap.t
    let empty: t = MetaMap.empty
    let get_valobj (k: ValObj.t) (v: t): Val.t option = MetaMap.find_opt k v

    let update_val (k: ValObj.t) (v: Val.t) (value: Val.t) =
        { value with meta = MetaMap.add k v value.meta }
end

module MakeType = functor(T: sig type t end) -> struct
    include MakeValObjType(T)
    let to_val (v: t) = Val.of_valobj (to_valobj v)
    let of_val (v: Val.t) = of_valobj v.v
end

module TypeName = struct
    module TC = MakeTypeData(struct type tdata = string end)
    include TC

    module type T = sig
        include ValObjType
        val type_name: string
    end

    module Add = functor(M: T) -> struct
        include M
        let () = TC.install (module M) M.type_name
    end

end

module ValShow = struct

    module type T = sig
        include ValObjType
        val pp: Format.formatter -> t -> unit
    end

    module TC = MakeTypeData(struct type tdata = (module T) end)

    module Add = functor(M: T) -> struct
        let () = TC.install (module M) (module M)
        let pp = M.pp
    end

    let pp out (v: Val.t) =
        match TC.find_valobj v.v with
        | None -> Format.fprintf out "<value>"
        | Some td ->
            let module M = (val td: T) in
            let v' = Option.get (M.of_valobj v.v) in
            M.pp out v'

end

module ValRead = struct

    module type T = sig
        include ValObjType
        val read_into: ?from:int -> ?len:int -> bytes -> t -> int
    end

    module type Tv = sig
        val read_into: ?from:int -> ?len:int -> bytes -> int
    end

    module TC = MakeTypeData(struct type tdata = (module T) end)

    module Add = functor(M: T) -> struct
        let () = TC.install (module M) (module M)
    end

    let get_val (v: Val.t) =
        match TC.find_valobj v.v with
        | None -> None
        | Some td ->
            let module M = (val td: T) in
            let v' = Option.get (M.of_valobj v.v) in
            let module V = struct
                let read_into ?from ?len dest = M.read_into ?from ?len dest v'
            end in
            Some (module V: Tv)
end

module VType = struct
    module T = MakeType(struct type t = string end)
    include T
    include TypeName.Add(struct include T let type_name = "type" end)
    include ValShow.Add(struct
        include T
        let pp out v = Format.fprintf out "<%s>" v
    end)
end

module type ValType = sig
    val type_valobj: ValObj.t
end

module MakeValType = functor (VT: sig
        type t
        val type_name: string
    end) -> struct

    module T' = MakeType(VT)
    include T'
    include TypeName.Add(struct include VT include T' end)

    let type_valobj = VType.to_valobj VT.type_name
    
    let type_meta_entry (m: Meta.t): VT.t option =
        Option.bind (Meta.get_valobj type_valobj m) T'.of_val
    let type_meta_entry_val (v: Val.t): VT.t option = type_meta_entry v.meta

end

module V = struct
    module Type = VType
    module Symbol = struct
        module T = MakeValType(struct
            type t = string
            let type_name = "symbol"
        end)
        include T
        include ValShow.Add(struct
            include T
            let pp out v = Format.fprintf out "%s" v
        end)
    end
    module List = struct
        module T = MakeValType(struct
            type t = Val.t list
            let type_name = "list"
        end)
        include T
        include ValShow.Add(struct
            include T
            let pp out v =
                let list_printer =
                    Format.pp_print_list
                        ~pp_sep:(fun out () -> Format.fprintf out "@ ")
                        ValShow.pp
                in
                Format.fprintf out "(@[<hov>%a@])" list_printer v
        end)
    end
    module Int = struct
        module T = MakeValType(struct
            type t = int
            let type_name = "int"
        end)
        include T
        include ValShow.Add(struct
            include T
            let pp out v = Format.fprintf out "%i" v
        end)
    end
    module Float = struct
        module T = MakeValType(struct
            type t = float
            let type_name = "float"
        end)
        include T
        include ValShow.Add(struct
            include T
            let pp out v = Format.fprintf out "%f" v
        end)
    end
    module String = struct
        module T = MakeValType(struct
            type t = string
            let type_name = "string"
        end)
        include T
        include ValShow.Add(struct
            include T
            let pp out s = Format.fprintf out "%S" s
        end)
    end
    module Bool = struct
        module T = MakeValType(struct
            type t = bool
            let type_name = "bool"
        end)
        include T
        include ValShow.Add(struct
            include T
            let pp out v = Format.fprintf out (if v then "#t" else "#f")
        end)
    end
end

