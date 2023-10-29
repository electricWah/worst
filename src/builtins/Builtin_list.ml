
(* open Worst_base;; *)

Builtin.(define "list-length" @@ begin
    let* l = S.List.pop in
    S.Int.push (List.length l)
end);;

Builtin.(define "list-push" @@ begin
    let* v = S.pop in
    let* l = S.List.pop in
    S.List.push (v :: l)
end);;

Builtin.(define "list-pop" @@ begin
    let* l = S.List.pop in
    match l with
    | v :: ls ->
        S.List.push ls >>
        S.push v
    | [] ->
        S.List.push [] >>
        S.Bool.push false
end);;

Builtin.(define "list-reverse" @@ begin
    let* l = S.List.pop in
    S.List.push (List.rev l)
end);;

Builtin.(define "list-append" @@ begin
    let* b = S.List.pop in
    let* a = S.List.pop in
    S.List.push (List.append a b)
end);;

(* /// list n `list-get` -> value : get the value at index n of list. *)
(* /// 0-indexed, negative numbers are from the other end of the list, *)
(* /// and out of range gives false with error? as true. *)
(* pub fn list_get(i: &mut Interpreter) -> BuiltinRet { *)
(*     let n = i.stack_pop::<i64>()?; *)
(*     let l = i.stack_pop::<List>()?; *)
(*     let n = n.into_inner(); *)
(*     let l = l.as_ref(); *)
(*     let n = if n < 0 { l.len() as i64 + n } else { n }; *)
(*     i.stack_push_result(l.get(n as usize).cloned().ok_or(false)); *)
(*     Ok(()) *)
(* } *)

(* /// list n `list-split-at` -> list-tail list-head : split a list into two at index n. *)
(* /// 0-indexed, negative numbers are from the other end of the list, *)
(* /// and out of range indexes are saturated so that one of the lists is empty. *)
(* pub fn list_split_at(i: &mut Interpreter) -> BuiltinRet { *)
(*     let n = i.stack_pop::<i64>()?.into_inner(); *)
(*     let mut l = i.stack_pop::<List>()?; *)
(*     let len = l.as_ref().len() as i64; *)
(*     let n = if n < 0 { len + n } else { n }; *)
(*     let n = if n < 0 { 0 } else if n > len { len } else { n }; *)
(*     let head = l.as_mut().pop_n(n as usize); *)
(*     i.stack_push(l); *)
(*     i.stack_push(head); *)
(*     Ok(()) *)
(* } *)

