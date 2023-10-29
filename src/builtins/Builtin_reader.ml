
(* open Worst_base;; *)

Builtin.(define "read-string->list" @@ begin
    let* src = S.String.pop in
    let chars = String.to_seq src in
    let reader = Reader.empty() in
    let reader = Reader.read_seq reader (Seq.to_dispenser chars) in
    let body = List.of_seq (Reader.all_vals ~eof:true reader) in
    S.List.push body
end);;

(* Builtin.(define "read-port->list" @@ begin *)
(*     let* p = S.InputPort.pop in *)
(*     let buf = Buffer.create 1000 in *)
(*     let _len = Base.ValInputPort.read_into p buf in *)
(*     let bufs = Buffer.to_seq buf in *)
(*     let reader = Reader.empty() in *)
(*     let reader = Reader.read_seq reader (Seq.to_dispenser bufs) in *)
(*     let body = Stdlib.List.of_seq (Reader.all_vals ~eof:true reader) in *)
(*     S.List.push body *)
(* end);; *)

