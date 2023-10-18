
Builtin.(define "value->constant" @@ begin
    let* v = S.pop in
    let b = I.BuiltinVal.to_val (S.push v) in
    S.push b
end);;

Builtin.(define "value-insert-meta-entry" @@ begin
    let* entry = S.pop in
    let* key = S.pop in
    let* v = S.pop in
    S.push (Ty.Meta.update_val key.v entry v)
end);;

