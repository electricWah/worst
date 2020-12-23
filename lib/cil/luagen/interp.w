
; [ body ] eval-interpreter ->
; Creates and loads functions that can be defined
; It's a chunk with one argument, the interpreter
; every input is taken from the stack
; define interpreter-only things like cil/quote
; do stuff
; every input is put back on the stack
define cil/eval-interpreter->builtin [
    const %interp-body
    [
        [ cil/new-id-name interp cil/expect-value ] eval
        const %interp
        define cil/interp-stack-pop [
            [ %interp :* stack_pop 1 () ] cil/lua-expr
        ]
        define cil/interp-stack-push [
            cil/expect-value
            const v
            [ %interp :* stack_push 0 (v) ] cil/lua-expr
        ]
        define cil/interp-quote [
            [ %interp :* body_read 1 () ] cil/lua-expr
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
    cil/eval-program->string
    interpreter-dump-stack
    lua-load-string if [ ] [
        ("cil/eval-interpreter->builtin: could not compile")
        swap list-push
        abort
    ]
]
export-name cil/eval-interpreter->builtin

; vi: ft=scheme

