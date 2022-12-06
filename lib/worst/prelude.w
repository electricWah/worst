
; Not really a standard library, more like a random bag of helpful stuff

define default-attributes []

define ' [ upquote ]
; updo thing => quote thing uplevel
define updo [ upquote quote uplevel uplevel ]
; do [ code... ] => eval code
define do [ upquote updo eval ]
; value const name -> define name [value]
define const [ value->constant upquote updo definition-add ]

; a b clone2 => a b a b
define clone2 [ swap clone dig clone bury ]

define equal [ drop drop #f ]
define (dispatch ((i64? i64?) stack-matches?)) equal [ i64-equal ]
define (dispatch ((f64? f64?) stack-matches?)) equal [ f64-equal ]
define (dispatch ((string? string?) stack-matches?)) equal [ string-equal ]
define (dispatch ((symbol? symbol?) stack-matches?)) equal [ symbol-equal ]
define (dispatch ((bool? bool?) stack-matches?)) equal [ bool-equal ]

define (dispatch ((i64? i64?) stack-matches?)) compare [ i64-compare ]
define (dispatch ((f64? f64?) stack-matches?)) compare [ f64-compare ]
define (dispatch ((string? string?) stack-matches?)) compare [ string-compare ]

define le [compare 1 equal not]
define lt [compare -1 equal]
define ge [compare -1 equal not]
define gt [compare 1 equal]

define (dispatch ((i64? i64?) stack-matches?)) add [ i64-add ]
define (dispatch ((f64? f64?) stack-matches?)) add [ f64-add ]
define (dispatch ((i64? i64?) stack-matches?)) sub [ i64-sub ]
define (dispatch ((f64? f64?) stack-matches?)) sub [ f64-sub ]
define (dispatch ((i64? i64?) stack-matches?)) mul [ i64-mul ]
define (dispatch ((f64? f64?) stack-matches?)) mul [ f64-mul ]
define (dispatch ((i64? i64?) stack-matches?)) div [ i64-div ]
define (dispatch ((f64? f64?) stack-matches?)) div [ f64-div ]

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

define list-iter [
    upquote const body
    const list
    list list-length const len
    0 while (clone len lt) [
        const n
        list n list-get
        body quote eval quote uplevel uplevel
        n 1 i64-add
    ] drop
]

define (dispatch ((list? list?) stack-matches?)) append [ list-append ]
define (dispatch ((string? string?) stack-matches?)) append [ string-append ]

define (dispatch (list?)) length [ list-length ]
define (dispatch (bytevector?)) length [ bytevector-length ]

define value->string [drop "<value>"]
define (dispatch (string?)) value->string []
define (dispatch (bool?)) value->string [if ["#t"] ["#f"]]
define (dispatch (symbol?)) value->string [symbol->string]
define (dispatch (i64?)) value->string [i64->string]
define (dispatch (f64?)) value->string [f64->string]
define (dispatch (file-port?)) value->string [drop "<file-port>"]
define (dispatch (embedded-file-port?)) value->string [drop "<embedded-file-port>"]

define (dispatch (builtin?)) value->string [
    builtin-name false? if [ drop "<builtin>" ] [
        value->string "<builtin " swap string-append ">" string-append
    ]
]

define (recursive dispatch (list?)) value->string [
    "(" "" dig list-iter [
        value->string
        ; concat accumulator with either "" or previous trailing " "
        bury string-append swap
        string-append " "
    ]
    ; drop trailing " " or semi-sacrificial ""
    drop ")" string-append
]
define (dispatch (bytevector?)) value->string [
    length const len
    "[" len value->string append " bytes]" append
    swap drop
]

define (dispatch (file-port?)) port->string [ file-port->string ]
define (dispatch (embedded-file-port?)) port->string [ embedded-file-port->string ]

; true only within the attributes clause of a define form
define in-definition-attributes [ quote definition-attributes dynamic-resolve ]

define list-empty? [clone list-length 0 equal]

define abs [ lt? 0 if [negate] [] ]
define max [ clone2 lt if [swap] [] drop ]
define min [ clone2 lt if [] [swap] drop ]

define (dispatch (embedded-file-port?))
port->string [ embedded-file-port->string ]
define (dispatch (file-port?))
port->string [ file-port->string ]

define read-port->list [ port->string read-string->list ]

define print [ stdout-port swap stdout-port-write-string stdout-port-flush drop drop ]
define print-value [ value->string print ]
define println [ value->string "\n" string-append print ]

define read-line [ stdin-port-read-line ]

define feature-enabled? [
    upquote const name
    #f features-enabled list-iter [ name equal if [ drop #t ] [ ] ]
]

export #t

