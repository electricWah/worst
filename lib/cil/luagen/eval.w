
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
    
    %eval-body eval
]
export-name cil/eval-program

; [ body ... ] [ inputs ] cil/eval-fragment+args -> outputs inputs
; [ body ... ] cil/eval-fragment -> outputs inputs
; Only call this within eval-program.
define cil/eval-fragment+args [
    make-place const %inputs
    const %body

    [] make-place const %args

    define cil/expect-value/generate [
        interpreter-stack-length equals? 0 swap drop if [
            %inputs place-get equals? [] if [
                drop drop
                cil/expect-value-generator
            ] [
                list-pop bury place-set drop
            ]
            const v
            %args place-get v list-push place-set drop
            v
        ] []
    ]

    define cil/expect-value/orvar [
        const %expectvar
        define cil/expect-value-generator [%expectvar]
        cil/expect-value/generate
    ]

    define cil/expect-value [
        define cil/expect-value-generator [cil/new-id #t cil/make-expr]
        cil/expect-value/generate
    ]

    define cil/expect-values [
        list-imake [drop cil/expect-value] list-iterate []
    ]

    [] make-place const %stmts

    define cil/emit-statement [
        cil/indentation place-get swap drop
        list-imake [drop %cil/indentation-value]
        swap list-append
        const stmt
        %stmts place-get stmt list-push place-set drop
    ]

    [] interpreter-stack-swap const %stack
    %body eval
    %stack interpreter-stack-swap
    list-reverse
    const rets

    rets
    %args place-get swap drop list-reverse
    %stmts place-get swap drop list-reverse
]
export-name cil/eval-fragment+args
define cil/eval-fragment [ [] cil/eval-fragment+args ]
export-name cil/eval-fragment

; vi: ft=scheme

