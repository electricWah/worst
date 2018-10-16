
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
    make-interpreter
    
    read-line

    newline
    string? if [
        print-string
        newline
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

