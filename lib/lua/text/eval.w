
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

    dict-empty
    quote args %args place-get list-reverse swap drop dict-set
    quote statements %statements place-get list-reverse swap drop dict-set
    quote returns rets dict-set
]
export-name lua-eval-code

; interp stack body lua-eval-interpreter -> body eval
; All args and returns use the given stack, which is isolated within body.
; You can reference expressions by name (const) or push/pop the given stack.
define lua-eval-interpreter [
    lua-eval-code

    quote statements dict-get swap drop const statements
    quote returns dict-get swap drop const rets
    quote args dict-get swap drop const args
    drop

    [
        quote interp_ const lua-eval-code:var-name
        1 lua-expect-values
    ] eval
    const interp

    ; args -> local arg = interp:stack_pop()
    args
    list-iterate [
        const arg
        ; obj name args retcount lua-method-call -> obj:name(args)
        interp
        quote stack_pop
        [] list-eval
        1 lua-method-call
        arg swap
        make-lua-assignment
        lua-emit-statement
    ]

    statements
    list-iterate [lua-emit-statement]

    ; returns -> interp:stack_push(ret)
    rets
    list-iterate [
        const r
        ; obj name args retcount lua-method-call -> obj:name(args)
        interp
        quote stack_push
        [r] list-eval
        0 lua-method-call
    ]
]
export-name lua-eval-interpreter

; For use within lua-eval
; body -> function(args) stmts; return ret, ... end
define lua-eval->function-expr [
    lua-eval-code
    quote statements dict-get swap drop const statements
    quote returns dict-get swap drop const rets
    quote args dict-get swap drop
    const args
    drop

    ["function (" args ", " list-join list-iterate [] ")"] list-eval
    0 make-lua-expr
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

; Writes given body into a "chunk" string
; body -> local args = ...; stmts; return ret, ...
define lua-eval->chunk-string [
    lua-eval-code
    quote statements dict-get swap drop const statements
    quote returns dict-get swap drop const rets
    quote args dict-get swap drop
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
]
export-name lua-eval->chunk-string

; Basically lua-eval->chunk-string -> lua-load-string
define lua-eval->lua-function [
    lua-eval->chunk-string
    lua-load-string if [] [
        ["lua-eval->lua-function failure"] swap list-push abort
    ]
]
export-name lua-eval->lua-function

; vi: ft=scheme

