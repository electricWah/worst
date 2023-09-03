
; a b type is-type2 => a b bool
define is-type2 [
    const t
    const b
    t is-type if [ b t is-type ] [ b #f ]
]

define equal [ value-equal ]
export equal

define equal? [ clone2 updo equal ]
export equal?

define compare [ value-compare ]
export compare

define le [compare 1 updo equal not]
define lt [compare -1 updo equal]
define ge [compare -1 updo equal not]
define gt [compare 1 updo equal]
export le
export lt
export ge
export gt

; a <op> b => a bool
define equals? [ clone upquote updo eval updo equal ]
define lt? [ clone upquote updo eval lt ]
define le? [ clone upquote updo eval le ]
define gt? [ clone upquote updo eval gt ]
define ge? [ clone upquote updo eval ge ]
export equals?
export le?
export lt?
export ge?
export gt?

define (dispatch (<i64> is-type2)) add [ i64-add ]
define (dispatch (<f64> is-type2)) add [ f64-add ]
define (dispatch (<i64> is-type2)) sub [ i64-sub ]
define (dispatch (<f64> is-type2)) sub [ f64-sub ]
define (dispatch (<i64> is-type2)) mul [ i64-mul ]
define (dispatch (<f64> is-type2)) mul [ f64-mul ]
define (dispatch (<i64> is-type2)) div [ i64-div ]
define (dispatch (<f64> is-type2)) div [ f64-div ]
export add
export sub
export mul
export div

define (<i64> type-dispatch) negate [ i64-negate ]
define (<f64> type-dispatch) negate [ f64-negate ]
export negate

define abs [ lt? 0 if [negate] [] ]
define (<i64> type-dispatch) abs [ i64-abs ]
define (<f64> type-dispatch) abs [ f64-abs ]
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

define (<list> type-dispatch) length [ list-length ]
define (<bytevector> type-dispatch) length [ bytevector-length ]
export length

define (dispatch (<list> is-type2)) append [ list-append ]
define (dispatch (<string> is-type2)) append [ string-append ]
export append

define value->string [drop "<value>"]
define (<string> type-dispatch) value->string []
define (<bool> type-dispatch) value->string [if ["#t"] ["#f"]]
define (<symbol> type-dispatch) value->string [value->string-display]
define (<i64> type-dispatch) value->string [value->string-display]
define (<f64> type-dispatch) value->string [value->string-display]
define (<unique> type-dispatch) value->string [drop "<unique>"]
define (<type-id> type-dispatch) value->string [
    <string> type-id->unique value-meta-entry false? if [ drop "<type>" ] [
        "<" swap ">" string-append string-append
    ]
]
define (<interpreter> type-dispatch) value->string [drop "<interpreter>"]
define (<i64map> type-dispatch) value->string [drop "<i64map>"]
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

export value->string

define print-value [ value->string print ]
export print-value
define println-value [ value->string println ]
export println-value

; not an op, but required just below
define feature-enabled? [
    upquote const name
    #f features-enabled list-iter [
        name equal if [ drop #t ] [ ]
    ]
]
export feature-enabled?

define port->string [ port-read->string swap drop ]
export port->string
define read-port->list [ port->string read-string->list ]
export read-port->list

