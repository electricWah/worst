
;;; vi: ft=scheme

define u8->char [
    1 make-u8vector u8vector->string 0 string-get swap drop
]

define char->string [ "" swap string-push ]

export-global u8->char
export-global char->string

