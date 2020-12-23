
; function expr and function definition
; I'm sure I already did this somewhere

define cil/function-def [
    const %fname
    const %fbody

    %fbody
    cil/eval-fragment
    const stmts
    list-length const ilen const args
    list-length const olen const outs

    [
        "function " %fname "("
        args list-map [cil/expr->string]
        ", " list-join
        list-iterate []
        ")"
    ]
    list-eval cil/emit-statement

    cil/do-indent [
        stmts list-iterate [cil/emit-statement]

        olen equals? 0 if [ drop ] [
            drop
            [ "return "
                outs list-map [cil/expr->string]
                ", " list-join
                list-iterate []
            ]
            list-eval cil/emit-statement
        ]

    ]

    ["end"] cil/emit-statement

    %fname string->symbol #t cil/make-expr
    ; TODO terrible
    quote arguments args map-set
    quote outs olen map-set
]
export-name cil/function-def

; args... function-var cil/lua-function-call -> outs...
; 
define cil/function-call [
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
export-name cil/function-call

; vi: ft=scheme

