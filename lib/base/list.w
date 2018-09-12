
;;; vi: ft=scheme

define list-empty? [
    list-length 0 equal? 2 negate dig drop drop
]

; [list...] (any -> any) list-map -> list'
define list-map [
    swap        ; list fn
    [] 2 ~dig   ; list fn acc
    define list-map-loop [
        list-empty? not swap drop   ; continue? list fn acc
        [
            list-pop-tail   ; elem list fn acc
            2 dig           ; fn elem list acc
            clone 3 ~dig    ; fn elem list fn acc
            eval            ; elem list fn acc
            3 dig swap      ; elem acc list fn
            list-push-head  ; acc list fn
            2 ~dig          ; list fn acc
            list-map-loop
        ] [] %if
    ]
    list-map-loop
    drop drop
]

; [list...] start-index [element -> element bool] -> found-index
define list-index-where/from [
    "Not implemented" abort
]

; n -> [0 .. n]
define iota [
    [] swap
    define inner [
        1 negate add
        clone
        2 ~dig
        list-push-head
        swap
        0 equal? not
        2 ~dig drop drop
        'inner call-when
    ]
    inner
    drop
]

export-global list-empty?
export-global list-map
export-global list-index-where/from
export-global iota

; list-iterate has-next take-next [body]
; ???

