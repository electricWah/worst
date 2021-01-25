
import cil/bootstrap/base

[ cil/interp-quote ]
cil/eval-interpreter->builtin
quote quote
definition-add

[
    cil/interp-into-parent drop
    cil/interp-quote
]
cil/eval-interpreter->builtin
quote upquote
definition-add

[
    cil/interp-quote const c
    cil/interp-into-parent drop
    ; [] [ "root-uplevel" cil/interp-error ] cil/lua-if-then-else
    c cil/interp-call
]
cil/eval-interpreter->builtin
quote updo
definition-add

[
    cil/expect-value const val
    lua-interp-definition const [ val ]
    cil/interp-quote
    cil/interp-define
]
cil/eval-interpreter->builtin
quote const
definition-add

[
    cil/interp-quote const ift
    cil/interp-quote const iff
    [ ift cil/interp-eval ] [ iff cil/interp-eval ] cil/lua-if-then-else
]
cil/eval-interpreter->builtin
quote if
definition-add

export-all


; vi: ft=scheme

