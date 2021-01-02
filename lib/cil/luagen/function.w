
; function expr and function definition
; I'm sure I already did this somewhere

; [ body ... ] cil/eval name cil/eval->function-def -> function def
define cil/eval->function-def [
    const %fname

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

    [ "cil/lua-function-value<" %fname ">" ] list-eval "" string-join
    cil/set-expr-tostring

    args olen cil/set-expr-callable
]
export-name cil/eval->function-def

define cil/function-def [ const %fname cil/eval %fname cil/eval->function-def ]
export-name cil/function-def

; args... function-var cil/lua-function-call -> outs...
; 
define cil/function-call [
    cil/expr-callable-inputs
    list-length const arglen
    const arguments drop

    cil/expr-callable-outputs const outs
    drop

    const func
    func
    cil/expr->string
    const funcstr

    [ funcstr cil/set-new-id-name arglen cil/expect-values ] list-eval
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

