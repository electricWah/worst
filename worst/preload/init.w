
; Not really a standard library, more like a random bag of helpful stuff

[
    upquote ; name
    upquote ; body
    <defenv> type->unique
    quote current-defenv uplevel
    defenv-new-locals
    value-insert-meta-entry
    swap
    quote definition-add uplevel
]
<defenv> type->unique
current-defenv
defenv-new-locals
value-insert-meta-entry
quote define definition-add

quote define definition-resolve drop
; updo thing => quote thing uplevel
define updo [ upquote quote uplevel uplevel ]
; value const name -> define name [value]
define const [ value->constant upquote updo definition-add ]
define false? [ clone not ]

; val type is-type => val bool
define is-type [ swap clone value-type dig type-equal ]

; bool if [ if-true ] [ if-false ]
define if [ upquote upquote swap dig quote swap eval-if drop uplevel ]

define value-set-defenv [
    <defenv> type->unique
    swap value-insert-meta-entry
]

define while [
    updo current-defenv defenv-new-locals const env
    upquote env value-set-defenv const while-cond
    upquote env value-set-defenv const while-body

    define the-whiler [
        ; while-body dump1 while-cond dump1
        const continuer
        while-cond uplevel const ok
        ok if [ while-body ] [ [] ] uplevel
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

define ' [ upquote ]
; do [ code... ] => eval code
define do [ upquote updo eval ]

define error? [clone is-error-key value-meta-entry not not]

define read-line [ stdin-port-read-line ]

#f const current-module

