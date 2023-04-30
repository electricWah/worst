
; hashmap: hash values into an i64map with equality collision avoidance

; alias the type (necessary?)
<i64map> const <hashmap>
export <hashmap>

define make-hashmap [ make-i64map ]
export make-hashmap

; k v make-cell
define make-cell [ [] swap list-push swap list-push ]
define cell-key [ 0 list-get ]
define cell-val [ 1 list-get ]

; hashmap key v hashmap-insert => hashmap
define (with-dynamics (value-hash equal)) hashmap-insert [
    const v const k const hashmap
    ; linear probe
    k value-hash
    ge? 0 if [ -1 ] [ 1 ] const next-hash ; towards 0
    while [
        const h
        hashmap h i64map-get false? if [
            ; free slot
            drop h #f
        ] [
            cell-key k equal if [
                ; overwrite this slot
                h #f
            ] [
                ; try next slot
                h next-hash add #t
            ]
        ]
    ] []
    const kh
    hashmap kh
    k v make-cell
    i64map-insert
]
export hashmap-insert

; hashmap key hashmap-get => value|#f
define (with-dynamics (value-hash equal)) hashmap-get [
    const k const hashmap
    ; linear probe
    k value-hash
    ge? 0 if [ -1 ] [ 1 ] const next-hash ; towards 0
    while [
        const h
        hashmap h i64map-get false? if [
            ; not here
            drop #f #f
        ] [
            clone cell-key k equal if [
                ; found it
                cell-val #f
            ] [
                ; next slot
                drop h next-hash add #t
            ]
        ]
    ] []
]
export hashmap-get

