
; Emitting code (or at least turning it into strings)

weakly
define cil/expr->string [
    import syntax/case
    define value->string [
        case {
            (cil/expr?) { cil/expr-value swap drop value->string }
            (string?) { to-string/debug swap drop }
            (bool?) { if ["true"] ["false"] }
            (list?) {
                list-map [10 ->string/prec]
                ", " string-join
                "{" swap string-append
                "}" string-append
            }
            ; don't forget map
            #t (->string)
        }
    ]
    define ->string/prec [
        const oprec
        cil/expr? if [
            cil/expr-kind
            #t equal? if [ drop drop value->string ] [
                drop const iprec
                cil/expr-value swap drop
                list-map [ string? if [] [ iprec ->string/prec ] ]
                "" string-join
                oprec iprec ascending? bury drop drop if [
                    "(" swap string-append
                    ")" string-append
                ] []
            ]
        ] [ value->string ]
    ]
    10 ->string/prec
]
export-name cil/expr->string

; [val1 ...valN] [name1 ...nameM] new? cil/emit-assignment
; -> name1... nameM now have values
define cil/emit-assignment [
    const new?
    const names
    const vals

    names list-empty? swap drop
    if [[]] [
        [
            new? if ["local "] []
            names list-map [cil/expr->string]
            ", " list-join list-iterate []
            vals list-empty? if [drop] [
                " = "
                swap
                list-map [cil/expr->string]
                ", " list-join list-iterate []
            ]
        ]
        list-eval
    ]
    list-empty? if [drop] [ cil/emit-statement ]
]
export-name cil/emit-assignment

define cil/eval->string [
    const stmts
    const args
    const rets

    [
        ; out statements on the stack instead
        define cil/emit-statement []
        args list-empty? if [drop] [
            [...] swap #t cil/emit-assignment
        ]
        stmts list-iterate []
        rets list-empty? if [drop] [
            drop
            [
                "return "
                rets list-map [cil/expr->string]
                ", " list-join
                list-iterate []
            ] list-eval
        ]
    ] list-eval
    list-map [
        ; list-map [ string? if [] [cil/expr->string] ]
        "" string-join
    ]
    "\n" string-join
]
export-name cil/eval->string

; Convenience wrapper
define cil/eval-program->string [
    cil/eval cil/eval->string
]
export-name cil/eval-program->string

; vi: ft=scheme


