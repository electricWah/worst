
define lua-assignment->string [
    import list

    quote var map-get const stmt-var drop
    quote val map-get const stmt-val drop
    drop

    stmt-var
    quote declared
    map-get const declared
    #t map-set
    drop
    
    [ declared if [] ["local "] stmt-var " = " stmt-val ] list-eval
    #f make-lua-expr
    lua-expr->string
]
export-name lua-assignment->string

define lua-expr->string [
    import list
    import syntax/case
    define value->string [
        case {
            (lua-expr?) {
                lua-expr-unwrap value->string
            }
            (list?) {
                ; ??
                list-map [10 ->string/prec]
                ", " string-join
                "{" swap string-append
                "}" string-append
            }
            (map?) {
                map-keys list-map [
                    const k
                    k
                    map-get const v
                    symbol? if [ ->string ] [
                        10 ->string/prec
                        "[" swap string-append
                        "]" string-append
                    ]
                    " = " string-append
                    v 10 ->string/prec
                    string-append
                ]
                swap drop
                ", " string-join
                "{" swap string-append
                "}" string-append
            }
            #t (->string)
        }
    ]
    define ->string/prec [
        const oprec
        lua-expr? if [
            quote %expr map-get swap drop
            #t equal? if [ drop drop value->string ] [
                drop const iprec
                lua-expr-unwrap
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
export-name lua-expr->string

define lua-statement->string [
    import syntax/case
    case {
        (false?) { }
        (lua-expr?) { lua-expr->string }
        (lua-assignment?) { lua-assignment->string }
        #t { ["lua-statement->string unexpected"] swap list-append abort }
    }
]
export-name lua-statement->string

; vi: ft=scheme

