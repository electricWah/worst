
; Not really a standard library, more like a random bag of helpful stuff

[
    upquote ; name
    upquote ; body
    <defenv> type-id->unique
    quote current-defenv uplevel
    defenv-new-locals
    value-insert-meta-entry
    swap
    quote definition-add uplevel
]
<defenv> type-id->unique
current-defenv
defenv-new-locals
value-insert-meta-entry
quote define definition-add

; updo thing => quote thing uplevel
define updo [ upquote quote uplevel uplevel ]
; value const name -> define name [value]
define const [ value->constant upquote updo definition-add ]
define false? [ clone not ]

; val type is-type => val bool
define is-type [ swap clone value-type-id dig type-id-equal ]

; bool if [ if-true ] [ if-false ]
define if [ upquote upquote dig not quote swap eval-if drop uplevel ]

define value-set-defenv [
    <defenv> type-id->unique
    swap value-insert-meta-entry
]

define while [
    updo current-defenv defenv-new-locals const env
    upquote env value-set-defenv const cond
    upquote env value-set-defenv const body

    define the-whiler [
        const continuer
        cond uplevel const ok
        ok if [ body ] [ [] ] uplevel
        ok if [ continuer continuer ] [ [] ] uplevel
    ]
    quote the-whiler definition-resolve clone uplevel
]

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
        body quote uplevel uplevel
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
; load-embedded "path/to/embedded/file.w"
define load-embedded [
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
load-embedded "base/attribute.w"
; predicate dispatch attribute
load-embedded "base/dispatch.w"
; equal, compare, add/sub/etc, length, value->string - things using dispatch
load-embedded "base/ops.w"

; doc attribute
load-embedded "doc/doc.w"
; bunch of docs for builtins:
; docs for everything above this line go in here
; docs for everything below this line should be added using the doc attribute
load-embedded "doc/builtins.w"

define ' [ upquote ]
; do [ code... ] => eval code
define do [ upquote updo eval ]

define list-empty? [clone list-length 0 equal]
define error? [clone <is-error> type-id->unique value-meta-entry not not]

define read-line [ stdin-port-read-line ]

; required by import module cache
load-embedded "base/hashmap.w"

; pretty useful to always have
load-embedded "base/case.w"

load-embedded "data/list.w"

; not ideal
load-embedded "cli/ansi.w"
#f const current-module
load-embedded "cli/help.w"
load-embedded "cli/repl.w"

; anything above this line is in the default module environment
; this should also be the last load-embedded as it redefines export
load-embedded "base/module.w"

command-line-arguments list-pop drop ; $0
list-empty? if [
    drop
    worst-repl
] [
    list-pop const path const args
    path
    string->fs-path
    file-open-options file-open-options-set-read
    file-open
    error? if [
        drop args path string->symbol cli-module-run
    ] [
        ; jank to get ui/cli and import "relative.w" working
        path const current-script-path
        read-port->list eval
    ]
]

