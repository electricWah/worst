
; Compile Worst to Lua.

; statement : expr-value
; expr : %expr + expr-value
; expr-value : string | expr list + prec | variable | literal

; kind make-lua-expr
; kind = #t : ->string as values
; kind = #f : precedence = 10
; kind = number : precedence ->
;   string: unprocessed
;   list: stringified with precedence
; TODO look through here for other expressions and increment used count
define make-lua-expr [
    map-empty
    quote %expr dig false? if [ drop 10 ] [] map-set
    quote value dig map-set
]
export-name make-lua-expr

define lua-expr? [ map? if [ quote %expr map-exists swap drop ] [ #f ] ]
export-name lua-expr?

; Assignments declare and/or set variables.
; They exist to reduce the amount of code generated,
; e.g. assigning constants to single-use variables,
; needlessly assigning variables to other variables,
; or assigning variables that are never used again.
; They may or may not produce a statement,
; and since they aren't evaluated until the last moment,
; they already know how the values will be used in the future.
; var val make-lua-assignment lua-emit-statement
define make-lua-assignment [
    const val
    const var

    var
    quote assign-count
    map-get

    false? if [ drop 0 ] [ ]
    const assign-count

    assign-count
    1 add
    map-set

    drop

    map-empty
    quote %assignment assign-count map-set
    quote var var map-set
    quote val val map-set
]
export-name make-lua-assignment

define lua-assignment? [
    map? if [ quote %assignment map-exists swap drop ] [ #f ]
]
export-name lua-assignment?

define lua-assignment->string [
    import data/map
    const stmt
    stmt . var swap drop
    . declared const declared
    .= declared #t
    drop
    [ declared if [] ["local "]
        stmt . var swap drop
        " = "
        stmt . val swap drop
    ] list-eval
    #f make-lua-expr
    lua-expr->string
]
export-name lua-assignment->string

define lua-expr->string [
    import syntax/case
    define value->string [
        case {
            (lua-expr?) { quote value map-get bury drop drop value->string }
            (list?) {
                list-map [value->string]
                ", " string-append
                "{" swap string-append
                "}" string-append
            }
            (map?) {
                map-keys list-map [
                    const k
                    k
                    map-get const v
                    symbol? if [ ->string ] [
                        value->string
                        "[" swap string-append
                        "]" string-append
                    ]
                    " = " string-append
                    v value->string
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
                quote value map-get bury drop drop
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
        (lua-expr?) { lua-expr->string }
        (lua-assignment?) { lua-assignment->string }
        #t { #f }
    }
]
export-name lua-statement->string

define lua-eval-code [
    const %body

    quote lua-eval-code:var-name
    definition-resolve false? if [ quote arg ] [ lua-eval-code:var-name ]
    const lua-eval-code:var-name
    drop drop

    quote %gensym
    definition-resolve false? if [ 0 make-place ] [ %gensym ]
    bury drop drop
    const %gensym

    [] make-place const %args
    [] make-place const %statements

    define lua-new-var [
        const name
        %gensym place-get
        1 add const id
        id place-set drop

        name ->string id ->string
        string-append string->symbol
        #t make-lua-expr
    ]

    define lua-expect-values [
        const n
        interpreter-stack-length n ascending? bury drop drop if [
            interpreter-stack list-reverse
            while (list-length n ascending? bury drop drop) {

                lua-eval-code:var-name
                lua-new-var const arg
                %args place-get
                arg list-push
                place-set
                drop
                arg
                
                list-push
            }
            list-reverse interpreter-stack-set
        ] []
    ]

    define lua-emit-statement [
        const stmt
        %statements place-get
        stmt list-push
        place-set drop
    ]

    [] interpreter-stack-swap const %stack
    %body eval
    %stack
    interpreter-stack-swap
    list-reverse
    list-map [ lua-expr? if [] [ #t make-lua-expr ] ]
    const rets

    map-empty
    quote args %args place-get list-reverse swap drop map-set
    quote statements %statements place-get list-reverse swap drop map-set
    quote returns rets map-set
]
export-name lua-eval-code

define lua-eval->definition [
    import list
    lua-eval-code const bodystate
    
    quote interp_ const lua-eval-code:var-name
    [
        2 lua-expect-values
        const interp
        const stack
        
        bodystate
        quote args map-get swap drop
        list-iterate [
            const arg
            ; obj name args retcount lua-method-call -> obj:name(args)
            interp
            quote stack_pop
            [stack] list-eval
            1 lua-method-call
            arg swap
            make-lua-assignment
            lua-emit-statement
        ]

        quote statements map-get swap drop
        list-iterate [lua-emit-statement]

        quote returns map-get swap drop
        list-iterate [
            const r
            ; obj name args retcount lua-method-call -> obj:name(args)
            interp
            quote stack_push
            [stack r] list-eval
            0 lua-method-call
        ]
    ] lua-eval-code

    quote args map-get swap drop
    list-map [ quote declared #t map-set ]
    const args
    [ "local " args list-iterate [", "] drop " = ..." ] list-eval
    #f make-lua-expr
    const args

    quote statements map-get swap drop
    args list-push
    list-choose [lua-statement->string]
    "\n" string-join

    swap drop
    interpreter-dump-stack
    lua-load-string if [] [
        ["lua-eval->definition failure"] swap list-push
    ]

]
export-name lua-eval->definition

; lhs rhs op prec lua-binop -> expr "lhs op rhs"
define lua-binop [
    const prec
    const op
    2 lua-expect-values
    const rhs
    const lhs

    [ lhs op rhs ] list-eval
    prec make-lua-expr
]
export-name lua-binop

; a b lua-dot -> a.b
define lua-dot [ quote . 0 lua-binop ]
export-name lua-dot

; val op prec lua-unop -> expr "op val"
define lua-unop [
    const prec
    const op
    1 lua-expect-values
    const val

    [ op val ] list-eval
    prec make-lua-expr
]
export-name lua-unop

; obj key lua-index -> expr "obj[key]"
define lua-index [
    2 lua-expect-values
    const key
    const obj
    [ obj "[" key "]" ] list-eval
    9 make-lua-expr
]
export-name lua-index

; [v0 v1 ...] lua-declare-vars -> local v0, v1, ...
define lua-declare-vars [
    list-length 0 equal? not bury drop drop if [
        list-map [ quote declared #t map-set ]
        const vars
        ["local " vars list-iterate [", "] drop ] list-eval
        #f make-lua-expr
        lua-emit-statement
    ] [drop]
]
export-name lua-declare-vars

; expr args retcount lua-funcall -> local r1, r2, ... = expr(args)
define lua-funcall [
    import list
    const rcount
    const args
    const func

    [ func "(" args list-iterate [", "] drop ")" ] list-eval
    #f make-lua-expr
    const fcall
    
    ; multival assign
    rcount 0 equal? bury drop drop if [
        ; Don't do any assigning
        fcall lua-emit-statement
    ] [
        ; Make enough return values
        rcount list-imake [
            drop
            quote v lua-new-var
            quote declared #t map-set
        ]
        ; interpreter-dump-stack
        const rets

        ; Emit the returns assignment as a statement
        [ "local " rets list-iterate [","] drop " = " fcall ] list-eval
        #f make-lua-expr
        lua-emit-statement

        ; Drop the values on the stack
        rets list-iterate []
    ]
]
export-name lua-funcall

; obj name args retcount lua-method-call -> obj:name(args)
define lua-method-call [
    const rcount
    const args
    quote : 0 lua-binop
    args rcount lua-funcall
]
export-name lua-method-call

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


; vi: ft=scheme


