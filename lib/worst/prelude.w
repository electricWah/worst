
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
define error? [clone <is-error> type-id->unique value-meta-entry not not]

define read-line [ stdin-port-read-line ]

; required by import module cache
import "worst/data/hashmap.w"

; better import/export
; anything above this line is in the default module environment
import "worst/base/import.w"

#f const current-module

command-line-arguments list-pop drop ; $0
list-empty? if [
    drop
    import ui/repl
    import ui/help
    worst-repl
] [
    list-pop const path const args
    path
    string->fs-path
    file-open-options file-open-options-set-read
    file-open
    error? if [
        import ui/cli
        drop args path string->symbol cli-module-run
    ] [
        ; jank to get ui/cli and import "relative.w" working
        path const current-script-path
        read-port->list eval
    ]
]

