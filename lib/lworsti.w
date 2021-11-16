
; Entry point
; - adds some necessary definitions and basic module loading
; - loads enough other modules to be useful
; - starts a REPL

import {
    ; Useful stuff for programming in general
    worst/misc
    data/list
    ; data/dict

    ; TODO without this, documentation attribute does nothing, for e.g. help
    ; syntax/attribute
    doc ; this should be worst/doc and also import its own stuff
}

; Interactive if given no arguments
command-line-arguments
list-pop drop ; first is $0
list-empty? if [
    drop
    import ui
    worst-repl
] [
    ; Read and eval first arg as file
    list-pop swap drop
    read-file eval
]

