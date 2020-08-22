
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
    eval
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
    open-input-file
    [] while [ swap port-read-value eof-object? not ] [ dig swap list-push ]
    drop drop
    list-reverse
]

"WORST_LIBDIR" env-get swap drop
"%/lib" swap or bury drop drop
const WORST_LIBDIR

; module-name resolve-import-path
; uses WORST_LIBDIR
define resolve-import-path [
    symbol? if [
        ->string
        WORST_LIBDIR "/" string-append
        swap string-append
        ".w" string-append
    ] [ "resolve-import-path: invalid" abort ]
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

; Interactive
import doc
import ui

worst-repl

; vi: ft=scheme

