
open Worst
(* open Ty *)
module I = Interpreter

include Builtins

let () =
    match Sys.argv with
    | [| _; filename |] ->
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
        (* Format.printf "%a\n" ValShow.pp (Option.get t) *)

    | _ -> Format.printf "no\n"
