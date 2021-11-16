
import {
    worst/interpreter
    syntax/cond
    data/string
}

define worst-repl [

    define clear-stack [ [] stack-set ]

    interpreter-empty
    interpreter-inherit-definitions

    const %interp

    ansi [
        "Welcome to the Worst interactive environment. Type " print
        bright yellow fg "help" print
        reset " for assistance.\n" print
    ]

    define display-prompt [
        const stack
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

    current-input-port 
    []
    while [
        %interp
        swap list-reverse interpreter-body-set
        interpreter-run

        ; don't display regular prompt if the quote prompt came up
        false? if [ drop #t ] [
            error? if [
                clone
                error->list ["quote-nothing"] equal? bury drop drop
                if [
                    ; if it's toplevel then more syntax is required
                    swap interpreter-toplevel bury swap dig
                    if [
                        drop
                        ansi [ cyan fg "... " print yellow fg "> " print reset ]
                        ; unintelligent read
                        swap port-read-value const v swap
                        v interpreter-stack-push
                        #f
                    ] [
                        ; quote-nothing but not at toplevel, reset
                        ->string ansi [ bright red fg print reset ]
                        "\n" print
                        interpreter-reset
                        #t
                    ]
                ] [
                    ; some other error, reset
                    ->string ansi [ bright red fg print reset ]
                    "\n" print
                    interpreter-reset
                    #t
                ]
            ] [
                ; some other pause, TODO debugging?
                ansi [
                    bright blue fg
                    "Paused (resetting): " print
                    cyan fg
                    ->string print
                    reset
                ]
                "\n" print
                interpreter-reset
                #t
            ]
        ]
        if [ interpreter-stack-get display-prompt ] []
        drop

        []
        swap

        ; eat whitespace
        while [
            port-peek-char
            cond [
                ; eof: escape both loops
                [eof-object?] [ drop #f #f ]
                ; newline: escape this loop
                ["\n" equal? swap drop] [
                    drop port-read-char drop
                    #t #f
                ]
                ; whitespace?
                [ "%s" string-contains-match? ] [
                    drop port-read-char drop
                    #t
                ]
                ; anything else: read a value
                [#t] [
                    drop
                    port-read-value
                    bury swap dig list-push swap
                    #t
                ]
            ]
        ] []
        bury swap dig
    ] [ ]
    drop drop
]

export worst-repl

; vi: ft=scheme

