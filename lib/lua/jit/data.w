
; kind make-lua-expr
; kind = #t : ->string as values
; kind = #f : precedence = 10
; kind = number : precedence ->
;   string: unprocessed
;   list: stringified with precedence
; TODO look through here for other expressions and increment used count
define make-lua-expr [
    map-empty
    quote %expr dig false? if [ drop 10 ] [] map-set
    quote value dig map-set
]
export-name make-lua-expr

define lua-expr? [ map? if [ quote %expr map-exists swap drop ] [ #f ] ]
export-name lua-expr?

define lua-expr-precedence [ quote %expr map-get swap drop ]
export-name lua-expr-precedence

define lua-expr-unwrap [ quote value map-get bury drop drop ]
export-name lua-expr-unwrap

; Assignments declare and/or set variables.
; They exist to reduce the amount of code generated,
; e.g. assigning constants to single-use variables,
; needlessly assigning variables to other variables,
; or assigning variables that are never used again.
; They may or may not produce a statement,
; and since they aren't evaluated until the last moment,
; they already know how the values will be used in the future.
; var val make-lua-assignment lua-emit-statement
define make-lua-assignment [
    const val
    const var

    var
    quote assign-count
    map-get

    false? if [ drop 0 ] [ ]
    const assign-count

    assign-count
    1 add
    map-set

    drop

    map-empty
    quote %assignment assign-count map-set
    quote var var map-set
    quote val val map-set
]
export-name make-lua-assignment

define lua-assignment? [
    map? if [ quote %assignment map-exists swap drop ] [ #f ]
]
export-name lua-assignment?

; vi: ft=scheme

