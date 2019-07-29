
; doc
; - define docs
; - help command
; - doc to html
; - ensure everything has docs
; - iterate through docs and write to html

; doc-for def-name [
;   title "what this function does"
;   usage "usage statement"
;   description "why you would use this function"
;   example "an example (may be specified more than once)"
;   warn "gotchas"
;   ... any more...
; ]

hash-table-empty make-place const %documentation
export %documentation

define doc-for [
    %documentation #f place-swap
    upquote
    upquote
    hash-table-set
    place-swap drop drop
]

define help [
    "Help topic? (try: help)" upquote const topic
    drop
    %documentation place-get swap drop
    topic hash-table-exists if [
        hash-table-get
        rot rot drop
        swap const name
        define title [
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
            "  " print
            upquote print
            "\n\n" print
        ]
        define example [
            "Example: " print
            bright yellow fg upquote print
            reset "\n" print
        ]
        ansi [ "\n" print eval "\n" print ]
    ] [
        ansi [
            red fg "No such topic " print
            reset topic ->string print
            reset "\n" print
        ]
        drop drop
    ]
]
doc-for help [
    title "Show information on a topic"
    description "It's help."
    usage "help topic-name"
    example "help help"
    example "help topics"
]

export doc-for
export help

; vi: ft=scheme

