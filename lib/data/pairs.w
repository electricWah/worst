
; Treating lists as container of key/value pairs

; [ k1 v1 k2 v2 ... ] pairs-keys -> + [ k1 k2 ... ]
define pairs-keys [ TODO
]

; [ k1 v1 k2 v2 ... ] pairs-iterate [ v k -> ... ] -> 
define pairs-iterate [
    upquote quote %pairs-iterate-body definition-add
    while [ list-empty? not ] [
        list-pop const %k
        list-pop const %v
        const %pairs-iterate-rest
        %v %k %pairs-iterate-body
        %pairs-iterate-rest
    ]
    drop
]

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

export #t

