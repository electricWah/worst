
define define [
    upquote <list> is-type if [ updo upquote ] [ [] swap ]
    const name
    const attrs

    upquote
    quote current-defenv uplevel
    defenv-new-locals
    value-set-defenv
    const body
    
    ; eval attrs: body name -> body name
    body name
    [ #t const definition-attributes ] attrs list-append
    updo eval
    const name
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
    clone value-defenv
    name def defenv-insert-local
    defenv-new-locals
    value-set-defenv
]

; define (recursive) infinite-loop [ infinite-loop ]
; attribute: define self within body to enable recursive calls
; works funny when there's an existing definition with the same name!
; TODO probably doesn't work
define recursive [
    const name
    const body

    define recursive-call [ name updo dynamic-resolve-any updo eval ]

    body
    name
    quote recursive-call definition-resolve
    value-set-not-dynamic-resolvable ; lol
    value-definition-add

    name
]

; define (with-dynamics (a b)) def (... a ... b ...)
; in def, treat given definitions as dynamic
; useful for e.g. mutually recursive defs
define with-dynamics [
    const name
    const body

    upquote const dynamics

    body
    dynamics list-iter [
        const def
        define dynamic-call [
            def dynamic-resolve-any
            quote eval uplevel
        ]
        def quote dynamic-call definition-resolve
        value-set-not-dynamic-resolvable ; lol
        value-definition-add
    ]

    name
]
export with-dynamics

