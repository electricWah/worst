
Builtin.(define "string-append" @@ begin
    let* b = S.String.pop in
    let* a = S.String.pop in
    S.String.push (a ^ b)
end);;



