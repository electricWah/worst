
; Map syntax

; ; map . k -> map-get
; define . [ upquote map-get swap drop ]
; export-name .

; ; map v .= k v -> map-set
; define .= [ upquote upquote map-set ]
; export-name .=

; a b map-merge -> map
; for k, v in b do a[k] = b
define map-merge [
    const b
    b map-keys swap drop list-iterate [
        b swap map-get dig drop
        map-set
    ]
]
export-name map-merge

; [ k1 v1 k2 v2 ... ] pairs->map -> map
define pairs->map [
    map-empty swap
    while (list-empty? not) {
        list-pop swap
        list-pop swap
        const l
        map-set
        l
    }
    drop
]
export-name pairs->map

; map map->pairs -> [k1 v1 k2 v2 ...]
define map->pairs [
    map-keys
    [] swap list-iterate [
        swap const acc
        map-get
        acc
        swap list-push
        swap list-push
    ]
    swap drop
]
export-name map->pairs

; vi: ft=scheme

