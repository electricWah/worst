
let gen_type_id =
    let id = ref 1 in
    fun () -> begin
        let id' = !id in
        id := id' + 1;
        id'
    end

module Val = struct
    type t = {
        type_id: int;
        v: Obj.t;
    }
end

module type Type = sig
    type t
    val type_id: int
    val to_val: t -> Val.t
    val of_val: Val.t -> t option
end

module MakeType = functor(T: sig type t end) -> struct
    type t = T.t
    let type_id = gen_type_id()

    let to_val (v: t): Val.t = { Val.type_id; v = Obj.repr v }
    let of_val (v: Val.t): t option = 
        if v.type_id = type_id then Some (Obj.obj v.v) else None
end

module MakeTypeData = functor(T: sig type tdata end) -> struct
    module Lookup = Hashtbl.Make(struct include Int let hash = Hashtbl.hash end)
    let class_data: T.tdata Lookup.t = Lookup.create 30

    let find_type (module M: Type) = Lookup.find_opt class_data M.type_id
    let find_val (v: Val.t) = Lookup.find_opt class_data v.type_id

    let install (module M: Type) (data: T.tdata) =
        Lookup.add class_data M.type_id data

end

module TypeName = struct
    module TC = MakeTypeData(struct type tdata = string end)
    include TC

    module type T = sig
        include Type
        val type_name: string
    end

    module Add = functor(M: T) -> struct
        include M
        let () = TC.install (module M) M.type_name
    end

    let of_val v = TC.find_val v

end

module ValCompare = struct
    module type T = sig
        include Type
        val compare: t -> t -> int
    end
    module TC = MakeTypeData(struct type tdata = (module T) end)
    module Add = functor(M: T) -> struct
        let () = TC.install (module M) (module M)
        let compare = M.compare
    end

    let compare (a: Val.t) (b: Val.t) =
        match Stdlib.compare a.type_id b.type_id with
        | 0 -> begin
            match TC.find_val a with
            | Some td -> begin
                let module M = (val td: T) in
                let a' = Option.get (M.of_val a) in
                let b' = Option.get (M.of_val b) in
                M.compare a' b'
            end
            | None -> compare a.v b.v
        end
        | x -> x
end

module ValEqual = struct
    module type T = sig
        include Type
        val equal: t -> t -> bool
    end
    module TC = MakeTypeData(struct type tdata = (module T) end)
    module Add = functor(M: T) -> struct
        let () = TC.install (module M) (module M)
        let equal = M.equal
    end

    let equal (a: Val.t) (b: Val.t) =
        if a.type_id == b.type_id then
            match TC.find_val a with
            | Some td -> begin
                let module M = (val td: T) in
                let a' = Option.get (M.of_val a) in
                let b' = Option.get (M.of_val b) in
                M.equal a' b'
            end
            | None -> a.v == b.v
        else false
end

module ValShow = struct
    module type T = sig
        include Type
        val pp: Format.formatter -> t -> unit
    end
    module TC = MakeTypeData(struct type tdata = (module T) end)
    module Add = functor(M: T) -> struct
        let () = TC.install (module M) (module M)
        let pp = M.pp
    end

    let pp out (v: Val.t) =
        match TC.find_val v with
        | Some td ->
            let module M = (val td: T) in
            let v' = Option.get (M.of_val v) in
            M.pp out v'
        | None ->
            let tn = Option.value (TypeName.of_val v) ~default:"value" in
            Format.fprintf out "<%s>" tn

end

module ValInputPort = struct
    module type T = sig
        include Type
        val read_into_bytes: ?start:int -> ?len:int -> bytes -> t -> int
    end

    module type Tv = sig
        val read_into_bytes: ?start:int -> ?len:int -> bytes -> int
    end

    module TC = MakeTypeData(struct type tdata = (module T) end)

    module Add = functor(M: T) -> struct
        let () = TC.install (module M) (module M)
    end

    type t = (module Tv)
    let type_name = "input-port"

    let of_val (v: Val.t) =
        match TC.find_val v with
        | None -> None
        | Some td ->
            let module M = (val td: T) in
            let v' = Option.get (M.of_val v) in
            let module V = struct
                let read_into_bytes ?start ?len dest =
                    M.read_into_bytes ?start ?len dest v'
            end in
            Some (module V: Tv)

    let read_into_bytes ?start ?len (module V: Tv) (b: bytes) =
        V.read_into_bytes ?start ?len b

    let read_into_buffer ?(bufsize=4096) ?(len=max_int) (module V: Tv) (b: Buffer.t) =
        let buf = Bytes.create bufsize in
        let rec read_all rem count =
            match V.read_into_bytes ~len:(min rem bufsize) buf with
            | 0 -> count
            | c ->
                Buffer.add_subbytes b buf 0 c;
                read_all (rem - c) (count + c)
        in read_all len 0
end

module ValOutputPort = struct
    module type T = sig
        include Type
        val write_bytes: ?from:int -> ?len:int -> bytes -> t -> int
        val flush: t -> unit
    end

    module type Tv = sig
        val write_bytes: ?from:int -> ?len:int -> bytes -> int
        val flush: unit -> unit
    end

    module TC = MakeTypeData(struct type tdata = (module T) end)

    module Add = functor(M: T) -> struct
        let () = TC.install (module M) (module M)
    end

    type t = (module Tv)
    let type_name = "output-port"

    let of_val (v: Val.t) =
        match TC.find_val v with
        | None -> None
        | Some td ->
            let module M = (val td: T) in
            let v' = Option.get (M.of_val v) in
            let module V = struct
                let write_bytes ?from ?len src = M.write_bytes ?from ?len src v'
                let flush () = M.flush v'
            end in
            Some (module V: Tv)

    let write_string (module V: Tv) (s: string) =
        V.write_bytes (Bytes.of_string s)

    let flush (module V: Tv) = V.flush()
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
    include Type
    val type_val: Val.t
end

module MakeValType = functor (VT: sig
        type t
        val type_name: string
    end) -> struct

    module T' = MakeType(VT)
    include T'
    include TypeName.Add(struct include VT include T' end)

    let type_val = VType.to_val VT.type_name
end

