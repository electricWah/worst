
;;; vi: ft=scheme

; The REPL runs an inner interpreter.
; - Keep an interpreter around
; - Hijack stdin, read it line by line, etc
; - Hijack define, allow you to redefine later
; - Add pretty-printing of datum and errors
; - Fully interactive: line history, current stack, error trace,
;   autocomplete, quote suggestions
; - Hopefully take out the original repl from the hell binary
;   and remove Debug/Display/Show from all types
; - customize all datum, stack, context and error printing

define worst-repl [

    current-interpreter interpreter-get-reader swap drop
    make-interpreter local %interpreter

    ; interpreter item => interpreter
    define %interpreter-push-stack [
        swap
        [] interpreter-swap-stack
        2 dig list-push-tail
        interpreter-swap-stack
        drop
    ]

    define error [
        ansi [ "Error: " bold red fg print-string reset ]
        print-string/n
    ]
    
    read-line newline

    string? if [
        %newline string-push
        %interpreter swap interpreter-push-input
        interpreter-read-next
        if [
            local %read
            %read symbol? if [
                interpreter-resolve-symbol if [
                    interpreter-eval-code
                ] [
                    "Not defined" error
                    %read symbol->string print-string/n
                ]
            ] [
                %interpreter-push-stack
            ]
        ] [
            failure-message error
            drop
        ]
    ] []

    ; o yeah nice

    ; define loop [
    ;     ; newline
    ;     ; with-stty [raw -echo] [read-line]
    ;     read-line
    ; ]
    ; loop
    ; do []
]

export-global worst-repl

