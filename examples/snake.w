
; Worst Snake game in ANSI terminal.

;;; vi: ft=scheme

"term/ansi.w" load-lib
"term/stty.w" load-lib

define read-key [
    standard-input-port
    0 10 make-u8vector
    port-read
    u8vector-truncate
    u8vector->string
    swap drop
]

27 u8->char local ESC

define escape-code? [
    0 string-get ESC equal?! swap drop
]

with-stty [raw] [
    ansi [
        true alternate-buffer
        cursor-hide
        "" print-string/n
        read-key
        false alternate-buffer
        cursor-show
    ]
]

