
; maybe make this dynamic?
define standard-worst-prompt [
    ; TODO make these work
    interpreter-stack-get ; const stack drop
    ansi [
        const stack drop
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

define worst-repl [

    define clear-stack [ while [stack-empty not] [drop] ]

    interpreter-empty
    current-defenv interpreter-defenv-set
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
            reader place-get swap
            reader-read-string const read-res
            ; also reader-check should return incomplete in some way
            reader swap place-set drop
            interp read-res interpreter-body-prepend
            interpreter-run
            ; error? if [
            ;     ; TODO broken until quotes are worst-only (if, while etc)
            ;     ; also check if toplevel
            ;     ; equals? ' quote-nothing if [ drop ] [
            ;         ansi [ bright red fg value->string print reset ]
            ;         "\n" print
            ;         interpreter-reset drop
            ;     ; ]
            ; ] [ drop ]
            #t
        ]
    ] []

]

