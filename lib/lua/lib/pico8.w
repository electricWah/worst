
define/export printh [extern "printh" [string] []]
define/export printh/clip ["@clip" swap extern "printh" [string string] []]

define/export stat [
    ^' cond [
        [is?! mem] [1]
        [true] ["stat: unknown" abort]
    ]
    extern "stat" [int] float
]

define/export _draw [ ^' lua-define-function "_draw" [] void [eval] ]

;;; vi: ft=scheme


