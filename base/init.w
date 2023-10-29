
; Bootstrap from builtins to a usable environment

[
    upquote ; name
    upquote ; body
    quote current-ambient-defset uplevel
    quote current-local-defset uplevel
    defset-merge
    make-defbody
    swap
    quote definition-add uplevel
]
current-ambient-defset
make-defbody
quote define definition-add

; updyn thing => quote thing uplevel
; "updyn" because it resolves in parent context, i.e. dynamically
define updyn [ upquote quote uplevel uplevel ]
; value const name -> define name [value]
define const [ value->constant upquote updyn definition-add ]
define false? [ clone not ]

; val type is-type => val bool
define is-type [ swap clone value-type-id dig value-equal ]

define current-defs [
    updyn current-ambient-defset
    updyn current-local-defset
    defset-merge
]

; quote swap definition-resolve const swapper

; bool if [ if-true ] [ if-false ]
define if [
    upquote upquote swap
    dig quote swap eval-if
    drop
    updyn current-defs make-defbody uplevel
]

; while [ -> bool ] [ body ]
define while [
    updyn current-defs const env
    upquote env make-defbody const cond
    upquote env make-defbody const body

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

define print [
    stdout-port swap port-write-string drop
    stdout-port port-flush
]
define println [ "\n" string-append print ]

; [a b ...] list-iter [code] => a code; b code; ...
define list-iter [
    upquote
    updyn current-defs make-defbody const body

    const list
    list list-length const len
    0 while (clone len value-compare -1 value-equal) [ ; lt
        const n
        list n list-get
        body quote uplevel uplevel
        n 1 i64-add
    ] drop
]

define list-empty? [clone list-length 0 value-equal]

command-line-arguments
list-pop drop ; worst exe
list-pop drop ; current file
const command-line-arguments

define file-eval [
    1 0 file-handle-open const fh
    fh port-read-all->string
    fh file-handle-close
    read-string->list
    updyn current-defs make-defbody updyn eval
]

; if can load zip files, do it (TODO)
; otherwise this file must be argv[1], so run argv[2]
command-line-arguments list-empty? if [
    no-file-given-TODO ; simple stdin mode?
] [
    command-line-arguments
    list-pop updyn file-eval
]

exit

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



