
; [ body ] eval-interpreter ->
; Creates and loads functions that can be defined
; It's a chunk with one argument, the interpreter
; every input is taken from the stack
; define interpreter-only things like cil/quote
; do stuff
; every input is put back on the stack

define cil/interpreter-eval [
    const %interp-body
    [
        import cil/luagen/lualib
        [ cil/new-id-name interp cil/expect-value ] eval
        const %interp
        define cil/interp-stack-pop [
            [ %interp :* stack_pop 1 () ] cil/lua-expr
        ]
        define cil/interp-stack-ref [
            cil/expect-value const idx
            [ %interp :* stack_ref 1 (idx) ] cil/lua-expr
        ]
        define cil/interp-stack-ref/type [
            cil/expect-value const t
            cil/expect-value const idx
            [ %interp :* stack_ref 1 (idx t) ] cil/lua-expr
        ]
        define cil/interp-stack-push [
            cil/expect-value const v
            [ %interp :* stack_push 0 (v) ] cil/lua-expr
        ]
        define cil/interp-quote [
            [ %interp :* body_read 1 () ] cil/lua-expr
        ]

        define cil/interp-into-parent [
            [ %interp :* into_parent 1 () ] cil/lua-expr
        ]

        define cil/interp-error [
            cil/expect-value const v
            [ %interp :* error 0 (v) ] cil/lua-expr
        ]

        define cil/interp-eval [
            cil/expect-value const v
            [ %interp :* eval 0 (v) ] cil/lua-expr
        ]
        define cil/interp-call [
            cil/expect-value const v
            [ %interp :* call 0 (v) ] cil/lua-expr
        ]

        define cil/interp-string->symbol [
            cil/expect-value const name
            [ "base" lua-require -> Symbol -> new .* #t (name) ] cil/lua-expr
        ]

        define cil/interp-define [
            cil/expect-value
            string? if [ cil/interp-string->symbol ] [ ]
            const name
            cil/expect-value const def
            [ %interp :* define 0 (name def) ] cil/lua-expr
        ]
        define cil/interp-resolve [
            cil/expect-value const name
            [ %interp :* resolve 1 (name) ] cil/lua-expr
        ]

        lexical (cil/expect-value)
        define cil/expect-value [
            ; interpreter-call-stack interpreter-dump-stack drop
            interpreter-stack-length equals? 0 swap drop if [
                cil/interp-stack-pop
            ] []
        ]
        %interp-body eval
        interpreter-stack list-iterate [ cil/interp-stack-push ]
        [] interpreter-stack-set
    ]
    cil/eval
]
export-name cil/interpreter-eval

; [ code that uses cil/interp-* ] cil/interpreter-function
; => a function that can be used as a builtin
; define cil/interpreter-function [
;     cil/interpreter-fragment
; ]
; export-name cil/interpreter-function

define cil/eval-interpreter->builtin [
    cil/interpreter-eval
    cil/eval->string
    interpreter-dump-stack
    lua-load-string if [ ] [
        ("cil/eval-interpreter->builtin: could not compile")
        swap list-push
        abort
    ]
]
export-name cil/eval-interpreter->builtin

; vi: ft=scheme

