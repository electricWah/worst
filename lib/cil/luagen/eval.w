
; Evaluating chunks of code.

; Evaluating code results in its input variables, output values,
; and the list of statements it emitted.

define cil/eval+args [

    make-place const %eval-inputs
    const %eval-body

    modifying-body (0 make-place list-push)
    weakly define %cil/gensym []

    modifying-body (drop [quote arg])
    weakly define %cil/new-id-name []

    modifying-body ("    " list-push)
    weakly define %cil/indentation-value []

    modifying-body (0 make-place list-push)
    weakly define %cil/indentation []

    define cil/set-new-id-name [
        const %cil/new-id-name
        quote %cil/new-id-name definition-copy-up
    ]
    define cil/new-id-name [ upquote updo cil/set-new-id-name ]
    define cil/new-id [
        %cil/gensym place-get
        1 add const n
        n place-set drop
        %cil/new-id-name ->string n ->string
        string-append string->symbol
    ]

    define cil/indent> [ %cil/indentation place-get 1 add place-set drop ]
    define cil/indent< [ %cil/indentation place-get 1 negate add place-set drop ]

    define cil/do-indent [ cil/indent> upquote eval cil/indent< ]
    define cil/do-unindent [ cil/indent< upquote eval cil/indent> ]

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
        const r
        %cil/list-eval-stack place-get swap drop
        list-reverse list-iterate []
        r
    ]


    [] make-place const %args

    define cil/expect-value/generate [
        interpreter-stack-length equals? 0 swap drop if [
            %eval-inputs place-get equals? [] if [
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
        %cil/indentation place-get swap drop
        do-times [%cil/indentation-value list-push]
        const stmt
        %stmts place-get stmt list-push place-set drop
    ]

    define cil/comment [ const c [ "-- " c ] list-eval cil/emit-statement ]

    [] interpreter-stack-swap const %stack

    %eval-body eval

    %stack interpreter-stack-swap list-reverse
    %args place-get swap drop list-reverse
    %stmts place-get swap drop list-reverse
]
export-name cil/eval+args
define cil/eval [ [] cil/eval+args ]
export-name cil/eval

; vi: ft=scheme

