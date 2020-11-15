
; def: function name(...) ... end
; expr: local name = function ...

; return value: a variable with extra fields, callable with lua-function-call
; arguments: list
; outs: int

; body name cil/lua-function-def -> var
define cil/lua-function-def [

    const %fname
    const %fbody

    cil/indent>
    cil/enter-new-emit-state

    %fbody
    cil/eval-chunk
    list-length const ilen const args
    list-length const olen const outs

    cil/indent<
    cil/emit-state-do-uplevel [
        ["function " %fname "(" args list-iterate [] ")"]
        list-eval cil/emit-statement
    ]
    cil/indent>

    outs args #f cil/emit-assignment

    olen equals? 0 if [ drop ] [
        drop
        [ "return "
            outs list-map [cil/expr->string]
            ", " list-join
            list-iterate []
        ]
        list-eval cil/emit-statement
    ]

    cil/indent<
    cil/leave-emit-state

    ["end"] cil/emit-statement

    %fname string->symbol #t cil/make-expr
    ; TODO terrible
    quote arguments args map-set
    quote outs olen map-set
]
export-name cil/lua-function-def

; body cil/lua-function-def -> var (with function properties on it)
define cil/lua-function-expr [
]

; args... function-var cil/lua-function-call -> outs...
; 
define cil/lua-function-call [
    interpreter-dump-stack
    quote arguments map-get const arguments drop
    quote outs map-get const outs drop

    const func
    func
    cil/expr->string
    interpreter-dump-stack
    const funcstr

    arguments list-length swap drop
    [ funcstr const cil/new-id-name cil/expect-values ] list-eval
    interpreter-dump-stack
    const args

    [
        func
        quote .*
        outs
        args
    ]
    list-eval
    cil/lua-expr
]
export-name cil/lua-function-call

; vi: ft=scheme

