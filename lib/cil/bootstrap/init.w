
define cil/eval-interpreter->builtin [
    cil/interpreter-eval
    cil/eval->string
    interpreter-dump-stack
    lua-load-string false? if [
        drop [] swap list-push abort
    ] [ ]
]
export-name cil/eval-interpreter->builtin

; vi: ft=scheme

