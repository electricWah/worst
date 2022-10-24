
; Not really a standard library, more like a random bag of helpful stuff

define default-attributes []

define ' [ upquote ]
; updo thing => quote thing uplevel
define updo [ upquote quote uplevel uplevel ]
; do [ code... ] => eval code
define do [ upquote updo eval ]

define equal [ drop drop #f ]
define (dispatch ((i64? i64?) stack-matches?)) equal [ i64-equal ]
define (dispatch ((f64? f64?) stack-matches?)) equal [ f64-equal ]
define (dispatch ((string? string?) stack-matches?)) equal [ string-equal ]
define (dispatch ((symbol? symbol?) stack-matches?)) equal [ symbol-equal ]
define (dispatch ((bool? bool?) stack-matches?)) equal [ bool-equal ]

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

define (dispatch ((i64? i64?) stack-matches?)) le [ i64-le ]
define (dispatch ((f64? f64?) stack-matches?)) le [ f64-le ]
define (dispatch ((i64? i64?) stack-matches?)) lt [ i64-lt ]
define (dispatch ((f64? f64?) stack-matches?)) lt [ f64-lt ]
define (dispatch ((i64? i64?) stack-matches?)) ge [ i64-ge ]
define (dispatch ((f64? f64?) stack-matches?)) ge [ f64-ge ]
define (dispatch ((i64? i64?) stack-matches?)) gt [ i64-gt ]
define (dispatch ((f64? f64?) stack-matches?)) gt [ f64-gt ]

define (dispatch ((list? list?) stack-matches?)) append [ list-append ]
define (dispatch ((string? string?) stack-matches?)) append [ string-append ]

define (dispatch (list?)) length [ list-length ]

define value->string [drop "<value>"]
define (dispatch (string?)) value->string []
define (dispatch (bool?)) value->string [if ["#t"] ["#f"]]
define (dispatch (symbol?)) value->string [symbol->string]
define (dispatch (i64?)) value->string [i64->string]
define (dispatch (f64?)) value->string [f64->string]
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

define false? [ clone not ]

; dynamic definitions (using dynamic values)

; true only within the attributes clause of a define form
define in-definition-attributes [ quote definition-attributes dynamic-resolve ]

define list-empty? [clone list-length 0 equal]

; a b clone2 => a b a b
define clone2 [ swap clone dig clone bury ]

; maybe these should eval, so you can do [5 le? (4 3 add)]
define equal? [ clone2 equal ]
; a <op> b => a bool
define equals? [ clone upquote equal ]
define lt? [ clone upquote lt ]
define le? [ clone upquote le ]
define gt? [ clone upquote gt ]
define ge? [ clone upquote ge ]

define abs [ lt? 0 if [negate] [] ]
define max [ clone2 lt if [swap] [] drop ]
define min [ clone2 lt if [] [swap] drop ]

define (dispatch (embedded-file-port?))
port->string [ embedded-file-port->string ]
define (dispatch (file-port?))
port->string [ file-port->string ]

define read-port->list [ port->string read-string->list ]

define ->string [ value->string ]

define print [ current-output-port swap port-write-string port-flush drop ]
define print-value [ ->string print ]
define println [ ->string "\n" string-append print ]

define read-line [ current-input-port buffered-port-read-line swap drop ]

export #t

