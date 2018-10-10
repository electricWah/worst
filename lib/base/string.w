
;;; vi: ft=scheme

define u8->char [
    1 make-u8vector u8vector->string 0 string-get swap drop
]

export-global u8->char

