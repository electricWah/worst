
;  [[test-definition egg [test-if [7 test-add] [6 test-add]]] compile-block] lua-compile

; expressions are as usual
; statements are emitted:
; expr expr->var name -> emit "var = expr;"; name
; ((expr body) ... (body)) conds
; body expr while-loop
; body list for-iter


; [ body ... ] lua-compile

; expr ignore -> ; expr
; expr var assign -> var ; var = expr
; variable/uninit -> var ; local var
; expr variable -> var ; local var = expr
; expr count variable/n -> var ; local var,...count = expr
; [expr...] returns -> return expr, ...
; count return/n -> return expr,...count
; c [tbody...] [fbody...] if-else -> if (c) then tbody... else fbody... end
; c [tbody...] #f if-else -> if (c) then tbody... end
; c [body...] while-do -> while (c) do body... end
; name [body...] arg-count definition -> function name(args...arg-count) body end
; [args...] name funcall -> name(args, ...)
; expr1 expr2 binop op -> (expr1 op expr2)
; break -> break
; expr1 expr2 table-index -> expr1[expr2]
; expr name table-dot -> expr.name
; expr name table-colon -> expr:name
; expr dot field-name -> expr.field-name
; [args ...] expr method-call field-name -> expr:field-name(args...)
; TODO
; [k1 v1 k2 v2 ...] table-literal -> { k1 = v1, k2 = v2, ... }
; everything else is just run as normal code
define lua-write [
    import syntax/variable
    import syntax/cond
    import list

    define list-csv->string [
        list-length cond [
            (equals? 0) [drop drop ""]
            (equals? 1) [
                drop
                list-pop swap drop
                ->expr expr->string
            ]
            (#t) [
                drop
                list-pop
                ->expr expr->string
                swap
                list-iterate [
                    swap ", " string-append swap
                    ->expr expr->string string-append
                ]
            ]
        ]
    ]

    define wrapped [ swap string-append upquote string-append ]
    define parenthesise [ "(" wrapped ")" ]
    define wrap-braces [ "{" wrapped "}" ]

    define tuple->string [ list-csv->string parenthesise ]

    ; an expr has already been stringified
    quote expr% gensym const %expr
    define expr? [
        list? if [
            list-length equals? 2 if [
                drop list-head %expr equal?
                bury drop drop
            ] [ drop #f ]
        ] [
            #f
        ]
    ]
    define make-expr [ [] swap list-push %expr list-push ]
    define expr-unwrap! [ 1 list-ref! swap drop ]
    define expr->string [ expr-unwrap! ]

    define ->expr [
        cond [
            (number?) [ ->string make-expr ]
            (string?) [ ->string make-expr ]
            (expr?) []
            (equals? #t) [ drop "true" make-expr ]
            (equals? #f) [ drop "false" make-expr ]
            (list?) [ list-csv->string wrap-braces make-expr ]
            (#t) [ "Don't know how to turn this into Lua" abort ]
        ]
    ]

    define binop [
        upquote const op
        ->expr expr-unwrap!
        swap ->expr expr-unwrap!
        [
            " + " const add
            " - " const sub
            " * " const mul
            " / " const div
            " .. " const string-append

            op call
        ] eval
        string-append
        swap
        string-append
        parenthesise
        make-expr
    ]

    define funcall [
        ->expr expr->string swap
        tuple->string string-append
        make-expr
    ]

    define table-dot [
        ->string const name
        ->expr expr->string
        "." string-append
        name string-append
        make-expr
    ]
    define dot [ upuote table-dot ]

    define table-colon [
        ->string const name
        ->expr expr->string
        ":" string-append
        name string-append
        make-expr
    ]
    define method-call [ upquote table-colon funcall ]

    define table-index [
        ->expr expr->string
        "[" swap string-append
        "]" string-append
        swap ->expr expr->string
        swap string-append
        make-expr
    ]

    ; define table-literal [
    ;     wrap-braces make-expr
    ; ]

    define variable/uninit [
        quote v gensym ->string const var
        emit-indent
        "local " emit
        var emit emit-done
        var make-expr
    ]

    define variable [
        quote v gensym ->string const var
        emit-indent
        "local " emit
        var emit
        " = " emit
        ->expr expr->string emit
        emit-done
        var make-expr
    ]

    define variable/n [
        const n
        [] n do-times [
            gensym ->string
            swap list-push
        ]
        const vars
        emit-indent
        "local " emit
        list-csv->string emit
        " = " emit
        ->expr expr->string emit
        emit-done
        ; TODO may need reversing
        vars list-iterate [ make-expr ]
    ]

    define assign [
        const var
        emit-indent
        var expr->string emit
        " = " emit
        ->expr expr->string emit
        emit-done
        var
    ]

    define ignore [ emit-indent ->expr expr->string emit emit-done ]

    ; if-elseif-else?
    define if-else [
        const do-else
        const do-if
        const if-this

        emit-indent
        "if " emit
        if-this ->expr expr->string parenthesise emit
        " then" emit emit-done
        do-if indented
        do-else false? if [drop] [
            emit-indent
            "else" emit emit-done
            indented
        ]
        emit-indent
        "end" emit
        emit-done
    ]

    define while-do [
        const do-this
        const while-this

        emit-indent
        "while " emit
        while-this ->expr expr->string parenthesise emit
        " do" emit emit-done
        do-this indented
        emit-indent "end" emit emit-done
    ]

    define definition [
        const name
        const argcount
        const body

        ; TODO generate args in compiler
        [] argcount do-times [
            quote arg gensym ->string make-expr list-push
        ]
        const args

        emit-indent
        "function " emit
        name ->string emit
        args tuple->string emit
        emit-done

        args list-iterate []
        body indented

        emit-indent
        "end" emit
        emit-done
    ]

    define return/n [
        const n
        [] n do-times [
            swap list-push
        ]
        emit-indent
        "return " emit
        list-csv->string emit
        emit-done
    ]

    define break [ emit-indent "break" emit emit-done ]

    define emit [
        print
    ]
    define emit-done [
        "\n" print
    ]

    0 const indentation

    define emit-indent [
        indentation do-times ["    " print]
    ]

    define indented [
        indentation 1 add const indentation
        eval
    ]
    
    eval
]
export-name lua-write

; vi: ft=scheme

