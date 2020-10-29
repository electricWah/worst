
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

export-name cil/make-expr
export-name cil/expr?
export-name cil/expr-value
export-name cil/expr-kind

; Emit state mirrors the interpreter eval stack
; - emitted statements usually go in the current state
; - you can go up levels in order to emit before the current state is done
; - ending a state re-emits everything into the parent
; this is so compilation can emit prerequisites before the current state
; e.g. inputs to an if/else statement,
; or a function call to something that hasn't been defined yet

define cil/make-emit-state [
    import data/map
    [
        %cil/emit-state #t
        parent #f
        ; children []
        statements []
    ] pairs->map
]

define cil/emit-state-enter-child [
    const parent
    cil/make-emit-state
    quote parent parent map-set
]

define cil/emit-state-emit-statement [
    const stmt
    quote statements map-get
    stmt list-push
    map-set
]

define cil/emit-state-statements [
    quote statements map-get list-reverse swap drop
]

define cil/emit-state-has-parent [ quote parent map-exists swap drop ]
define cil/emit-state-parent [ quote parent map-get swap drop ]
; this parent set-parent
define cil/emit-state-set-parent [
    quote parent swap map-set
]

define cil/emit-state-leave-child [
    cil/emit-state-statements const stmts
    cil/emit-state-parent
    swap drop
    stmts list-iterate [ cil/emit-state-emit-statement ]
]

export-name cil/make-emit-state
export-name cil/emit-state-enter-child
export-name cil/emit-state-leave-child
export-name cil/emit-state-emit-statement
export-name cil/emit-state-has-parent
export-name cil/emit-state-parent
export-name cil/emit-state-set-parent
export-name cil/emit-state-statements

; vi: ft=scheme

