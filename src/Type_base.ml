
open Val_base

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
    include ValEqual.Add(struct include T include Int end)
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

module In_channel = struct
    module T = MakeValType(struct
        type t = in_channel
        let type_name = "in-channel"
    end)
    include T
end

module Out_channel = struct
    module T = MakeValType(struct
        type t = out_channel
        let type_name = "out-channel"
    end)
    include T
    include ValOutputPort.Add(struct
        include T
        let write_bytes ?(from=0) ?len data p =
            let from = max from 0 in
            let dlen = Bytes.length data in
            let len = min (Option.value len ~default:dlen) dlen in
            Stdlib.output p data from len;
            len

        let flush v = Stdlib.flush v
    end)
end

module UnixFileDescr = struct
    module T = MakeValType(struct
        type t = Unix.file_descr
        let type_name = "file-handle"
    end)
    include T
    include ValInputPort.Add(struct
        include T
        let read_into_bytes ?(start=0) ?len buf src =
            let start = max start 0 in
            let buflen = Bytes.length buf in
            let len = min (Option.value len ~default:buflen) buflen in
            Unix.read src buf start len
    end)
    include ValOutputPort.Add(struct
        include T
        let write_bytes ?(from=0) ?len data p =
            let from = max from 0 in
            let dlen = Bytes.length data in
            let len = min (Option.value len ~default:dlen) dlen in
            Unix.write p data from len

        let flush v = Unix.fsync v
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

module Lookup = struct
    module M = Map.Make(struct
        include Val
        let compare = ValCompare.compare
    end)
    module T = MakeValType(struct
        type t = Val.t M.t
        let type_name = "lookup"
    end)
    include T
    (* include ValShow.Add(struct *)
    (*     include T *)
    (*     let pp out v = *)
    (*         let list_printer = *)
    (*             Format.pp_print_list *)
    (*                 ~pp_sep:(fun out () -> Format.fprintf out "@ ") *)
    (*                 ValShow.pp *)
    (*         in *)
    (*         Format.fprintf out "(@[<hov>%a@])" list_printer v *)
    (* end) *)

    let empty: t = M.empty
    let get (k: Val.t) (v: t): Val.t option = M.find_opt k v
end

