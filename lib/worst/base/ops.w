
; a b and2? (a -> a bool) (b -> b bool) -> a b bool
; do both predicates return true on the top two values?
define and2? [
    upquote const predA
    upquote const predB
    const b
    predA eval if [ b predB eval ] [ b #f ]
]

define equal [ drop drop #f ]

define (dispatch (and2? i64? i64?)) equal [ i64-equal ]
define (dispatch (and2? f64? f64?)) equal [ f64-equal ]
define (dispatch (and2? string? string?)) equal [ string-equal ]
define (dispatch (and2? symbol? symbol?)) equal [ symbol-equal ]
define (dispatch (and2? bool? bool?)) equal [ bool-equal ]
define equal? [ clone2 updo equal ]
export equal
export equal?

define (dispatch (and2? i64? i64?)) compare [ i64-compare ]
define (dispatch (and2? f64? f64?)) compare [ f64-compare ]
define (dispatch (and2? string? string?)) compare [ string-compare ]
export compare

define le [compare 1 equal not]
define lt [compare -1 equal]
define ge [compare -1 equal not]
define gt [compare 1 equal]
export le
export lt
export ge
export gt

; a <op> b => a bool
define equals? [ clone upquote updo eval equal ]
define lt? [ clone upquote updo eval lt ]
define le? [ clone upquote updo eval le ]
define gt? [ clone upquote updo eval gt ]
define ge? [ clone upquote updo eval ge ]
export equals?
export le?
export lt?
export ge?
export gt?

define (dispatch (and2? i64? i64?)) add [ i64-add ]
define (dispatch (and2? f64? f64?)) add [ f64-add ]
define (dispatch (and2? i64? i64?)) sub [ i64-sub ]
define (dispatch (and2? f64? f64?)) sub [ f64-sub ]
define (dispatch (and2? i64? i64?)) mul [ i64-mul ]
define (dispatch (and2? f64? f64?)) mul [ f64-mul ]
define (dispatch (and2? i64? i64?)) div [ i64-div ]
define (dispatch (and2? f64? f64?)) div [ f64-div ]
export add
export sub
export mul
export div

define (type-dispatch i64?) negate [ i64-negate ]
define (type-dispatch f64?) negate [ f64-negate ]
export negate

define abs [ lt? 0 if [negate] [] ]
define (type-dispatch i64?) abs [ i64-abs ]
define (type-dispatch f64?) abs [ f64-abs ]
export abs

define max [ clone2 lt if [swap] [] drop ]
define min [ clone2 lt if [] [swap] drop ]

; a b bool-and => bool
define bool-and [ if [ ] [ drop #f ] ]
export bool-and
define bool-and? [ clone2 bool-and ] ; idk
export bool-and?
define bool-or [ if [ drop #t ] [ ] ]
export bool-or

define (type-dispatch list?) length [ list-length ]
define (type-dispatch bytevector?) length [ bytevector-length ]
export length

define (dispatch (and2? list? list?)) append [ list-append ]
define (dispatch (and2? string? string?)) append [ string-append ]
export append

; define value-hash [ drop #f bool-hash ] ; the default hash is that of false
; define (dispatch (bool?)) value-hash [ bool-hash ]
; define (dispatch (symbol?)) value-hash [ symbol-hash ]
; define (dispatch (string?)) value-hash [ string-hash ]
; define (dispatch (i64?)) value-hash [ i64-hash ]

define value->string [drop "<value>"]
define (type-dispatch string?) value->string []
define (type-dispatch bool?) value->string [if ["#t"] ["#f"]]
define (type-dispatch symbol?) value->string [symbol->string]
define (type-dispatch i64?) value->string [i64->string]
define (type-dispatch f64?) value->string [f64->string]
define (type-dispatch interpreter?) value->string [drop "<interpreter>"]
define (type-dispatch i64map?) value->string [drop "<i64map>"]
; define (dispatch (file-port?)) value->string [drop "<file-port>"]
; define (dispatch (embedded-file-port?)) value->string [drop "<embedded-file-port>"]

define (type-dispatch builtin?) value->string [
    drop "<builtin>"
    ; builtin-name false? if [ drop "<builtin>" ] [
    ;     value->string "<builtin " swap string-append ">" string-append
    ; ]
]

define (with-dynamics (value->string) type-dispatch list?) value->string [
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

export value->string

define print-value [ value->string print ]
export print-value
define println-value [ value->string println ]
export println-value

define port->string [ println #f error ]
define (type-dispatch file-port?) port->string [ file-port->string ]
define (type-dispatch embedded-file-port?) port->string [ embedded-file-port->string ]
export port->string
define read-port->list [ port->string read-string->list ]
export read-port->list

