
define cil/lua-eval->chunk [
    [] cil/eval+args
    const stmts
    const args
    const outs

    args list-map [ 
    stmts
    list-map [ "" string-join ]
    "\n" string-join
]

export-all

; vi: ft=scheme


