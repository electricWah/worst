
define lua-break [ ["break"] #f make-lua-expr lua-emit-statement ]
export-name lua-break

; [(c1 if1) (c2 if2) ...] lua-case -> if c1 then if1 elseif c2 ... end
define lua-case [
]

; cond ift iff lua-if-then-else -> if cond then ift else iff end
; TODO figure out the correct order of stack operations and stick with it everywhere
define lua-if-then-else [
    const %iff
    const %ift
    1 lua-expect-values
    const %c

    %c bool? if [ if [ %ift] [ %iff ] eval ] [
        drop

        %ift lua-eval-code const %tstate
        %iff lua-eval-code const %fstate

        import data/map

        %tstate
        . args list-length const t-arglen drop
        . returns list-length const t-retlen drop
        drop

        %fstate
        . args list-length const f-arglen drop
        . returns list-length const f-retlen drop
        drop

        t-arglen negate t-retlen add const t-size
        f-arglen negate f-retlen add const f-size
        t-size f-size equal? bury drop drop if [ ] [
            ("true and false arms have different arity") abort
        ]

        ; maximum number of vars to create
        ; max [ tin, tout, fin, fout ]
        t-arglen t-retlen f-arglen f-retlen
        max max max
        const vcount

        ; number of vars required as input
        t-arglen f-arglen
        max
        const icount

        ; number of vars unassigned
        vcount icount negate add
        const rcount

        ; TODO take this out of list-eval
        ; interpreter-dump-stack
        [icount lua-expect-values] list-eval
        const ivars

        rcount list-imake [ drop quote ifr lua-new-var ]
        const rvars

        rvars lua-declare-vars

        ivars rvars list-append
        ; list-reverse
        ; interpreter-dump-stack
        const rvars

        define write-arm [
            . args
            list-reverse
            rvars
            swap
            list-iterate [
                ; every arm arg is assigned from rvars
                swap list-pop swap bury
                make-lua-assignment
                ; quote new #t map-set
                lua-emit-statement
            ]
            drop ; remaining rvars

            . statements
            list-iterate [ lua-emit-statement ]

            . returns
            ; list-reverse
            rvars swap
            list-iterate [
                swap list-pop swap bury swap
                make-lua-assignment
                lua-emit-statement
            ]
            drop

            drop
        ]

        [ "if " %c " then" ] list-eval
        #f make-lua-expr
        lua-emit-statement

        %tstate write-arm
        ["else"] #f make-lua-expr lua-emit-statement
        %fstate write-arm
        ["end"] #f make-lua-expr lua-emit-statement

        rvars list-iterate []
    ]

]
export-name lua-if-then-else

; expr body lua-while -> while expr do body end
define lua-while [
    const %body
    1 lua-expect-values const %c

    %body lua-eval-code const %bstate

    import data/map

    %bstate
    . args
    list-length const arglen
    list-reverse
    const args
    . returns
    list-length const retlen
    const rets
    . statements
    const stmts
    drop

    arglen retlen equal? if [drop drop] [
        ["while body input and output arity must be equal"] abort
    ]

    args
    list-iterate [
        ; expect a value and give it to the arg
        2 lua-expect-values swap
        make-lua-assignment
        lua-emit-statement
    ]

    ["while " %c " do"] list-eval
    #f make-lua-expr
    lua-emit-statement

    stmts list-iterate [lua-emit-statement]

    args
    rets
    list-iterate [
        swap list-pop dig
        make-lua-assignment
        lua-emit-statement
    ]
    drop

    ["end"] #f make-lua-expr lua-emit-statement

    ; TODO wrong order here(?)
    args
    list-iterate []

]
export-name lua-while

; init limit step [body] lua-for-iter -> for v = init, limit, step do body end
; body = [v -> ]
; body must have arity -1, i.e. the smallest body is [drop]
; as loops must have 0 arity in total but v is put on the stack
define lua-for-iter [
    const %body

    3 lua-expect-values
    const %step
    const %limit
    const %init

    quote f lua-new-var const %forvar

    %body lua-eval-code const %bstate

    import data/map

    %bstate
    . args
    list-length const arglen
    list-reverse
    const args
    . returns
    list-length const retlen
    const rets
    . statements
    const stmts
    drop

    arglen retlen 1 add equal? if [drop drop] [
        ["for (iter) body input arity must be 1 more than output"] abort
    ]

    args
    ; the top arg is the iter
    list-reverse list-pop const %forvar-arg list-reverse
    list-iterate [
        ; expect a value and give it to the arg
        2 lua-expect-values swap
        make-lua-assignment
        lua-emit-statement
    ]

    ["for " %forvar " = " %init ", " %limit ", " %step " do"] list-eval
    #f make-lua-expr
    lua-emit-statement

    %forvar-arg %forvar make-lua-assignment lua-emit-statement

    stmts list-iterate [lua-emit-statement]

    args
    rets
    list-iterate [
        swap list-pop dig
        make-lua-assignment
        lua-emit-statement
    ]
    drop

    ["end"] #f make-lua-expr lua-emit-statement

    ; TODO wrong order here(?)
    rets
    list-iterate []
]
export-name lua-for-iter

; expr n [body] lua-for-in -> for v1, ..., vn in expr do body end
; body = [v1 ... vn -> ]
; body must have arity -n
; as loops must have 0 arity in total but all iter vars are put on the stack
define lua-for-in [
    const %body

    0 swap ascending? if [swap drop] [
        ["for (in) must declare at least one loop variable"] abort
    ]
    const %nvars

    1 lua-expect-values
    const %expr

    %body lua-eval-code const %bstate

    %nvars list-imake [ drop quote f lua-new-var ] const forvars

    import data/map

    %bstate
    . args
    list-length const arglen
    list-reverse
    const args
    . returns
    list-length const retlen
    const rets
    . statements
    const stmts
    drop

    arglen retlen %nvars add equal? if [drop drop] [
        arglen retlen interpreter-dump-stack
        ["for (in) body input arity must be n more than output"] abort
    ]

    args
    ; first assign forvars
    forvars
    list-map [ swap list-pop dig make-lua-assignment ]
    const forvar-assignments
    
    ; then expect more inputs
    list-iterate [
        ; expect a value and give it to the arg
        2 lua-expect-values swap
        make-lua-assignment
        lua-emit-statement
    ]

    ["for " forvars ", " list-join list-iterate [] " in " %expr " do"] list-eval
    #f make-lua-expr
    lua-emit-statement

    forvar-assignments list-iterate [lua-emit-statement]
    stmts list-iterate [lua-emit-statement]

    args
    rets
    list-iterate [
        swap list-pop dig
        make-lua-assignment
        lua-emit-statement
    ]
    drop

    ["end"] #f make-lua-expr lua-emit-statement

    ; TODO wrong order here(?)
    rets
    list-iterate []
]
export-name lua-for-in



; vi: ft=scheme

