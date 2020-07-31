
; Dicts are mutable maps
; here done with a place wrapping a map
; contrast with lib/dict.w which should probably go in the bin

define dict? [ place? if [ place-get map? swap drop ] [ #f ] ]

define dict->map [ place-get swap drop ]
define map->dict [ make-place ]
define dict-empty [ map-empty make-place ]
define dict-exists [
    const k
    place-get k map-exists const r
    drop drop
    k r
]

define dict-set [
    const v
    const k
    place-get
    k v map-set
    place-set
]

define dict-get [
    const k
    place-get
    k map-get const r
    drop
    drop
    k r
]

define dict-remove [
    const k
    place-get
    k map-remove
    place-set
]

define dict-keys [
    place-get
    map-keys const r
    drop
    r
]

export-name dict?
export-name dict-empty
export-name dict-exists
export-name dict-set
export-name dict-get
export-name dict-remove
export-name dict-keys
export-name dict->map
export-name map->dict

; vi: ft=scheme


