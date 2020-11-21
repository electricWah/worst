
; Evaluating chunks of code.

; [ code ... ] cil/eval-program
; Don't try to call this inside itself or combine two textual chunks in any way
; as it sets up indentation, gensym, etc. and there will be problems.
define cil/eval-program [
    const %eval-body
    0 make-place const cil/gensym-place
    define %cil/new-id-name [ quote arg ]
    define cil/new-id [
        cil/gensym-place place-get
        1 add const n
        n place-set drop
        %cil/new-id-name ->string n ->string
        string-append string->symbol
    ]
    define cil/new-id-name [
        [quote] upquote list-push list-reverse
        quote %cil/new-id-name
        updo definition-add
    ]
    
    0 make-place const cil/indentation
    define cil/indent> [ cil/indentation place-get 1 add place-set drop ]
    define cil/indent< [ cil/indentation place-get 1 negate add place-set drop ]

    define cil/do-indent [ cil/indent> upquote eval cil/indent< ]
    define cil/do-unindent [ cil/indent< upquote eval cil/indent> ]

    define %cil/indentation-value ["    "]

    ; Emit levels mirror the interpreter eval stack
    ; - emitted statements usually go in the current level
    ; - you can go up levels in order to emit before the current level is done
    ; - ending a level re-emits everything into the parent
    ; this is so compilation can emit prerequisites before the current level
    ; e.g. inputs to an if/else statement,
    ; or a function call to something that hasn't been defined yet
    cil/make-emit-state make-place const %cil/emit-state

    ; Extra bits for the emit state
    define cil/emit-statement [
        const stmt
        %cil/emit-state place-get

        cil/indentation place-get swap drop
        list-imake [drop %cil/indentation-value]

        stmt
        list-append
        cil/emit-state-emit-statement
        place-set drop
    ]

    ; enter a fresh state
    define cil/enter-new-emit-state [
        %cil/emit-state place-get
        cil/emit-state-enter-child
        place-set drop
    ]

    ; leave the current state and return the statements
    define cil/leave-emit-state [
        %cil/emit-state place-get
        cil/emit-state-leave-child
        place-set drop
    ]

    define cil/do-emit-state [
        cil/enter-new-emit-state
        upquote eval
        cil/leave-emit-state
    ]

    ; cil/emit-state-do-uplevel [ body ]
    ; do body in the context of the parent emit state
    define cil/emit-state-do-uplevel [
        upquote const %esubody
        %cil/emit-state place-get
        cil/emit-state-parent
        dig swap place-set drop

        #f cil/emit-state-set-parent
        const %uplevelstate

        %esubody eval

        %cil/emit-state place-get
        %uplevelstate swap cil/emit-state-set-parent
        place-set drop
    ]
    
    %eval-body eval

    %cil/emit-state place-get
    cil/emit-state-statements
    bury drop drop
]
export-name cil/eval-program

; [ body ... ] cil/eval-fragment -> outputs inputs
; Only call this within eval-program.
define cil/eval-fragment [
    const %body

    [] make-place const %args

    define cil/expect-value [
        interpreter-stack-length equals? 0 swap drop if [
            cil/new-id #t cil/make-expr
            const v
            %args place-get v list-push place-set drop
            v
        ] []
    ]

    define cil/expect-values [
        list-imake [drop cil/expect-value] list-iterate []
    ]

    ; list-eval but reading from the stack first
    define cil/list-eval [
        const %cil/list-eval-body
        [] interpreter-stack-swap
        make-place const %cil/list-eval-stack
        lexical (cil/expect-value)
        define cil/expect-value [
            interpreter-stack-length equals? 0 swap drop if [
                %cil/list-eval-stack place-get
                list-empty? if [ drop drop cil/expect-value ] [
                    list-pop bury
                    place-set drop
                ]
            ] []
        ]
        %cil/list-eval-body list-eval
        list-reverse const r
        %cil/list-eval-stack place-get swap drop
        list-reverse list-iterate []
        r
    ]

    [] interpreter-stack-swap const %stack
    %body eval
    %stack interpreter-stack-swap
    list-reverse
    const rets

    rets
    %args place-get swap drop list-reverse
]
export-name cil/eval-fragment

; vi: ft=scheme

