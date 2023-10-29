
open Worst_base;;

module UnixFileDescrS = Builtin.Stack(Type.UnixFileDescr);;

Builtin.(define "command-line-arguments" @@ begin
    Sys.argv
    |> Array.to_list
    |> List.map V.String.to_val
    |> S.List.push
end);;

let open_options =
    let open_flags = Unix.([
        O_RDONLY;
        O_WRONLY;
        O_RDWR;
        O_NONBLOCK;
        O_APPEND;
        O_CREAT;
        O_TRUNC;
        O_EXCL;
        O_NOCTTY;
        O_DSYNC;
        O_SYNC;
        O_RSYNC;
        O_SHARE_DELETE;
        O_CLOEXEC;
        O_KEEPEXEC;
    ]) in
    let rec open_flags_of_int acc i = function
        | f :: fs ->
            let acc' = if Int.logand i 1 == 1 then f::acc else acc in
            let i' = Int.shift_right i 1 in
            open_flags_of_int acc' i' fs
        | [] -> acc
    in
    fun i -> open_flags_of_int [] i open_flags
;;

Builtin.(define "file-handle-open" @@ begin
    let* cperm = S.Int.pop in
    let* open_flags = S.Int.pop in
    let* path = S.String.pop in
    let opts = open_options open_flags in
    let fd = Unix.openfile path opts cperm in
    UnixFileDescrS.push fd
end);;

Builtin.(define "file-handle-close" @@ begin
    let* fh = UnixFileDescrS.pop in
    Unix.close fh;
    ok
end);;

(* TODO.unix_open_file_etc *)

