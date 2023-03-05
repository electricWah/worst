
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
        quote eval uplevel
    ]

    quote new-def definition-resolve name
]
export dispatch

