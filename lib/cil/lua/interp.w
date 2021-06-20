
define cil/lua-interpreter-eval* [
    const %body

    define interpreter-quote [
        [] %interpreter "body_read" 1 cil/lua-method-call
    ]

    define interpreter-stack-push [
        1 cil/expect-values/list
        %interpreter "stack_push" 0 cil/lua-method-call
    ]

    define interpreter-eval [
        1 cil/expect-values/list
        %interpreter "eval" 0 cil/lua-method-call
    ]

    ; modname funcname interpreter-emit-modcall ->
    ; interpreter:eval(require(modname)[funcname])
    define interpreter-modcall [

        const %funcname
        const %modname
        %interpreter list-push const %args
        %modname [] swap list-push
        "require" #t cil/lua-function-call
        %funcname cil/lua-index
        %args swap 
    ]

    define interpreter-emit-if-then-else [
        3 cil/expect-values/list
        "cil/lua/control" quote emit_if_then_else interpreter-modcall
        0 cil/lua-function-call
    ]

    define interpreter-emit-loop [
        1 cil/expect-values/list
        "cil/lua/control" quote emit_loop interpreter-modcall
        0 cil/lua-function-call
    ]

    define interpreter-expect-value [
        [] "cil/base" quote expect_value interpreter-modcall
        1 cil/lua-function-call
    ]

    [ cil/expect-value const %interpreter ]
    [] %body list-push
    [ %interpreter cil/lua-interpreter-eval ]
    list-append list-append
    cil/eval->lua-chunk
]
export-name cil/lua-interpreter-eval*

define define-lua-builtin [
    upquote const %name
    upquote const %body

    [ cil/expect-value const %interpreter ]
    [] %body list-push
    [ %interpreter cil/lua-interpreter-eval ]
    list-append list-append

    define interpreter-quote [
        [] %interpreter "body_read" 1 cil/lua-method-call
    ]

    define interpreter-eval [
        cil/expect-value [] swap list-push
        %interpreter "eval" 0 cil/lua-method-call
    ]

    cil/eval->lua-chunk
    stack-dump
    lua-load-string
    false? if [ abort ] []
    %name updo definition-add
]
export-name define-lua-builtin

; cil/lua-eval-context [ body ]
; 

; vi: ft=scheme

