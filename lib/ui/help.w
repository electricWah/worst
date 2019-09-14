
import syntax/attributes
import doc

define help [
    @[documentation [
        title "Show information on a topic"
        ; description "It's help."
        usage "help topic-name"
        example "help help"
        example "help tags"
        ; see-also help-tags
        section docs
        tags [help repl]
    ]]
    import list
    import doc
    import syntax/cond
    import syntax/variable

    "Help topic? (try: help)" upquote const topic drop

    define write-help [
        ansi [ doc-eval [
            define title [
                "\n" print
                bold topic ->string print
                reset " - " print
                upquote print
                "\n" print
            ]
            define usage [
                "Usage: " print
                yellow fg upquote print
                reset "\n" print
            ]
            define description [
                "\n  " print
                upquote print
                "\n\n" print
            ]
            define example [
                "Example: " print
                bright yellow fg upquote print
                reset "\n" print
            ]
            define see-also [
                "See also: " print
                bright cyan fg upquote ->string print
                reset "\n" print
            ]
            define section [ upquote drop ]
            define tags [
                upquote const taglist
                "Tags: " print taglist ->string print "\n" print
            ]
            define undocumented [ red fg "Undocumented.\n" print ]
            define internal [ "For internal use.\n" print ]
        ] ]
        "\n" print
    ]

    define show-tags [
        ansi [
            doc-tags hash-table-keys
            swap drop
            list-length ->string
            bold cyan fg print
            reset " available tags " print
            ->string green fg print
            reset ".\n" print
        ]
    ]
    define print-tag [
        const tag
        ansi [
            doc-tags tag hash-table-get
            dig drop
            list-length ->string
            bold cyan fg print
            reset " topics tagged " print
            bold tag ->string print " " print
            ->string reset green fg print
            reset ".\n" print
        ]
    ]

    ; turn this into cond for more special help words
    topic equals? tags if [
        drop show-tags
    ] [
        #f variable used
        has-documentation? if [ write-help used <- #t ] [ drop ]

        topic doc-tag? if [ print-tag used <- #t ] []

        drop
        used get if [
        ] [
            ansi [
                red fg "No such topic found.\n" print reset
            ]
        ]
    ]
]

export help

; vi: ft=scheme
