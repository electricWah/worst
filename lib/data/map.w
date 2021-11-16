
; Map syntax

; ; map . k -> map-get
; define . [ upquote map-get swap drop ]

; ; map v .= k v -> map-set
; define .= [ upquote upquote map-set ]

; a b map-merge -> map
; for k, v in b do a[k] = b
define map-merge [
    const b
    b map-keys swap drop list-iterate [
        b swap map-get dig drop
        map-set
    ]
]

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

; map map-iterate [ k v -> ...] -> ...
define map-iterate [
    upquote quote %map-iterate-body definition-add
    const %map-iterate-map
    %map-iterate-map map-keys swap drop
    list-iterate [
        %map-iterate-map swap map-get dig drop
        %map-iterate-body
    ]
]

; if the value exists already, replace it using the given code
; otherwise just map-set
; map k v map-replace [ map k v v-existing -> map ]
define map-replace [
    upquote quote %%map-upsert definition-add
    const %v
    map-exists if [
        map-get %v swap %%map-upsert
    ] [
        %v map-set
    ]
]

export #t

; vi: ft=scheme

