
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
    import list
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
    import list
    import syntax/case
    define value->string [
        case {
            (lua-expr?) {
                quote value map-get bury drop drop value->string
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
        (false?) { }
        (lua-expr?) { lua-expr->string }
        (lua-assignment?) { lua-assignment->string }
        #t { ["lua-statement->string unexpected"] swap list-append abort }
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

; interp stack body lua-eval-interpreter -> body eval
; All args and returns use the given stack, which is isolated within body.
; You can reference expressions by name (const) or push/pop the given stack.
define lua-eval-interpreter [
    import list
    
    lua-eval-code

    quote statements map-get swap drop const statements
    quote returns map-get swap drop const rets
    quote args map-get swap drop const args
    drop

    [
        quote interp_ const lua-eval-code:var-name
        2 lua-expect-values
    ] eval
    const interp
    const istack

    ; args -> local arg = interp:stack_pop(istack)
    args
    list-iterate [
        const arg
        ; obj name args retcount lua-method-call -> obj:name(args)
        interp
        quote stack_pop
        [istack] list-eval
        1 lua-method-call
        arg swap
        make-lua-assignment
        lua-emit-statement
    ]

    statements
    list-iterate [lua-emit-statement]

    ; returns -> interp:stack_push(istack, ret)
    rets
    list-iterate [
        const r
        ; obj name args retcount lua-method-call -> obj:name(args)
        interp
        quote stack_push
        [istack r] list-eval
        0 lua-method-call
    ]
]
export-name lua-eval-interpreter

; For use within lua-eval
; body -> function(args) stmts; return ret, ... end
define lua-eval->function-expr [
    import list

    lua-eval-code
    quote statements map-get swap drop const statements
    quote returns map-get swap drop const rets
    quote args map-get swap drop
    list-map [ quote declared #t map-set ]
    const args
    drop

    ["function (" args ", " list-join list-iterate [] ")"] list-eval
    #f make-lua-expr
    const argstmt

    rets list-empty? swap drop if [#f] [
        ["return " rets ", " list-join list-iterate []] list-eval
        #f make-lua-expr
    ]
    const retstmt

    statements
    argstmt list-push
    list-reverse
    retstmt list-push
    ["end"] #f make-lua-expr list-push
    list-reverse

    list-choose [lua-statement->string]

    "\n" string-join
    [] swap list-push
    #f make-lua-expr

]
export-name lua-eval->function-expr

; Compiles eval into a lua function ("chunk")
; body -> local args = ...; stmts; return ret, ... -> lua-load-string
define lua-eval->lua-function [
    import list

    lua-eval-code
    quote statements map-get swap drop const statements
    quote returns map-get swap drop const rets
    quote args map-get swap drop
    list-map [ quote declared #t map-set ]
    const args
    drop

    args list-empty? swap drop if [#f] [
        [ "local " args ", " list-join list-iterate [] " = ..." ] list-eval
        #f make-lua-expr
    ]
    const argstmt

    rets list-empty? swap drop if [#f] [
        [ "return " rets ", " list-join list-iterate [] ] list-eval
        #f make-lua-expr
    ]
    const retstmt

    statements
    argstmt list-push
    list-reverse retstmt list-push list-reverse

    list-choose [lua-statement->string]
    "\n" string-join

    interpreter-dump-stack
    lua-load-string if [] [
        ["lua-eval->lua-function failure"] swap list-push abort
    ]
]
export-name lua-eval->lua-function


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
    0 make-lua-expr
]
export-name lua-index

; [v0 v1 ...] lua-declare-vars -> local v0, v1, ...
define lua-declare-vars [
    list-length 0 equal? not bury drop drop if [
        list-map [ quote declared #t map-set ]
        const vars
        ["local " vars ", " list-join list-iterate []] list-eval
        #f make-lua-expr
        lua-emit-statement
    ] [drop]
]
export-name lua-declare-vars


; expr name lua-expr->variable -> local name = expr; name
define lua-expr->variable [
    const name
    1 lua-expect-values const expr
    name lua-new-var
    quote declared #t map-set
    const var
    var expr make-lua-assignment lua-emit-statement
    var
]
export-name lua-expr->variable

; multivalue, impure-safe version
; expr args retcount lua-funcall -> local r1, r2, ... = expr(args); r1, r2, ...
; single-value version
; expr args #t lua-funcall -> expr(args)
define lua-funcall [
    import list
    const rcount
    const args
    const func

    [ func "(" args ", " list-join list-iterate [] ")" ] list-eval
    0 make-lua-expr
    const fcall

    ; pure single expression
    rcount #t equal? bury drop drop if [
        fcall
    ] [
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
            [ "local " rets ", " list-join list-iterate [] " = " fcall ]
            list-eval
            #f make-lua-expr
            lua-emit-statement

            ; Drop the values on the stack
            rets list-iterate []
        ]
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

define lua-nil [ quote nil #t make-lua-expr ]
export-name lua-nil

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

; Redefining things in terms of themselves

; func in-n lua-function-wrap-definition -> def
; stack_push_all(func(unpack(stack_pop(in_n))))
[
    [
        2 lua-expect-values
        const arglen
        const func
        [
            ; lua-eval-interpreter can't do dynamic stack lengths
            2 lua-expect-values
            const istack
            const interp

            ; interp:stack_pop(istack)
            define interp-stack-pop [
                interp
                quote stack_pop
                [istack] list-eval
                1
                lua-method-call
            ]

            ; local args = {}
            [] quote args lua-expr->variable const args

            ; for i = 0, arglen, 1 do
            0 arglen 1 [
                1 lua-expect-values drop ; drop i

                ; table.insert(args, interp:stack_pop()) end

                quote table.insert
                [args interp-stack-pop] list-eval
                0
                lua-funcall
            ]
            lua-for-iter

            ; local rets = {func(unpack(args))}
            [
                ; func(unpack(args))
                func
                [
                    ; unpack(args)
                    quote unpack [args] list-eval
                    #t lua-funcall
                ]
                list-eval
                #t lua-funcall
            ]
            list-eval
            quote rets lua-expr->variable
            const rets

            ; for i, r in ipairs(rets) do interp:stack_push(r) end
            quote ipairs [rets] list-eval #t lua-funcall
            2
            [
                2 lua-expect-values
                const i
                const ret

                interp
                quote stack_push
                [istack ret] list-eval
                0 lua-method-call
            ]
            lua-for-in

            ; TODO

        ]
        lua-eval->function-expr
    ]
    lua-eval-interpreter
]
lua-eval->lua-function
quote lua-function-wrap-definition
definition-add
export-name lua-function-wrap-definition

; define lua-expr? [ map? if [ quote %expr map-exists swap drop ] [ #f ] ]
; quote %expr must be injected because lua doesn't have access to symbols
; (technically it does, but only through require(), which may fail at runtime
;  if this is accidentally used outside of jit - so don't let it happen)
; [
;     [
;         ; quote %expr
;         1 lua-expect-values const expr-symbol
;         [
;             [
;                 ; return pop()[quote %expr] ~= nil
;                 1 lua-expect-values
;                 clone ; keep it on stack
;                 expr-symbol lua-index
;                 lua-nil
;                 quote ~= 6 lua-binop
;             ]
;             lua-eval-interpreter
;         ]
;         lua-eval->function-expr
;     ]
;     lua-eval-interpreter
; ]
; lua-eval->lua-function
; quote %expr swap eval
; quote lua-expr? definition-add
; export-name lua-expr?

; vi: ft=scheme


