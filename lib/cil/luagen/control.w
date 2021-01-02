
; ... cond [ ift -> ... ] [ iff -> ... ] cil/lua-if-then-else -> ...
define cil/lua-if-then-else [
    const %iff
    const %ift

    import data/map

    ; eval condition and save output point for declaring inputs/outputs
    [ cil/new-id-name ifc cil/expect-value ] eval
    const %ifc

    cil/new-id-name ift
    %ift cil/eval
    const %tstmts
    list-length const %tilen
    const %tins
    list-length const %tolen
    const %touts


    cil/new-id-name iff
    %iff %tins cil/eval+args
    const %fstmts
    list-length const %filen
    const %fins
    list-length const %folen
    const %fouts

    %tilen negate %tolen add const tarity
    %filen negate %folen add const farity

    tarity farity equal? if [ drop drop ] [
        interpreter-dump-stack
        ("true and false arms have different arity") abort
    ]

    ; how many variables will be needed?
    %tilen %tolen %filen %folen max max max
    const arglen
    ; grab the max expected inputs
    %tilen %filen ascending? bury drop drop if [%fins] [%tins]
    const input-vars
    input-vars list-map [cil/expect-value/orvar]
    const input-values

    ; if there are more outputs than inputs, declare those
    %tilen %filen max negate arglen add 0 max
    list-imake [drop cil/new-id-name ifout cil/new-id #t cil/make-expr]
    const extra-outputs

    ; since extra-outputs are only in addition to inputs,
    ; append them to get the full list of used variables
    extra-outputs input-vars list-append const ifargs

    ; output generated code
    [] extra-outputs #t cil/emit-assignment

    ; remove 'local varN = varN'
    ; [a1 a2 ...] [b1 b2 ...] -> [[a1 b1] [a2 b2] ...] where aN != bN
    []
    input-values input-vars
    list-iterate [
        interpreter-dump-stack
        const var
        list-pop const val
        const vals
        var val equal? bury drop drop if [ ] [
            [var val] list-eval list-push
        ]
        vals
        interpreter-dump-stack
    ]
    drop

    ; [[a1 b1] ...] => [a1 ...] [b1 ...]
    [] [] dig
    list-iterate [
        list-pop const var
        list-pop const val
        drop
        swap val list-push
        swap var list-push
    ]

    #t cil/emit-assignment

    ["if " %ifc cil/expr->string " then"] list-eval cil/emit-statement

    cil/do-indent [
        %tstmts list-iterate [cil/emit-statement]
        ifargs %tolen list-split swap drop
        %touts swap
        #f cil/emit-assignment
    ]

    ; TODO don't emit else if false does nothing
    ["else"] cil/emit-statement

    cil/do-indent [
        %fstmts list-iterate [cil/emit-statement]
        ifargs %folen list-split swap drop
        %fouts swap
        #f cil/emit-assignment
    ]

    ["end"] cil/emit-statement

    ; leave all possible outputs on the stack

    ifargs
    %tolen %folen max
    list-split swap drop
    list-iterate []

]
export-name cil/lua-if-then-else

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

    %wbody cil/eval
    const wstmts
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


