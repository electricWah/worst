
; An interpreter

; TODO remove all #t/#f and just throw errors around instead

; Doesn't need to be a mutable vector with list-ref and list-set
define ctx-empty [
    import list
    list-quasiquote [
        ~[hash-table-empty] ^[[] [] #f]
    ]
]
export-name ctx-empty

define ctx-defs [ import list 0 list-ref! ]
export-name ctx-defs
define ctx-defs-set [ 0 swap list-set ]
export-name ctx-defs-set
define ctx-body [ import list 1 list-ref! ]
export-name ctx-body
define ctx-body-set [ 1 swap list-set ]
export-name ctx-body-set
define ctx-childs [ import list 2 list-ref! ]
export-name ctx-childs
define ctx-childs-set [ 2 swap list-set ]
export-name ctx-childs-set
define ctx-parent [ import list 3 list-ref! ]
export-name ctx-parent
define ctx-parent-set [ 3 swap list-set ]
export-name ctx-parent-set

; ctx thing ctx-resolve -> result #t
;                    or -> #f ; couldn't resolve
; e.g.
; 6 ctx-resolve if [ "got" ] [ "no 6" ]
define ctx-resolve [
    const name
    ctx-defs name hash-table-exists if [
        hash-table-get
        bury drop drop
        #t
    ] [
        drop drop
        ctx-parent false? if [ ] [
            name ctx-resolve dig drop
        ]
    ]
]
export-name ctx-resolve

; ctx val name -> ctx
define ctx-def-add [
    const name
    const val

    ctx-defs name val hash-table-set
    ctx-defs-set
]
export-name ctx-def-add

; stack ctx val -> stack ctx
; abort undefined if can't resolve it
define interp-eval [
    symbol? if [
        ctx-resolve if [
            define def [
                ctx-empty swap ctx-parent-set
                upquote ctx-body-set
            ]
            eval
        ] [ quote undefined abort ]
    ] [
        ; stack ctx val -> (val . stack) ctx
        bury swap dig list-push swap
    ]
]
export-name interp-eval

; context-next-code
; ctx -> ctx val #t
;     -> ctx #f
define ctx-body-read [
    ctx-body list-empty? if [ drop #f ] [
        list-pop bury ctx-body-set
        swap
        #t
    ]
]
export-name ctx-body-read

; ctx -> ctx
define ctx->child-innermost [
    while [
        ; ctx -> ctx child #t
        ;     -> ctx #f
        ctx-childs list-empty? if [ drop #f ] [
            ; ctx childs
            list-pop
            ; ctx childs child
            bury
            ; child ctx childs
            ctx-childs-set
            ; child ctx
            ctx-parent-set
            ; child
            #t
        ]
    ] [ ]
]
export-name ctx->child-innermost

; context-next
; ctx -> ctx val #t
;     -> ctx #f
define ctx-code-read [
    while [
        ctx->child-innermost
        ctx-body-read if [ #t #f ] [
            ctx-parent false? if [ #f ] [
                swap drop #t
            ]
        ]
    ] [ ]
]
export-name ctx-code-read

; ctx new-child -> ctx
define ctx-child-push [
    swap ctx-childs
    dig list-push
    ctx-childs-set
]
export-name ctx-child-push

; context-uplevel
; ctx -> ctx ok? ; should it error?
define ctx-into-parent [
    ctx-parent false? if [ ] [
        swap #f ctx-parent-set swap
        swap ctx-child-push
        #t
    ]
]
export-name ctx-into-parent

; stack ctx -> stack ctx
; will raise errors
define interp-run [
    while [ ctx-code-read if [ interp-eval #t ] [ #f ] ] [ ]
]
export-name interp-run

define interpreter [
    import syntax/variable
    import list
    import dict

    ; TODO set ctx and stack from config block

    dict %builtins

    define-attribute builtin [
        args (options)
        before [
            const name
            variable %body

            #f variable %simple
            define simple [ #t %simple set ]
            options eval

            %simple get if [
                [eval-simple-builtin]
                %body get
                list-push
                %body set
            ] [ ]

            name %body get
            %builtins set

            %body get
            name
        ]
    ]

    [] variable %body

    upquote
    [
        define body [ upquote eval %body set ]
        eval
    ] eval

    [] ; stack
    ctx-empty
    %body get ctx-body-set

    %builtins keys list-iterate [
        const name

        [] name list-push
        quote builtin list-push
        name
        ctx-def-add
    ]

    define eval-simple-builtin [
        const %body
        variable %context
        variable %stack
        define stack-pop [
            %stack get list-pop swap %stack set
            updo evaluate
            if [ ] [ quote wrong-type abort ]
        ]
        define stack-push [
            %stack get swap list-push %stack set
        ]
        %body eval
        %stack get
        %context get
    ]

    define builtin [ upquote %builtins get! eval ]

    interp-run
    drop
]
export-name interpreter

; vi: ft=scheme

