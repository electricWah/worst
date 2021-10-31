
; Entry point
; - adds some necessary definitions and basic module loading
; - loads enough other modules to be useful
; - starts a REPL

; Core stuff necessary for everything else
"worst/base" module-import
; define
"worst/define" module-import
; import/export
"worst/module" module-import

; Useful stuff for programming in general
import worst/misc

import data/list
import data/dict

; Real import/export
; import worst/module

; TODO fix: without this, documentation attribute does nothing, for e.g. help
import syntax/attribute

import doc

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

