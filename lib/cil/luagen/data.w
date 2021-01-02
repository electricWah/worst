
; Data types for CIL

define cil/expr? [ map? if [ quote %cil/expr map-exists swap drop ] [#f] ]

; value kind make-expr
; kind = #t : ->string as values
; kind = #f : precedence = 10
; kind = number : precedence ->
;   string: unprocessed
;   list: stringified with precedence
define cil/make-expr [
    import data/map
    const kind
    const value
    [ %cil/expr #t ] pairs->map
    quote value value map-set
    quote kind kind map-set
]

define cil/expr-value [ quote value map-get swap drop ]
define cil/expr-kind [ quote kind map-get swap drop ]

; expr inputs outputs set-expr-callable
define cil/set-expr-callable [
    const o const i
    quote inputs i map-set
    quote outputs o map-set
]

define cil/expr-callable-inputs [ quote inputs map-get swap drop ]
define cil/expr-callable-outputs [ quote outputs map-get swap drop ]

define cil/set-expr-tostring [ map-set-string ]

export-name cil/make-expr
export-name cil/expr?
export-name cil/expr-value
export-name cil/expr-kind

export-name cil/set-expr-callable
export-name cil/expr-callable-inputs
export-name cil/expr-callable-outputs

export-name cil/set-expr-tostring

; vi: ft=scheme

