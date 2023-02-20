
import ui/ansi

; maybe make this dynamic?
define standard-worst-prompt [
    interpreter-stack-get const stack drop
    ansi [
        green fg
        "worst " print

        reset cyan fg
        stack list-reverse
        print-value

        bold yellow fg
        " > " print
        reset
    ]
]
export standard-worst-prompt

define worst-repl [

    define clear-stack [ while [stack-empty not] [drop] ]

    interpreter-empty
    do [
        import ui/help
        current-defenv interpreter-defenv-set
    ]
    ; quote pause interpreter-definition-remove ; this breaks it ; please don't try pause
    const interp

    reader-empty make-place const reader

    ansi [
        "Welcome to the Worst interactive environment. Type " print
        bright yellow fg "help" print
        reset " for assistance.\n" print
    ]

    define continuation-prompt [
        ansi [ cyan fg "... " print yellow fg "> " print reset ]
    ]

    define exit-message [
        ; ansi [ erase-line green fg "\r:)\n" print reset ]
        "\n" print
    ]

    while [
        interp standard-worst-prompt
        read-line
        equals? "" if [ drop exit-message #f ] [
            reader place-get swap reader-read-string
            dig reader swap place-set drop
            error? if [ "read error" stack-dump error ] [ drop ]
            interp swap interpreter-body-prepend
            while [
                interpreter-run
                const paused
                interpreter-complete? if [ #f ] [
                    paused error? if [
                        equals? ' quote-nothing if [ drop ] [
                            ansi [ bright red fg value->string print reset ]
                            "\n" print
                            interpreter-reset drop
                        ]
                        #f
                    ] [
                        stack-dump
                        pause
                        #t
                    ]
                ]
            ] []
            #t
        ]
    ] []

]

export worst-repl

