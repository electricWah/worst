
; define (dispatch (cond: -> bool)) name [ body ]
; => define name [ cond if [ body ] [ previous definition for name ] ]
define dispatch [
    upquote
    quote current-defenv uplevel
    defenv-new-locals
    value-set-defenv
    const dispatch-case

    const name
    const body
    name updo definition-resolve const prev
    prev not not const any-prev

    ; new-def checks dispatch-case and picks body or prev to eval at same level
    define new-def [
        dispatch-case eval if [ body ] [
            any-prev if [ prev ] [
                (no-matching-dispatch)
                name list-push
                list-reverse
                error
            ]
        ]
        uplevel
    ]

    quote new-def definition-resolve name
]

; meta key on definitions for type lookup
make-unique const type-dispatch-key

; type-dispatch on top item (for now):
; define (<string> type-dispatch) thingy [ ... ]
; should be faster than normal dispatch because it uses an lookup of type-id
; (type predicates are defined to have their type-id in meta)
define type-dispatch [
    const dispatch-type-id
    const name const body

    ; get or default map in existing def
    name updo definition-resolve const prev-def
    prev-def type-dispatch-key value-meta-entry
    false? const fresh-dispatch
    fresh-dispatch if [ drop make-lookup ] [ ]
    ; add current body to lookup
    ; TODO check it's not already in there
    dispatch-type-id type-hash body
    lookup-insert
    ; if fresh-dispatch, set prev-def (or error) as the default case
    fresh-dispatch if [
        prev-def false? if [
            drop
            define error-case [ "no type-dispatch or default:" name error ]
            quote error-case definition-resolve
        ] []
        ; add it as 0 (default) in the lookup
        0 swap lookup-insert
    ] [] ; <- it will have already been set, hopefully
    const dispatch-lookup

    define new-def [
        ; TODO get current frame meta, don't use new body if not fresh-dispatch
        ; but for now use dispatch-lookup
        const v
        dispatch-lookup v value-type type-hash lookup-get
        false? if [ drop dispatch-lookup 0 lookup-get ] []
        v swap uplevel
    ]

    quote new-def definition-resolve
    type-dispatch-key dispatch-lookup value-insert-meta-entry
    name
]

