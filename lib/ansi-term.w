
;;; vi: ft=scheme

; ANSI terminal stuff

; ansi [
;   1 1 csi goto
;  
; ]

define ansi [
    quote^
    define esc [
        27 1 make-u8vector u8vector->string
        swap string-append
    ]
    define csi [
        ; code csi
        "[" esc
        swap string-append
        print-string
    ]

    define csi/n [
        local sym
        quote^ %n->list
        [datum-describe->string swap drop] list-map
        list-pop-tail swap
        "" [swap ";" string-append string-append] list-fold
        swap string-append
        sym string-append
        csi
    ]

    define sgr [ "m" csi/n 1 ]

    define cursor-up [ "A" csi/n 1 ]
    define cursor-down [ "B" csi/n 1 ]
    define cursor-forward [ "C" csi/n 1 ]
    define cursor-back [ "D" csi/n 1 ]
    define cursor-next-line [ "E" csi/n 1 ]
    define cursor-previous-line [ "F" csi/n 1 ]

    define cursor-move [ "H" csi/n 2 ]

    define clear-screen-from-cursor [ "0J" csi ]
    define clear-screen-to-cursor [ "1J" csi ]
    define clear-screen [ "2J" csi ]

    define clear-line-from-cursor [ "0K" csi ]
    define clear-line-to-cursor [ "1K" csi ]
    define clear-line [ "2K" csi ]

    define scroll-up [ "S" csi/n 1 ]
    define scroll-down [ "T" csi/n 1 ]

    define reset [ 0 sgr ]
    define bold [ 1 sgr ]
    define faint [ 2 sgr ]
    define italic [ 3 sgr ]
    define underline [ 4 sgr ]
    define slow-blink [ 5 sgr ]
    define fast-blink [ 6 sgr ]
    define reverse [ 7 sgr ]

    define black [0]
    define red [1]
    define green [2]
    define yellow [3]
    define blue [4]
    define magenta [5]
    define cyan [6]
    define white [7]

    ; r g b rgb6, each 0 .. 5
    define rgb6 [
        16 add
        swap 6 mul add
        swap 36 mul add
    ]

    define grey26 [ 232 add ]

    define fg [ 38 5 2 dig "m" csi/n 3 ]
    define bg [ 48 5 2 dig "m" csi/n 3 ]

    with-output-to-u8vector [ eval ] write-u8vector
    ; eval
]

; 1 1 csi-set-xy
; 1 1 csi-move-xy

export-global ansi


