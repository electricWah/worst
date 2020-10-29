
; Lua expr construction

; 0     variables, a.b, f(), a:b(), the a in a[b], (expr)
; 1     ^   (right associative - take care)
; 2     not # - (unary)
; 3     * / %
; 4     + -
; 5     ..  (right associative but probably doesn't matter)
; 6     < > <= >= ~= ==
; 7     and
; 8     or
; 10    return, assignment, the b in a[b]

; reassign:
; nil
; not = !
; unary - = ~
; and = && 
; or = ||
; a[b] = a -> [b]
; f() = f . ()

; also f .* n () and obj :* method n ()
; -> n is the number of return values

; lua-expr [ body ] -> one expr
define cil/lua-expr [
    define nil [ quote nil #t cil/make-expr ]
    define ... [ quote ... #t cil/make-expr ]

    define -> [
        upquote const key-expr
        cil/expect-value const obj
        key-expr eval const key
        [ obj "[" key "]" ] list-eval
        0 cil/make-expr
    ]

    ; expr R -> ret0 ret1 ... retR
    define %cil/lua-mulrets [
        const rcount
        const valexpr

        rcount
        equals? #t if [
            ; Single return value, inlined
            drop valexpr
        ] [
            equals? 0 if [
                ; No return value, side effect only
                drop
                [ valexpr cil/expr->string ] list-eval
                cil/emit-statement
            ] [
                ; Multiple return values
                list-imake [
                    drop 
                    cil/new-id #t cil/make-expr
                ]
                const mrets

                [ valexpr ] list-eval const mvals

                mvals mrets #t cil/emit-assignment

                mrets list-iterate []
            ]
        ]
    ]

    define :* [
        cil/expect-value const %obj
        upquote const %mname
        upquote const %mn
        upquote const %margs

        %margs cil/list-eval
        const margs

        [
            %obj ":" %mname "("
                margs list-map [cil/expr->string]
                "," list-join
                list-iterate []
            ")"
        ]
        list-eval
        0 cil/make-expr
        %mn %cil/lua-mulrets
    ]

    define .* [
        cil/expect-value const %f
        upquote const %fn
        upquote const %fargs

        %fargs
        cil/list-eval const fargs
        [
            %f "("
                fargs list-map [cil/expr->string]
                "," list-join
                list-iterate []
            ")"
        ]
        list-eval

        0 cil/make-expr
        %fn %cil/lua-mulrets
    ]

    define lua-2op [
        const prec
        const op
        cil/expect-value
        cil/expr? if [] [#t cil/make-expr] const rhs
        cil/expect-value
        cil/expr? if [] [#t cil/make-expr] const lhs

        [ lhs " " op " " rhs ] list-eval
        prec cil/make-expr
    ]

    define lua-1op [
        const prec
        const op
        cil/expect-value
        cil/expr? if [] [#t cil/make-expr] const v

        [ op " " v ] list-eval
        prec cil/make-expr
    ]

    define ^ [ quote ^ 1 lua-2op ]
    define ! [ quote not 2 lua-1op ]
    define # [ quote # 2 lua-1op ]
    define ~ [ quote - 2 lua-1op ]
    define * [ quote * 3 lua-2op ]
    define / [ quote / 3 lua-2op ]
    define % [ quote % 3 lua-2op ]
    define + [ quote + 4 lua-2op ]
    define - [ quote - 4 lua-2op ]
    define .. [ quote .. 5 lua-2op ]
    define < [ quote < 6 lua-2op ]
    define > [ quote > 6 lua-2op ]
    define <= [ quote <= 6 lua-2op ]
    define >= [ quote >= 6 lua-2op ]
    define ~= [ quote ~= 6 lua-2op ]
    define == [ quote == 6 lua-2op ]
    define && [ quote and 7 lua-2op ]
    define || [ quote or 8 lua-2op ]

    ; TODO remove this when definitions are aware
    #f cil/set-emit-mode
    eval
]
export-name cil/lua-expr

; vi: ft=scheme

