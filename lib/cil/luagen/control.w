
; stack... [ stack... -> stack... bool ] cil/lua-loop -> stack...
define cil/lua-loop [
    const %wbody
    ; several emit contexts
    ; enter body emit
    ; eval wbody to get ins and outs
    ; if the arity is wrong, quit
    ; emit arg reassigns
    ; temporarily leave body emit
    ; emit arg defs
    ; emit "repeat"
    ; re-enter body emit and leave it
    ; emit until(!cont)

    ; local args = stack
    ; do {
    ;   do stuff with args
    ;   set args to new values
    ; } while (cont)
    ; leave args on stack

    cil/indent>
    cil/enter-new-emit-state

    %wbody cil/eval-fragment
    list-length const wilen
    const wargs
    list-length const wolen
    const woutputs

    wilen 1 add wolen equal? if [drop drop] [
        interpreter-dump-stack
        ["cil/loop: wrong arity"] abort
    ]

    woutputs
    list-reverse list-pop const wcont
    const woutputs

    []
    wargs list-iterate [
        drop const acc
        cil/expect-value
        acc swap list-push
    ]
    const winputs

    cil/do-unindent [
        cil/emit-state-do-uplevel [
            winputs wargs #t cil/emit-assignment
            ["repeat"] cil/emit-statement
        ]
    ]

    ; body emitted here
    ; emit reassignments
    woutputs wargs #f cil/emit-assignment

    cil/leave-emit-state
    cil/indent<

    [ wcont ! ] cil/lua-expr const wcont

    ["until (" wcont cil/expr->string ")"] list-eval cil/emit-statement

    wargs list-iterate []
]
export-name cil/lua-loop

; init limit step [ body : ... var -> ... ] cil/lua-for-iter =
; for var = init, limit, step do body end
define cil/lua-for-iter [
    const %fibody

    cil/expect-value const %fistep
    cil/expect-value const %filimit
    cil/expect-value const %fiinit

    ; enter emit state and eval chunk to get name of input var for body
    cil/indent>
    cil/enter-new-emit-state

    %fibody
    ; interpreter-dump-stack
    cil/eval-chunk
    list-length const ilen const args
    list-length const olen const outs

    ilen olen 1 add equal? if [drop drop] [
        interpreter-dump-stack
        ["cil/lua-for-iter: wrong arity"] abort
    ]

    args list-pop const carg
    const args

    cil/do-unindent [
        cil/emit-state-do-uplevel [
            define S [cil/expr->string]
            ["for " carg S " = " %fiinit S ", " %filimit S
                %fistep equals? 1 if [drop] [", " swap S]
                " do"]
            list-eval cil/emit-statement
        ]
    ]

    outs args #f cil/emit-assignment

    cil/indent<
    cil/leave-emit-state

    ["end"] cil/emit-statement


]
export-name cil/lua-for-iter

; vi: ft=scheme


