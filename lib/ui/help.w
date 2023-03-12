
; import worst/doc/attribute
import {
    ui/ansi
    data/pairs
    syntax/case
}

doc [
    title "Show information on how to use the Worst interactive environment."
    tags [help repl]
]

define help [
    ansi [
        define $ [ upquote print ]
        define ^ [ yellow fg upquote value->string print reset ]
        define ex [ green fg updo $ reset ]

        $"Some useful commands:\n"
        ; ^ tutorial $": an interactive introduction to Worst\n"
        ^ info $": try " ^ info ex " name" $" for information about " ex "name\n"
    ]
]

doc [
    title "Show information on a definition or topic."
    see-also [help]
    example [info info]
    example [info help]
]
define info [
    upquote const topic
    topic updo definition-resolve
    value-doc
    case [
        (false?) [
            drop ansi [ bright red fg "No info available.\n" print reset ]
        ]
        (string?) [
            ansi [
                topic value->string yellow fg print
                reset ": " print
                cyan fg print
                reset "\n" print
            ]
        ]
        (list?) [
            ; toggle: #t = key, #f = value
            #t swap
            list-iter [ ansi [
                swap if [
                    cyan fg print-value
                    reset "\t" print
                    #f
                ] [
                    println-value
                    #t
                ]
            ] ]
            drop
        ]
    ]
]

export (help info)

