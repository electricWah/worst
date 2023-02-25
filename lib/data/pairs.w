
; Treating lists as container of key/value pairs

; [ k1 v1 k2 v2 ... ] pairs-iterate [ k v -> ... ] -> 
define pairs-iter [
    upquote const body
    const list
    list list-length const len
    0 while (clone len lt) [
        const n
        list n list-get const k
        list n 1 add list-get const v
        k v body quote eval quote uplevel uplevel
        n 2 i64-add
    ] drop
]
export pairs-iter

; [ k1 v1 k2 v2 ... ] pairs-keys -> + [ k1 k2 ... ]
define pairs-keys [
    [] swap pairs-iter [ drop list-push ] list-reverse
]
export pairs-keys

; [ k1 v1 ... ] k pairs-get -> <vN where kN == k> | #f
define pairs-get [
    const %kq
    #f swap
    while [ list-empty? not ] [
        list-pop %kq equal?
        bury drop drop if [
            list-pop bury drop drop []
        ] [
            list-pop drop
        ]
    ]
    drop
]
export pairs-get

