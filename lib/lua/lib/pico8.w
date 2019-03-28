
extern/export printh [invoke "printh" [string] []]
extern/export printh/clip ["@clip" swap invoke "printh" [string string] []]

extern/export stat: [
    syntax-read
    cond [
        [is? mem] [0]
        [is? cpu] [1]
        [is? clipboard] [4]
        [is? sfx] [TODO]
        [is? note] [TODO]
        [true] ["stat: unknown; accepts: mem cpu clipboard sfx note" abort]
    ]
    invoke "stat" [int] [float]
]

; define/export _draw [
;     syntax-read
;     ^' lua-define-function "_draw" [] void [eval]
; ]

extern/export print-at [invoke "print" [string int int] []]

extern/export cls [invoke "cls" [] []]
extern/export cursor [invoke "cursor" [int int] []]

extern/export btn: [
    syntax-read
    cond [
        [is? L] [0]
        [is? R] [1]
        [is? U] [2]
        [is? D] [3]
        [is? O] [4]
        [is? X] [5]
        [true] ["btn: unknown; accepts: L R U D O X" abort]
    ]
    invoke "btn" [int] [bool]
]

;;; vi: ft=scheme

