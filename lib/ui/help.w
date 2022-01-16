
import worst/doc/attribute
import ui/ansi

doc [
    title "Show information on how to use the Worst interactive environment."
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
    title "Show information on a definition or topic."
    see-also [help doc]
    example [info help]
    example [info tags]
]
define info [
    upquote const topic
    topic definition-resolve
    value-doc
    bury drop drop
    cond [
        (false?) [
            drop
            ansi [ bright red fg "No info available.\n" print reset ]
        ]
        (string?) [
            ansi [
                topic ->string yellow fg print
                reset ": " print
                cyan fg print
                reset "\n" print
            ]
        ]
        (list?) [
            list-iterate-pairs [ ansi [
                cyan fg
                ->string print
                reset
                "\t" print
                string? if [] [ ->string ] print
                "\n" print
            ] ]
        ]
    ]
]

export (help info)

; vi: ft=scheme

