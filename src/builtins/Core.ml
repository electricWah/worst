
Builtin.(define_type (module V.Symbol));;
Builtin.(define_type (module V.List));;
Builtin.(define_type (module V.Int));;
Builtin.(define_type (module V.Float));;
Builtin.(define_type (module V.String));;
Builtin.(define_type (module V.Bool));;

Builtin.(define "drop" @@ begin
    let* _drop = S.pop in
    ok
end);;

Builtin.(define "clone" @@ begin
    let* v = S.pop in
    S.push v
    >> S.push v
end);;

Builtin.(define "swap" @@ begin
    let* b = S.pop in
    let* a = S.pop in
    S.push b >> S.push a
end);;

Builtin.(define "dig" @@ begin
    let* a = S.pop in
    let* b = S.pop in
    let* c = S.pop in
    S.push b >> S.push a >> S.push c
end);;

Builtin.(define "bury" @@ begin
    let* a = S.pop in
    let* b = S.pop in
    let* c = S.pop in
    S.push a >> S.push c >> S.push b
end);;

Builtin.(define "not" @@ begin
    let* v = S.pop in
    let v' = V.Bool.of_val v == Some false in
    S.Bool.push v'
end);;

Builtin.(define "eval" @@ begin
    let* v = S.pop in
    I.eval_next_val v
end);;

Builtin.(define "eval-if" @@ begin
    let* e = S.pop in
    let* cond = S.Bool.pop in
    if cond then I.eval_next_val e else ok
end);;

Builtin.(define "quote" @@ begin
    let* v = I.body_next_exn in
    S.push v
end);;

Builtin.(define "uplevel" @@ begin
    I.into_parent_frame >>
    let* v = S.pop in
    I.eval_next_val v
end);;

Builtin.(define "upquote" @@ begin
    I.into_parent_frame >>
    let* v = I.body_next_exn in
    S.push v
end);;

