
import syntax/attribute

define clear-stack [ [] interpreter-stack-set ]

define with-stty [
    upquote const %stty-opts
    upquote const %with-stty-body
    define %run-stty [
        "stty" system-resolve-path make-command
        swap command-set-arguments
        current-input-port command-set-stdin
        command-spawn
    ]
    define %stty-ok [
        process-wait
        0 equal? if [drop drop drop] ["stty failed" abort] ; TODO
    ]
    ["-g"] %run-stty
    process-stdout-port port-read-line [] swap list-push const %stty-restore
    drop %stty-ok
    
    %stty-opts %run-stty %stty-ok

    %with-stty-body eval

    %stty-restore %run-stty %stty-ok
]

; Read a line from a person, using arrow keys and backspace and everything
; TODO. Using rlwrap for now
define input-line-editor [
    ; Split the input line around the position of the cursor
    "" make-place const read-a
    "" make-place const read-z
    define read-char [ current-input-port port-read-char swap drop ]
    define peek-char [ current-input-port port-peek-char swap drop ]
    define has-char? [ current-input-port port-has-char? swap drop ]
    define read-escape-sequence [
        peek-char equals? #\133 if [
            drop read-char drop
            read-char
        ] [
            drop
        ]
    ]
    with-stty ["raw" "-echo" "-brkint"] [ read-char ]
    equals? #\033 if [
        drop has-char? if [
            read-escape-sequence
        ] [
            ; Escape key pressed. Do nothing?
        ]
    ] [
        interpreter-dump-stack
        ->string
        read-a place-get swap
        bury
        swap string-append
        place-set drop
    ]
]

define worst-repl-prompt [
    ansi [
        green fg
        "worst " print

        reset cyan fg
        interpreter-stack current-output-port swap port-write-value drop

        bold yellow fg
        " > " print
        reset
    ]
]

; TODO quote is a definition here so definition-add needs to handle functions
lexical-alias quote %%quote
define read-eval-loop [
    define quote-read-syntax? [
        %%quote %%quote
        %%quote definition-get uplevel
        not not
        swap drop
    ]

    define run-repl [
        %%quote quote uplevel
        eof-object? %%quote current-context-clear when
        symbol?
        %%quote call swap
        %%quote drop swap
        %%quote uplevel swap
        %%quote swap when drop
        call
        %%quote run-repl definition-resolve swap drop
        current-context-set-code
    ]

    define port-drop-char [ port-read-char drop ]

    define port-drop-newline [
        port-peek-char
        #\newline equal?
        bury drop drop
        %%quote port-drop-char when
    ]

    define syntax-read [
        source-input-port
        port-has-data
        %%quote port-drop-newline when
        port-has-data not
        swap drop
        %%quote worst-repl-prompt when
        source-input-port
        port-read-value swap drop
    ]

        ; source-input-port
        ; port-has-data
        ; %%quote port-drop-newline when
        ; interpreter-dump-stack
        ; %%quote worst-repl-prompt when
        ; source-input-port
        ; port-read-value swap drop
    ; ]

    define quote [
        %%quote %%quote
        %%quote syntax-read
        %%quote quote-read-syntax? uplevel
        %%quote swap when
        drop uplevel
    ]
    run-repl
]

lexical (read-eval-loop)
lexical-alias quote %%quote
define worst-repl [
    define source-input-port [ current-input-port ]
    define syntax-read [
        source-input-port port-has-char? if [
            port-peek-char equals? #\newline swap drop if [
                port-read-char drop
                #t
            ] [ #f ]
        ] [ #t ]
        if [ drop worst-repl-prompt source-input-port ] []
        port-read-value swap drop
    ]

    define %run [
        define %%repl []
        read-eval-loop
        drop
    ]

    ; use builtin-quote: quote depends on %%repl via quote-read-syntax?
    ; Unwinds stack to toplevel (wherever %%repl is defined)
    define %abort-to-repl [
        ; interpreter-dump-stack
        %%quote %%repl
        %%quote definition-exists uplevel swap drop
        if [] [
            [current-context-remove-children %%quote %abort-to-repl uplevel]
            %%quote current-context-set-code
            %%quote uplevel
            %%quote uplevel
            uplevel
        ]
    ]

    ; define current-error-handler [
    ;     define current-output-port [current-error-port]
    ;     ansi [
    ;         ->string
    ;         " " string-append
    ;         "Error: " swap string-append
    ;         bold bright red fg print
    ;         current-output-port swap port-write-value drop
    ;         reset "\n" print
    ;     ]
    ;     updo %abort-to-repl
    ; ]

    define quote-read-syntax? [
        %%quote %%repl
        %%quote definition-exists uplevel
        swap drop
    ]
    ansi [
        "Welcome to the Worst interactive environment. Type " print
        bright yellow fg "help" print
        reset " for assistance.\n" print
    ]
    %run
]

export-name clear-stack
export-name input-line-editor
; export-name with-stty
export-name worst-repl-prompt
export-name worst-repl

; vi: ft=scheme

