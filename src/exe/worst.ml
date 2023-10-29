
open Worst
(* open Ty *)
module I = Interpreter

include Builtins

let () =
    (* if zip get then run that else *)
    if Array.length Sys.argv >= 2 then
        let filename = Array.get Sys.argv 1 in
        let inch = In_channel.open_text filename in
        let reader = Reader.empty() in
        let reader = Reader.read_seq reader (fun () -> In_channel.input_char inch) in
        let body = List.of_seq (Reader.all_vals ~eof:true reader) in
        let i =
            I.make ~body ()
            |> Builtin.install_all
        in
        let _i' = I.run i in
        ()
    else Format.printf "no\n"

