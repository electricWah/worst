
define worst-repl [

    import worst/interpreter
    import syntax/cond
    import data/string

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

        false? if [ drop ] [
            ; error
            ->string print
            "\n" print
            interpreter-reset
        ]
        interpreter-stack-get display-prompt
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

export-name worst-repl

; vi: ft=scheme

