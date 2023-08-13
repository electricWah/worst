
define define [
    upquote <list> is-type if [ updo upquote ] [ [] swap ]
    const name
    const attrs

    upquote
    quote current-defs uplevel
    value-set-ambients
    const body
    
    ; eval attrs: body name -> body name
    body name
    [ #t const definition-attributes ] attrs list-append
    updo eval
    const name
    <symbol> type-id->unique name value-insert-meta-entry
    const body

    body name
    quote definition-add uplevel
]
export define

; true only within the attributes clause of a define form
define in-definition-attributes [ quote definition-attributes dynamic-resolve ]
export in-definition-attributes

define value-definition-add [
    const def const name
    clone <defset> type-id->unique value-meta-entry
    false? if [ drop defset-empty ] []
    name def defset-insert
    value-set-ambients
]
export value-definition-add

; define (recursive) infinite-loop [ infinite-loop ]
; TODO doesn't work, make it tail call without breaking upquote?
define recursive [
    const name
    const body

    define recursive-call [ name updo dynamic-resolve-any updo eval ]

    body
    name
    quote recursive-call definition-resolve
    value-definition-add

    name
]

; like while, no body, uplevels up the stack, maybe put this somewhere else?
define uplevel-while [
    updo current-defs const env
    upquote env value-set-ambients const cond
    ; upquote env value-set-ambients const body

    define the-whiler [
        const continuer
        cond uplevel ; const ok
        ; ok if [ body ] [ [] ] uplevel
        if [ continuer continuer ] [ [] ] updo uplevel
    ]
    quote the-whiler definition-resolve clone uplevel
]
export uplevel-while

; define (with-uplevel cool-uplevel) f [ ... cool-uplevel ... ]
; Add a definition available within the definition body to use uplevel
; as if at the top level of the definition, regardless of actual location.
; The uplevel-er will error if it is not invoked within the definition.
define with-uplevel [
    upquote const upleveler
    const name
    const body

    make-unique const body-toplevel-flag

    define uplevel-fn [
        uplevel-while [
            ; is-top = in correct stack frame
            body-toplevel-flag updo current-frame-meta-entry const is-top
            ; either do an uplevel or noop
            is-top if [ quote uplevel ] [ [] ]
            uplevel
            ; continue if not is-top
            is-top not
        ]
    ]

    body
    body-toplevel-flag #t value-insert-meta-entry
    upleveler quote uplevel-fn definition-resolve value-definition-add

    name
]
export with-uplevel

; using with-dynamics to do recursion means it might find the dynamic-call def
; within a nested call, so this flag is there to ignore those defs
make-unique const is-dynamic-call

; define (with-dynamics (a b)) def (... a ... b ...)
; in def, treat given definitions as dynamic
; useful for e.g. mutually recursive defs
define with-dynamics [
    const name
    const body

    upquote const dynamics

    make-unique const body-toplevel-flag

    body
    body-toplevel-flag #t value-insert-meta-entry
    dynamics list-iter [
        const def-name
        define dynamic-call [
            uplevel-while [
                ; if it's the toplevel thing
                body-toplevel-flag updo current-frame-meta-entry const is-top
                ; resolve in the frame above
                def-name quote definition-resolve updo uplevel
                const found-def
                is-top if [
                    found-def false? if [
                        def-name "can't find dynamic" error
                    ] [
                        ; but the resolved def isn't the dynamic call itself
                        is-dynamic-call value-meta-entry
                        ; if so, drop found-def and continue, else this is it
                        if [ #t ] [ found-def #f ]
                    ]
                ] [ #t ]
            ]
            uplevel
        ]
        def-name
        quote dynamic-call definition-resolve
        is-dynamic-call #t value-insert-meta-entry
        value-definition-add
    ]
    name
]
export with-dynamics

