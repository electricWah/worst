
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

export-global current-output-port
export-global print-string
export-global print-string/n
export-global newline
export-global write-u8vector

;;; vi: ft=scheme

