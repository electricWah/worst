
; Entry point
; - adds some necessary definitions and basic module loading
; - loads enough other modules to be useful
; - starts a REPL

[ quote quote quote uplevel uplevel ] quote upquote definition-add

[ ; define name [def...]
    upquote upquote ; name [def ...]
    swap quote definition-add uplevel
] quote define definition-add

define const [
    [quote] swap list-push list-reverse
    upquote
    quote definition-add uplevel
]

define ->string [ to-string/terse swap drop ]

define port-write-value [ ->string port-write-string ]

; bool if [if-true] [if-false]
define if [
    upquote upquote
    ; cond true false => false true cond
    swap dig
    quote swap when drop
    quote eval uplevel
]

; while [-> bool] [body ...]
define while [
    upquote quote %%cond definition-add
    upquote quote %%while-body definition-add
    [
        %%cond if [%%while-body %%loop] [[]] current-context-set-code
    ] const %%loop
    %%loop current-context-set-code
]

define syntax-read [ source-input-port port-read-value swap drop ]

; path read-file -> list
define read-file [
    open-input-file false? if [ drop [] swap list-push abort ] []
    [] while [ swap port-read-value eof-object? not ] [ dig swap list-push ]
    drop drop
    list-reverse
]

"WORST_LIBPATH" env-get swap drop
false? if [drop ""] []
"[^:]+" string-global-matches
list-reverse "%/lib" list-push list-reverse
const WORST_LIBPATH

define import-path->file-name [
    "/" string-append p string-append ".w" string-append
]

; module-name resolve-import-path
; uses WORST_LIBPATH
define resolve-import-path [
    ->string const p
    #f ; not-found
    WORST_LIBPATH
    while [list-empty? not] [
        list-pop import-path->file-name const path

        path open-input-file
        false? if [drop drop] [
            port-close
            drop
            drop path [] ; exit loop and return path instead of not-found
        ]
    ]
    drop ; drop remaining WORST_LIBPATH
]

; Very basic import/export
define import [
    upquote resolve-import-path
    read-file quote eval uplevel
]
define export-name [
    upquote
    definition-resolve
    swap
    quote definition-add quote uplevel uplevel
]

; Generally useful utilities
import worst/base

import data/list
import data/dict

; Real import/export
import worst/module

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

; vi: ft=scheme


