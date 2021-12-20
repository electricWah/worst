
import worst/doc/attribute
import ui/ansi

doc [
    title "Show information on how to use the Worst interactive environment."
    ; description "It's help."
    usage "help topic-name"
    example "help help"
    example "help tags"
    ; see-also help-tags
    section docs
    tags [help repl]
]

define help [
    ansi [
        define $ [ upquote print ]
        define info [ yellow fg $"info" reset ]
        define ex [ green fg updo $ reset ]

        $"Some useful commands:\n"
        info $": try " info ex " name" $" for information about " ex "name"
        $"\n"
    ]
]

doc [
    title "Show information on a topic"
]
define info [
    upquote const topic
    topic definition-resolve
    value-doc
    bury drop drop
    false? if [
        drop
        ansi [ bright red fg "No info available.\n" print reset ]
    ] [
        ansi [ bright magenta fg "Sorry, no nice formatting for now!\n" print reset ]
        ->string print "\n" print
    ]
]

; define helpo [
;     "Help topic? (try: help)" upquote const topic drop

;     define write-help [
;         ansi [
;             doc-eval [
;                 title [
;                     const v
;                     "\n" print
;                     bold topic ->string print
;                     reset " - " print
;                     v print
;                     "\n" print
;                 ]
;                 usage [
;                     const v
;                     "Usage: " print
;                     yellow fg v print
;                     reset "\n" print
;                 ]
;                 description [
;                     const v
;                     "\n  " print
;                     v print
;                     "\n\n" print
;                 ]
;                 example [
;                     "Example: " print
;                     bright yellow fg
;                     string? if [ print ] [
;                         list-iterate [ "\n    " print print ]
;                     ]
;                     reset "\n" print
;                 ]
;                 see-also [
;                     const v
;                     "See also: " print
;                     bright cyan fg v ->string print
;                     reset "\n" print
;                 ]
;                 tags [
;                     const taglist
;                     "Tags: " print taglist ->string print "\n" print
;                 ]
;                 undocumented [ drop red fg "Undocumented.\n" print ]
;                 internal [ drop "For internal use.\n" print ]
;             ]
;             if [] []
;         ]
;         "\n" print
;     ]

;     define show-tags [
;         ansi [
;             doc-tags dict-keys
;             swap drop
;             list-length ->string
;             bold cyan fg print
;             reset " available tags " print
;             ->string green fg print
;             reset ".\n" print
;         ]
;     ]
;     define print-tag [
;         const tag
;         ansi [
;             doc-tags tag dict-get
;             dig drop
;             list-length ->string
;             bold cyan fg print
;             reset " topics tagged " print
;             bold tag ->string print " " print
;             ->string reset green fg print
;             reset ".\n" print
;         ]
;     ]

;     ; turn this into cond for more special help words
;     topic equals? tags if [
;         drop show-tags
;     ] [
;         import syntax/variable
;         #f make-place const used
;         has-documentation? if [
;             write-help
;             used #t place-set drop
;         ] [ drop ]

;         topic doc-tag? if [
;             print-tag
;             used #t place-set drop
;         ] []

;         drop
;         used place-get swap drop if [
;         ] [
;             ansi [
;                 red fg "No such topic found.\n" print reset
;             ]
;         ]
;     ]
; ]

export (help info)

; vi: ft=scheme

