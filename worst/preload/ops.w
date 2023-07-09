
; a b type is-type2 => a b bool
define is-type2 [
    const t
    const b
    t is-type if [ b t is-type ] [ b #f ]
]

define equal [ "TODO fix not equal" dump drop drop drop #f ]
define (dispatch (<i64> is-type2)) equal [ i64-equal ]
define (dispatch (<f64> is-type2)) equal [ f64-equal ]
define (dispatch (<string> is-type2)) equal [ string-equal ]
define (dispatch (<symbol> is-type2)) equal [ symbol-equal ]
define (dispatch (<bool> is-type2)) equal [ bool-equal ]

define equal? [ clone2 updo equal ]

define (dispatch (<i64> is-type2)) compare [ i64-compare ]
define (dispatch (<f64> is-type2)) compare [ f64-compare ]
define (dispatch (<string> is-type2)) compare [ string-compare ]

define le [compare 1 updo equal not]
define lt [compare -1 updo equal]
define ge [compare -1 updo equal not]
define gt [compare 1 updo equal]

; a <op> b => a bool
define equals? [ clone upquote updo eval updo equal ]
define lt? [ clone upquote updo eval lt ]
define le? [ clone upquote updo eval le ]
define gt? [ clone upquote updo eval gt ]
define ge? [ clone upquote updo eval ge ]

define (dispatch (<i64> is-type2)) add [ i64-add ]
define (dispatch (<f64> is-type2)) add [ f64-add ]
define (dispatch (<i64> is-type2)) sub [ i64-sub ]
define (dispatch (<f64> is-type2)) sub [ f64-sub ]
define (dispatch (<i64> is-type2)) mul [ i64-mul ]
define (dispatch (<f64> is-type2)) mul [ f64-mul ]
define (dispatch (<i64> is-type2)) div [ i64-div ]
define (dispatch (<f64> is-type2)) div [ f64-div ]

define (<i64> type-dispatch) negate [ i64-negate ]
define (<f64> type-dispatch) negate [ f64-negate ]

define abs [ lt? 0 if [negate] [] ]
define (<i64> type-dispatch) abs [ i64-abs ]
define (<f64> type-dispatch) abs [ f64-abs ]

define max [ clone2 lt if [swap] [] drop ]
define min [ clone2 lt if [] [swap] drop ]

; a b bool-and => bool
define bool-and [ if [ ] [ drop #f ] ]
define bool-and? [ clone2 bool-and ] ; idk
define bool-or [ if [ drop #t ] [ ] ]

define (<list> type-dispatch) length [ list-length ]
define (<bytevector> type-dispatch) length [ bytevector-length ]

define (dispatch (<list> is-type2)) append [ list-append ]
define (dispatch (<string> is-type2)) append [ string-append ]

define list-empty? [clone list-length 0 equal]
; define list-empty? [clone list-empty]
; 0 0 equal dump1

define value-hash [ drop #f bool-hash ] ; the default hash is that of false
define (<bool> type-dispatch) value-hash [ bool-hash ]
define (<symbol> type-dispatch) value-hash [ symbol-hash ]
define (<string> type-dispatch) value-hash [ string-hash ]
define (<bytevector> type-dispatch) value-hash [ bytevector-hash ]
define (<unique> type-dispatch) value-hash [ unique-hash ]
define (<i64> type-dispatch) value-hash [ i64-hash ]

define value->string [drop "<value>"]
define (<string> type-dispatch) value->string []
define (<bool> type-dispatch) value->string [if ["#t"] ["#f"]]
define (<symbol> type-dispatch) value->string [symbol->string]
define (<i64> type-dispatch) value->string [i64->string]
define (<f64> type-dispatch) value->string [f64->string]
define (<unique> type-dispatch) value->string [drop "<unique>"]
; define (<type> type-dispatch) value->string [
;     <string> type->unique value-meta-entry false? if [ drop "<type>" ] [
;         "<" swap ">" string-append string-append
;     ]
; ]
; define (<interpreter> type-dispatch) value->string [drop "<interpreter>"]
define (<lookup> type-dispatch) value->string [drop "<lookup>"]
; define (dispatch (file-port?)) value->string [drop "<file-port>"]
; define (dispatch (embedded-file-port?)) value->string [drop "<embedded-file-port>"]

define (<builtin> type-dispatch) value->string [
    drop "<builtin>"
    ; builtin-name false? if [ drop "<builtin>" ] [
    ;     value->string "<builtin " swap string-append ">" string-append
    ; ]
]

define (with-dynamics (value->string) <list> type-dispatch) value->string [
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

define print-value [ value->string print ]
define println-value [ value->string println ]

; not an op, but required just below
define feature-enabled? [
    upquote const name
    #f features-enabled list-iter [
        name equal if [ drop #t ] [ ]
    ]
]

define port->string [ println #f error ]
feature-enabled? fs-os if [
    define (<file-port> type-dispatch) port->string [ file-port->string ]
    quote port->string clone definition-resolve swap updo definition-add
] []
; define (<embedded-file-port> type-dispatch) port->string [ embedded-file-port->string ]

define read-port->list [ port->string read-string->list ]

