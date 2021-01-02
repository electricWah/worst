
define luavar [
    upquote const name

    [] swap list-push
    const value

    [
        name cil/set-new-id-name
        cil/new-id #t cil/make-expr
    ] eval
    const variable

    value [variable] list-eval
    #t cil/emit-assignment

    quote variable definition-resolve swap drop
    name updo definition-add
]

define => [
    upquote call const target
    [] swap list-push
    [target] list-eval
    #f cil/emit-assignment
]

; expr1 expr2 lua-assign1 -> expr1 = expr2
define cil/lua-assign [
    [] swap list-push
    swap [] swap list-push
    #f cil/emit-assignment
]

define lua-definition [
    upquote const %ldefname
    upquote const %ldefbody

    %ldefbody
    [ %ldefname cil/set-new-id-name cil/new-id ] eval ->string
    cil/function-def
]

define lua-interp-definition [
    upquote const %ldefname
    upquote const %ldefbody

    [ %ldefname cil/set-new-id-name cil/new-id ] eval ->string
    const %fname

    %ldefbody cil/interpreter-eval
    %fname
    cil/eval->function-def
]

define lua-callable [
    [ cil/function-call ] swap list-push
    upquote
    updo definition-add
]

export-all

; vi: ft=scheme

