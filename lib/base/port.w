
define current-output-port [ standard-output-port ]

define print-string [
    string->u8vector
    current-output-port
    swap
    port-write
    drop
]

define newline [ "
" print-string ]

define print-string/n [ print-string newline ]

define write-u8vector [ current-output-port swap port-write drop ]

; TODO make this a new object and just redefine current-output-port
define with-output-to-u8vector [
    quote^ ; body
    0 0 make-u8vector swapvar %out
    define print-string [
        string->u8vector
        with-swapvar %out [
            swap u8vector-append
        ]
    ]
    define write-u8vector [
        with-swapvar %out [
            swap u8vector-append
        ]
    ]
    eval
    [] %out
]

export-global current-output-port
export-global print-string
export-global print-string/n
export-global newline
export-global write-u8vector
export-global with-output-to-u8vector

;;; vi: ft=scheme

