
; Not really a standard library, more like a random bag of helpful stuff

[
    upquote ; name
    upquote ; body
    quote current-defenv uplevel
    defenv-new-locals
    value-set-defenv
    swap
    quote definition-add uplevel
]
current-defenv
defenv-new-locals
value-set-defenv
quote define definition-add

; updo thing => quote thing uplevel
define updo [ upquote quote uplevel uplevel ]
; value const name -> define name [value]
define const [ value->constant upquote updo definition-add ]
define false? [ clone not ]

; a b clone2 => a b a b
define clone2 [ swap clone dig clone bury ]

define print [ stdout-port swap stdout-port-write-string stdout-port-flush drop drop ]
define println [ "\n" string-append print ]

define list-iter [
    upquote
    updo current-defenv defenv-new-locals value-set-defenv
    const body

    const list
    list list-length const len
    0 while (clone len i64-compare -1 i64-equal) [ ; lt
        const n
        list n list-get
        body quote eval quote uplevel uplevel
        n 1 i64-add
    ] drop
]

; basic import/export: embedded only, trusted/cooperative modules, no caching
; export name (toplevel only)
define export [
    upquote const name
    name updo definition-resolve false? if [
        drop
        "export: not defined: " name symbol->string string-append
        println error
    ] [ ]
    name quote definition-add quote uplevel uplevel
]
; import "path/to/embedded/file.w"
define import [
    upquote
    string->fs-path
    embedded-file-open
    embedded-file-port->string
    read-string->list
    updo current-defenv
    defenv-new-locals
    value-set-defenv
    updo eval
]

; define (attr...) name (body...)
import "worst/base/attribute.w"
; predicate dispatch attribute
import "worst/base/dispatch.w"
; equal, compare, add/sub/etc, length, value->string - things using dispatch
import "worst/base/ops.w"

; doc attribute
import "worst/base/doc.w"
; bunch of docs for builtins:
; docs for everything above this line go in here
; docs for everything below this line should be added using the doc attribute
import "worst/base/builtin-docs.w"

define ' [ upquote ]
; do [ code... ] => eval code
define do [ upquote updo eval ]

define list-empty? [clone list-length 0 equal]

define read-line [ stdin-port-read-line ]

; better import/export
; anything above this line is in the default module environment
import "worst/base/import.w"

import syntax/case

command-line-arguments list-pop drop ; $0
case [
    (list-empty?) {
        drop
        import ui/repl
        import ui/help
        worst-repl
    }
    ; TODO check file exists or is a module
    #t {
        list-pop swap drop
        const path
        path
        string->fs-path
        file-open-options file-open-options-set-read
        file-open
        false? if [
            ; TODO nicer error
            drop path pause
        ] [
            ; TODO load module
            read-port->list eval
        ]
    }
]

