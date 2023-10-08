
Builtin.(define "println" @@ begin
    let* s = S.String.pop in
    Format.printf "%s@." s;
    ok
end)

    (* match Option.bind (I.stack_top i) V.String.of_val with *)
    (* | Some s -> *) 
    (* | None -> Format.printf "no\n" *)
    (* end; *)
    (* i *)

