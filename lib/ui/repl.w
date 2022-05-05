
import {
    worst/interpreter
    syntax/cond
    data/string
    data/pairs
}

define worst-repl [

    define clear-stack [ while [stack-empty not] [drop] ]

    interpreter-empty
    interpreter-inherit-definitions
    const interp

    reader-empty const reader

    ansi [
        "Welcome to the Worst interactive environment. Type " print
        bright yellow fg "help" print
        reset " for assistance.\n" print
    ]

    define standard-prompt [
        interp interpreter-stack-get const stack drop
        ansi [
            green fg
            "worst " print

            reset cyan fg
            stack list-reverse
            current-output-port swap
            port-write-value drop

            bold yellow fg
            " > " print
            reset
        ]
    ]

    define continuation-prompt [
        ansi [ cyan fg "... " print yellow fg "> " print reset ]
    ]

    define exit-message [
        ; ansi [ erase-line green fg "\r:)\n" print reset ]
        "\n" print
    ]

    while [
        standard-prompt
        current-input-port port-read-line swap drop
        equals? "" if [ drop exit-message #f ] [
            reader swap reader-write-string drop
            []
            while [reader reader-next dig drop] [ list-push ]
            false? if [drop] [ "read error" stack-dump error ]
            list-reverse
            interp swap interpreter-body-prepend
            interpreter-run swap drop if [
            ] [
                interp interpreter-stack-pop false? if [ drop ] [
                    equals? quote-nothing if [
                        ; somehow toggle prompt?
                        drop
                    ] [
                        ansi [ bright red fg value->string print reset ]
                        "\n" print
                        interpreter-reset drop
                    ]
                ]
            ]
            #t
        ]
    ] []

]

export worst-repl

; vi: ft=scheme

