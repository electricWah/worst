
;;; vi: ft=scheme

; Readline
; Customize prompt by overriding *read-line-prompt*
; Understands Ctrl-C and backspace

make-hash-table swapvar %special-chars
with-swapvar %special-chars [
    4 '%key-eof hash-table-set
    3 '%key-ctrl-c hash-table-set
    10 '%key-enter hash-table-set
    13 '%key-enter hash-table-set
    27 '%key-escape hash-table-set ; TODO escape key
    127 '%key-backspace hash-table-set
]
define %special-char? [
    with-swapvar %special-chars [
        swap hash-table-exists 2 dig
    ]
]
define %special-char-get [
    with-swapvar %special-chars [
        swap hash-table-get 2 dig
    ]
]

;;; *prompt-line*
; Also defined:
;; Everything in [ansi]
;; $input - get current input line (don't forget to print it)
;; $cursor - move cursor to editing position, assuming it is at the *beginning*
define *draw-read-line* [
    bold green fg "> " print-string reset
    cursor-save
    $input print-string
    cursor-restore
    $cursor
]

define/enclose read-line [
        %special-chars %special-char-get %special-char?
] [
    false swapvar %ctrl-d
    false swapvar %entered
    false swapvar %cancelled
    "" swapvar %reading-line
    define refresh-line [
        define $input [ swapvar-get %reading-line ]
        define $cursor [
            ; just go to end at the moment
            with-swapvar %reading-line [ string-length swap ]
            0 equal?! if [drop] [ cursor-forward ]
        ]
        ansi [
            0 cursor-to-column
            clear-line
            *draw-read-line*
            reset
        ]
        current-output-port output-port-flush drop
    ]
    define append-reading [
        with-swapvar %reading-line [
            swap string-append
        ]
    ]
    define %key-eof [ true %ctrl-d drop ]
    define %key-escape []
    define %key-enter [ true %entered drop ]
    define %key-ctrl-c [ true %cancelled drop ]
    define %key-backspace [
        with-swapvar %reading-line [
            "" equal?! if [] [ string-pop drop ]
        ]
    ]
    define read-char [
        standard-input-port
        0 6 make-u8vector
        ; port-read
        with-stty [raw -echo] [port-read]
        local %len
        %len u8vector-truncate
        swap drop
        cond [
            [%len 0 equal?! swap drop] [ %key-eof ]
            [%len 1 equal?! swap drop] [
                0 u8vector-get swap drop
                %special-char? if [
                    %special-char-get
                    swap drop eval
                ] [
                    u8->char char->string append-reading
                ]
            ]
            [
                0 u8vector-get
                27 equal?! swap drop if [
                    ; TODO escape sequence ignored
                    drop
                ] [
                    u8vector->string append-reading
                ]
            ]
        ]
    ]
    define read-line-loop [
        refresh-line
        read-char
        swapvar-get %entered [] [
            false cond [
                [swapvar-get %cancelled] []
                [swapvar-get %ctrl-d] []
                [not!]
            ]
            [read-line-loop] [] %if
        ] %if
    ]
    read-line-loop

    cond [
        [swapvar-get %cancelled] [ false ]
        [swapvar-get %ctrl-d] [ 'eof ]
        ["" %reading-line]
    ]
]

export-global *draw-read-line*
export-global read-line

