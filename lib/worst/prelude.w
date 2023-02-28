
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

define ' [ upquote ]
; updo thing => quote thing uplevel
define updo [ upquote quote uplevel uplevel ]
; do [ code... ] => eval code
define do [ upquote updo eval ]
; value const name -> define name [value]
define const [ value->constant upquote updo definition-add ]

define define [
    upquote list? if [ updo upquote ] [ [] swap ]
    const name
    const attrs

    upquote
    quote current-defenv uplevel
    defenv-new-locals
    value-set-defenv
    const body
    
    ; eval attrs: body name -> body name
    body name
    [ #t const definition-attributes ] attrs list-append
    updo eval
    const name
    const body

    body name
    quote definition-add uplevel
]

; define (dispatch (cond: -> bool)) name [ body ]
; => define name [ cond if [ body ] [ previous definition for name ] ]
define dispatch [
    upquote
    quote current-defenv uplevel
    defenv-new-locals
    value-set-defenv
    const dispatch-case

    const name
    const body
    name updo definition-resolve const prev
    prev not not const any-prev

    ; new-def checks dispatch-case and picks body or prev to eval at same level
    define new-def [
        dispatch-case eval if [ body ] [
            any-prev if [ prev ] [
                (no-matching-dispatch)
                name list-push
                list-reverse
                error
            ]
        ]
        quote eval uplevel
    ]

    quote new-def definition-resolve name
]

; define (recursive) infinite-loop [ infinite-loop ]
; attribute: define self within body to enable recursive calls
; works funny when there's an existing definition with the same name!
; TODO probably doesn't work
define recursive [
    const name
    const body

    define recursive-call [ name updo dynamic-resolve-any updo eval ]

    body
    name
    quote recursive-call definition-resolve
    value-set-not-dynamic-resolvable ; lol
    value-definition-add

    name
]

define list-iter [
    upquote const body
    const list
    list list-length const len
    0 while (clone len i64-compare -1 i64-equal) [ ; lt
        const n
        list n list-get
        body quote eval quote uplevel uplevel
        n 1 i64-add
    ] drop
]

; define (with-dynamics (a b)) def (... a ... b ...)
; in def, treat given definitions as dynamic
; useful for e.g. mutually recursive defs
define with-dynamics [
    const name
    const body

    upquote const dynamics

    body
    dynamics list-iter [
        const def
        define dynamic-call [
            def dynamic-resolve-any
            quote eval uplevel
        ]
        def quote dynamic-call definition-resolve
        value-set-not-dynamic-resolvable ; lol
        value-definition-add
    ]

    name
]

; a b and2? (a -> a bool) (b -> b bool) -> a b bool
; do both predicates return true on the top two values?
define and2? [
    upquote const predA
    upquote const predB
    const b
    predA eval if [ b predB eval ] [ b #f ]
]

; a b clone2 => a b a b
define clone2 [ swap clone dig clone bury ]

define equal [ drop drop #f ]

define (dispatch (and2? i64? i64?)) equal [ i64-equal ]
define (dispatch (and2? f64? f64?)) equal [ f64-equal ]
define (dispatch (and2? string? string?)) equal [ string-equal ]
define (dispatch (and2? symbol? symbol?)) equal [ symbol-equal ]
define (dispatch (and2? bool? bool?)) equal [ bool-equal ]

define (dispatch (and2? i64? i64?)) compare [ i64-compare ]
define (dispatch (and2? f64? f64?)) compare [ f64-compare ]
define (dispatch (and2? string? string?)) compare [ string-compare ]

define le [compare 1 equal not]
define lt [compare -1 equal]
define ge [compare -1 equal not]
define gt [compare 1 equal]

define (dispatch (and2? i64? i64?)) add [ i64-add ]
define (dispatch (and2? f64? f64?)) add [ f64-add ]
define (dispatch (and2? i64? i64?)) sub [ i64-sub ]
define (dispatch (and2? f64? f64?)) sub [ f64-sub ]
define (dispatch (and2? i64? i64?)) mul [ i64-mul ]
define (dispatch (and2? f64? f64?)) mul [ f64-mul ]
define (dispatch (and2? i64? i64?)) div [ i64-div ]
define (dispatch (and2? f64? f64?)) div [ f64-div ]

define (dispatch (i64?)) negate [ i64-negate ]
define (dispatch (f64?)) negate [ f64-negate ]
define (dispatch (i64?)) abs [ i64-abs ]
define (dispatch (f64?)) abs [ f64-abs ]

define false? [ clone not ]
define equal? [ clone2 equal ]
; a <op> b => a bool
define equals? [ clone upquote updo eval equal ]
define lt? [ clone upquote updo eval lt ]
define le? [ clone upquote updo eval le ]
define gt? [ clone upquote updo eval gt ]
define ge? [ clone upquote updo eval ge ]

; a b bool-and => bool
define bool-and [ if [ ] [ drop #f ] ]
define bool-and? [ clone2 bool-and ] ; idk
define bool-or [ if [ drop #t ] [ ] ]

define (dispatch (and2? list? list?)) append [ list-append ]
define (dispatch (and2? string? string?)) append [ string-append ]

define (dispatch (list?)) length [ list-length ]
define (dispatch (bytevector?)) length [ bytevector-length ]

define value->string [drop "<value>"]
define (dispatch (string?)) value->string []
define (dispatch (bool?)) value->string [if ["#t"] ["#f"]]
define (dispatch (symbol?)) value->string [symbol->string]
define (dispatch (i64?)) value->string [i64->string]
define (dispatch (f64?)) value->string [f64->string]
define (dispatch (interpreter?)) value->string [drop "<interpreter>"]
define (dispatch (i64map?)) value->string [drop "<i64map>"]
; define (dispatch (file-port?)) value->string [drop "<file-port>"]
; define (dispatch (embedded-file-port?)) value->string [drop "<embedded-file-port>"]

define (dispatch (builtin?)) value->string [
    builtin-name false? if [ drop "<builtin>" ] [
        value->string "<builtin " swap string-append ">" string-append
    ]
]

define (with-dynamics (value->string) dispatch (list?)) value->string [
    "(" "" dig list-iter [
        value->string
        ; concat accumulator with either "" or previous trailing " "
        bury string-append swap
        string-append " "
    ]
    ; drop trailing " " or semi-sacrificial ""
    drop ")" string-append
]

;define (dispatch (bytevector?)) value->string [
;    length const len
;    "[" len value->string append " bytes]" append
;    swap drop
;]

; true only within the attributes clause of a define form
define in-definition-attributes [ quote definition-attributes dynamic-resolve ]

define list-empty? [clone list-length 0 equal]

define abs [ lt? 0 if [negate] [] ]
define max [ clone2 lt if [swap] [] drop ]
define min [ clone2 lt if [] [swap] drop ]

; define value-hash [ drop #f bool-hash ] ; the default hash is that of false
; define (dispatch (bool?)) value-hash [ bool-hash ]
; define (dispatch (symbol?)) value-hash [ symbol-hash ]
; define (dispatch (string?)) value-hash [ string-hash ]
; define (dispatch (i64?)) value-hash [ i64-hash ]

; define hashtable-insert [
;     swap clone value-hash dig
;     hashtable-insert-hashed-value
; ]

; define hashtable->pairs [
;     const ht
;     []
;     ht hashtable-hash-keys list-iter [
;         ht swap hashtable-hash-bucket list-append
;     ]
; ]

define print [ stdout-port swap stdout-port-write-string stdout-port-flush drop drop ]
define print-value [ value->string print ]
define println [ value->string "\n" string-append print ]

define port->string [ println #f error ]
define (dispatch (file-port?)) port->string [ file-port->string ]
define (dispatch (embedded-file-port?)) port->string [ embedded-file-port->string ]
define read-port->list [ port->string read-string->list ]

define read-line [ stdin-port-read-line ]

define feature-enabled? [
    upquote const name
    #f features-enabled list-iter [
        name equal if [ drop #t ] [ ]
    ]
]

feature-enabled? os if [
    "WORST_LIBPATH" environment-variable
    false? if [ drop () ] [ ":" string-split ]
] [ () ]
const WORST_LIBPATH

define export [
    upquote
    list? if [] [ () swap list-push ]
    const exports
    quote module-exports dynamic-resolve-local
    false? if [ "export: not in a module" println error ] [
        const module-exports
        module-exports place-get
        exports list-iter [
            const x
            x dynamic-resolve-local false? if [
                "export: not defined: " x value->string string-append
                clone println error
            ] [ ]
            const def
            x def defset-insert
        ]
        module-exports swap place-set drop
    ]
]

current-defenv make-place const global-default-module-definitions
define default-module-definitions [
    quote current-default-module-definitions updo dynamic-resolve-local
    false? if [
        drop global-default-module-definitions place-get
    ] [ eval ]
]
define import [
    defset-empty make-place const all-imports
    upquote
    list? if [] [ () swap list-push ]
    list-iter [
        symbol? if [
            symbol->string const modname
            #f
            ; maybe check feature-enabled? os
            WORST_LIBPATH list-iter [
                const path
                false? if [
                    drop
                    path string->fs-path
                    modname ".w" string-append string->fs-path
                    fs-path-concat
                    file-open-options file-open-options-set-read
                    ; try opening
                    file-open error? if [ drop #f ] [ ]
                ] []
            ]
            false? if [
                ; still not found, try embedded
                drop
                modname ".w" string-append string->fs-path
                embedded-file-open
                error? if [ ] [ embedded-file-port->string ]
            ] [
                file-port->string
            ]
            false? if [ modname "not-found" error ] [
            ]
            ; read module
            read-string->list const modbody

            defset-empty make-place const module-exports
            interpreter-empty
            default-module-definitions
            defset-empty
            quote module-exports module-exports defset-insert
            defenv-merge-locals
            interpreter-defenv-set

            modbody interpreter-eval-list-next
            interpreter-run
            const ret
            interpreter-complete? if [
                ; TODO caching
                drop
                module-exports place-get
                all-imports place-get
                swap defset-merge
                all-imports swap place-set drop
            ] [
                "Error in " print modname value->string print ": " print
                ret value->string println
                (module error) error
            ]
        ] [ "import non-symbol" TODO ]
    ]
    all-imports place-get
    updo current-defenv-merge-locals
]
global-default-module-definitions current-defenv place-set drop

import worst/doc
global-default-module-definitions current-defenv place-set drop

import syntax/case

command-line-arguments list-pop drop ; $0
case [
    (list-empty?) {
        drop
        import ui
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

