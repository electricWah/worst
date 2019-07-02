
define print [ current-output-port swap port-write-string drop ]

define ansi [

    0 const black
    1 const red
    2 const green
    3 const yellow
    4 const blue
    5 const magenta
    6 const cyan
    7 const white
    define bright [ upquote call 8 add ]

    define reset        [ "\e[0m" print ]
    define bold         [ "\e[1m" print ]
    define faint        [ "\e[2m" print ]
    define italic       [ "\e[3m" print ]
    define underline    [ "\e[4m" print ]
    define slow-blink   [ "\e[5m" print ]
    define fast-blink   [ "\e[6m" print ]
    define reverse      [ "\e[7m" print ]

    define cursor-save    [ "\e[s" print ]
    define cursor-restore [ "\e[u" print ]

    define fg [
        ->string
        "\e[38;5;" swap string-append
        "m" string-append
        print
    ]

    define bg [
        ->string
        "\e[48;5;" swap string-append
        "m" string-append
        print
    ]

    upquote eval
]

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
        peek-char equals? #\[ if [
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
        rot
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

define worst-repl [
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

    define run [
        define %%repl []
        read-eval-loop
        drop
    ]

    define %abort-to-repl [
        quote %%repl quote current-context-has-definition? uplevel swap drop
        if [] [
            [updo %abort-to-repl] quote current-context-set-code
            quote uplevel
            quote uplevel
            uplevel
        ]
    ]

    define current-error-handler [
        define current-output-port [current-error-port]
        ansi [
            ->string
            " " string-append
            "Error: " swap string-append
            bold bright red fg print
            current-output-port swap port-write-value drop
            reset "\n" print
        ]
        updo %abort-to-repl
    ]
    define quote-read-syntax? [
        builtin-quote %%repl
        builtin-quote current-context-has-definition? uplevel
        swap drop
    ]
    current-input-port [] swap list-push
    builtin-quote source-input-port definition-add
    run
]

export print
export input-line-editor
; export with-stty
export ansi
export worst-repl-prompt
export worst-repl

; vi: ft=scheme

