
; Data types for CIL - compiled version

import cil/bootstrap/base

; type key create-accessor -> an accessor function
[
    cil/expect-value const key
    cil/expect-value const type
    lua-interp-definition accessor [
        1 type cil/interp-stack-ref/type
        [ -> [key] ] cil/lua-expr
    ]
]
cil/eval-interpreter->builtin
quote cil/data/create-accessor definition-add

[
    cil/expect-value const key
    cil/expect-value const type
    lua-interp-definition accessor [
        cil/expect-value const value
        1 type cil/interp-stack-ref/type
        [ -> [key] ] cil/lua-expr
        value cil/lua-assign
    ]
]
cil/eval-interpreter->builtin
quote cil/data/create-setter definition-add

[
    cil/expect-value const type
    [ "base" lua-require -> Type ] cil/lua-expr const type_t
    lua-interp-definition expr_is [
        cil/new-id-name val
        1 cil/interp-stack-ref const v
        [ type_t -> is .* #t (type v) ] cil/lua-expr
    ]
]
cil/eval-interpreter->builtin
quote cil/data/create-predicate definition-add

[
    cil/expect-value const type
    lua-interp-definition constructor [ type [ ] lua-setmetatable ]
]
cil/eval-interpreter->builtin
quote cil/data/create-constructor definition-add



[ [ "base" lua-require -> Type -> new .* #t ("cil/expr") ] cil/lua-expr ]
cil/eval-interpreter->builtin
eval const %cil/expr-type

%cil/expr-type
[
    cil/expect-value const type
    lua-interp-definition constructor [
        cil/expect-value const value
        cil/expect-value const kind
        type [ ] lua-setmetatable luavar v
        [ v -> kind ] cil/lua-expr
        kind cil/lua-assign
        [ v -> value ] cil/lua-expr
        value cil/lua-assign
        v
    ]
]
cil/eval-interpreter->builtin
eval
quote *cil/make-expr definition-add

; TODO this is not cloned
%cil/expr-type
[
    cil/expect-value const type
    lua-interp-definition expr_set_callable [
        cil/expect-value const o
        cil/expect-value const i
        1 type cil/interp-stack-ref/type const v
        [ v -> outputs ] cil/lua-expr o cil/lua-assign
        [ v -> inputs ] cil/lua-expr i cil/lua-assign
    ]
]
cil/eval-interpreter->builtin
eval
quote *cil/set-expr-callable definition-add

%cil/expr-type
[
    cil/expect-value const type
    lua-interp-definition expr_set_tostring [
        cil/expect-value const s
        1 type cil/interp-stack-ref/type const v
        [ v -> to_string ] cil/lua-expr s cil/lua-assign
    ]
]
cil/eval-interpreter->builtin
eval
quote *cil/set-expr-tostring definition-add

%cil/expr-type cil/data/create-predicate
quote cil/expr? definition-add

%cil/expr-type "kind" cil/data/create-accessor
quote cil/expr-kind definition-add
%cil/expr-type "value" cil/data/create-accessor
quote cil/expr-value definition-add
%cil/expr-type "inputs" cil/data/create-accessor
quote cil/expr-inputs definition-add
%cil/expr-type "outputs" cil/data/create-accessor
quote cil/expr-outputs definition-add

%cil/expr-type "to_string" cil/data/create-setter
quote cil/set-expr-tostring definition-add

quote %cil/expr-type definition-remove
quote *cil/make-expr quote cil/make-expr definition-rename
quote *cil/set-expr-callable quote cil/set-expr-callable definition-rename
quote *cil/set-expr-tostring quote cil/set-expr-tostring definition-rename

[
    [ cil/new-id-name type [ "base" lua-require -> Type ] cil/lua-expr ] eval
    luavar type_t
    [ type_t -> new .* #t ("cil/expr") ] cil/lua-expr
    luavar expr_type

    lua-interp-definition expr_type_builtin [ expr_type ]
    "%cil/expr-type" cil/interp-define

    lua-interp-definition expr_is [
        cil/new-id-name val
        1 cil/interp-stack-ref const v
        [ type_t -> is .* #t (expr_type v) ] cil/lua-expr
    ]
    "cil/expr?" cil/interp-define

    lua-interp-definition expr_make [
        cil/expect-value const kind
        cil/expect-value const value

        expr_type [] lua-setmetatable luavar v
        
        [ v -> kind ] cil/lua-expr
        kind cil/lua-assign
        [ v -> value ] cil/lua-expr
        value cil/lua-assign
        v
    ]
    "cil/make-expr" cil/interp-define

    lua-interp-definition expr_kind [
        1 expr_type cil/interp-stack-ref/type
        [ -> kind ] cil/lua-expr
    ]
    "cil/expr-kind" cil/interp-define

    lua-interp-definition expr_value [
        1 expr_type cil/interp-stack-ref/type
        [ -> value ] cil/lua-expr
    ]
    "cil/expr-value" cil/interp-define

    ; TODO this is not cloned
    lua-interp-definition expr_set_callable [
        cil/expect-value const o
        cil/expect-value const i
        1 expr_type cil/interp-stack-ref/type const v
        [ v -> outputs ] cil/lua-expr o cil/lua-assign
        [ v -> inputs ] cil/lua-expr i cil/lua-assign
    ]
    "cil/set-expr-callable" cil/interp-define

    lua-interp-definition expr_value [
        1 expr_type cil/interp-stack-ref/type
        [ -> inputs ] cil/lua-expr
    ]
    "cil/expr-callable-inputs" cil/interp-define
    lua-interp-definition expr_value [
        1 expr_type cil/interp-stack-ref/type
        [ -> outputs ] cil/lua-expr
    ]
    "cil/expr-callable-outputs" cil/interp-define

    lua-interp-definition expr_set_callable [
        cil/expect-value const s
        1 expr_type cil/interp-stack-ref/type const v
        [ v -> to_string ] cil/lua-expr s cil/lua-assign
    ]
    "cil/set-expr-tostring" cil/interp-define

    ; TODO __tostring using recursive function definition
    ; using "local function"
    
    "" cil/comment
]
drop
; cil/eval-interpreter->builtin
; eval

; export-all

; vi: ft=scheme

