
; reader: read basic Worst syntax from somewhere

import syntax/struct

define-struct-type reader [
    fields [idx str datum state nest-data nest-info] [get set]
    literal-constructor make-reader
]

define reader-empty [
    make-reader [
        idx 0
        str ""
        datum #f
        state #f
        nest-data []
        nest-info []
    ]
]

lexical (set-reader-str)
define reader-read-string [ set-reader-str ]

define reader-next [
    ...
]

; [input-idx input-string current-datum state nest-stack-data nest-stack-info]
define reader-empty [[0 "" #f #f [] []]]
export-name reader-empty

; reader str reader-read-string -> reader
define reader-read-string [
    swap
    1 list-ref!
    dig string-append
    1 swap list-set
]
export-name reader-read-string

; reader reader-next -> reader data #t
;                    -> reader #f #f    ; end
;                    -> reader error #f ; failure
; only works on basic worst syntax
; lexical (make-reader)
define reader-next [
    import syntax/cond
    import syntax/assign
    import syntax/variable

    list-iterate [] => [input-idx input-str datum state nest-data nest-info]
    input-idx variable input-idx
    datum variable datum
    state variable state
    nest-data variable nest-data
    nest-info variable nest-info

    define reconstruct-reader [
        []
        nest-info get list-push
        nest-data get list-push
        state get list-push
        datum get list-push
        input-str list-push
        input-idx get list-push
    ]

    define next-char [
        input-str string-length
        input-idx get
        swap greater-than if [
            drop
            string-ref const c
            1 add input-idx set
            drop ; str
            c #t
        ] [
            drop drop drop
            #f
        ]
    ]

    define continue [#t]
    define set-state [ upquote state set ]
    define toplevel? [ nest-data get list-empty? swap drop ]

    define push-nesting [
        nest-info get
        swap list-push
        nest-info set

        nest-data get
        [] list-push
        nest-data set
    ]

    define pop-nesting [
        nest-info get
        list-pop
        swap nest-info set
        equal? if [drop drop] ["wrong closing nesting found" reader-abort]
        nest-data get
        list-pop
        swap
        nest-data set
        list-reverse
    ]

    define push-datum [
        nest-data get
        list-pop
        dig
        list-push
        list-push
        nest-data set
    ]

    define complete-datum [
        set-state #f
        toplevel? if [
            reconstruct-reader
            swap
            #t #f ; stop looping, return true
        ] [
            push-datum
            continue
        ]
    ]

    define set-data [
        datum get false? if [drop] [
            reconstruct-reader "set-data: would overwrite" abort
        ]
        datum set
    ]

    define take-data [
        datum get false? if [
            drop
            reconstruct-reader "take-data: nothing to take" abort
        ] []
        #f datum set
    ]
    define incomplete [ reconstruct-reader #f #f ]
    define reader-abort [ reconstruct-reader swap abort ]
    while [
        state get
        cond [
            [equals? #f] [
                drop
                next-char if [
                    ; reader debug
                    cond [
                        [char-whitespace?] [drop #t]
                        [equals? #\"] [
                            drop
                            "" set-data
                            set-state string
                            continue
                        ]
                        [equals? #\#] [
                            drop
                            set-state hash
                            continue
                        ]
                        [equals? #\;] [
                            drop
                            set-state comment-line
                            continue
                        ]
                        [equals? #\[] [
                            drop "[]" push-nesting
                            continue
                        ]
                        [equals? #\]] [
                            drop "[]" pop-nesting
                            complete-datum
                        ]
                        [equals? #\(] [
                            drop "()" push-nesting
                            continue
                        ]
                        [equals? #\)] [
                            drop "()" pop-nesting
                            complete-datum
                        ]
                        [#t] ["unimplemented" abort]
                    ]
                ] [ incomplete ]
            ]
            [equals? string] [
                drop
                next-char if [
                    cond [
                        [equals? #\"] [
                            drop
                            take-data
                            complete-datum
                        ]
                        [equals? #\\] [
                            set-state string/escape
                            continue
                        ]
                        [#t] [
                            take-data
                            swap string-push
                            set-data
                            continue
                        ]
                    ]
                ] [ incomplete ]
            ]
            [equals? string/escape] [
                drop
                next-char if [
                    take-data
                    swap string-push
                    set-data
                    set-state string
                ] [ incomplete ]
            ]
            [equals? hash] [
                drop
                ; read #t and #f
                next-char if [
                    cond [
                        [equals? #\t] [ drop #t complete-datum ]
                        [equals? #\f] [ drop #f complete-datum ]
                        [#t] ["unknown hash character" reader-abort]
                    ]
                ] [ incomplete ]
            ]
            [equals? comment-line] [
                drop
                next-char if [
                    equals? #\newline if [
                        drop set-state #f continue
                    ] [
                        drop continue
                    ]
                ] [ incomplete ]
            ]
            [#t] ["bad state" abort]
        ]
        ; reader debug
        ; interpreter-dump-stack
    ] []
]
export-name reader-next

; vi: ft=scheme

