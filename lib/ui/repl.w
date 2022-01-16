
import {
    worst/interpreter
    syntax/cond
    data/string
}

define worst-repl [

    define clear-stack [ [] stack-set ]

    interpreter-empty
    interpreter-inherit-definitions
    const interp

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

    ; read-one -> value #t | continue? #f
    define read-one [
        while [
            current-input-port
            port-peek-char
            cond [
                [eof-object?] [
                    ; leave loop, don't continue
                    drop drop drop #f #f #f
                ]
                ; newline: leave loop, maybe continue
                ["\n" equal? swap drop] [
                    drop port-read-char drop drop
                    #t #f #f
                ]
                ; drop whitespace
                [ "%s" string-contains-match? ] [
                    drop port-read-char drop drop
                    #t
                ]
                ; anything else: read a value, leave loop
                [#t] [
                    drop
                    port-read-value
                    swap drop
                    #t #f
                ]
            ]
        ] []
    ]

    ; read-more -> [values] | #f
    define read-more [
        []
        while [
            read-one
            if [ list-push #t ] [ ; value
                ; newline / eof
                if [ list-reverse ] [ #f ]
                #f
            ]
        ] []
    ]

    define stack-prompt [
        while [
            continuation-prompt
            read-one
            if [
                interp swap
                interpreter-stack-push
                drop
                #f
            ] []
        ] []
    ]

    define toplevel-quote-error? [
        error? if [
            interp interpreter-toplevel swap drop if [
                clone error->list list-pop swap drop
                "quote-nothing" equal? bury drop drop
            ] [ #f ]
        ] [ #f ]
    ]

    define eval/prompt [
        interp interpreter-run swap drop
        cond [
            [false?] [ drop standard-prompt ]
            [toplevel-quote-error?] [
                drop
                stack-prompt ; eval/prompt
            ]
            [#t] [
                ansi [ red fg ->string print reset ] "\n" print
                interp interpreter-reset drop
                standard-prompt
            ]
        ]
    ]

    define interp-give [
        const more
        interp interpreter-body-get
        more list-append
        interpreter-body-set
        drop
    ]

    while [
        eval/prompt
        read-more
        false? if [ ] [ interp-give #t ]
    ] []

]

export worst-repl

; vi: ft=scheme

