
define parse-rule [
    quote^ symbol->string
    quote^ local rule-body

    parser-new-rule swapvar %rule

    define state-transition [
        quote^ combo? if [] [ combo-just ]
        quote^
        with-swapvar %rule [
            swap parser-set-state
            swap parser-accept-state
        ]
    ]

    define set-state [
        quote^
        with-swapvar %rule [
            swap parser-set-state
        ]
    ]

    define accept [
        quote^ eval
        char? if [ char-class-just ] [ ]
        combo? if [] [ combo-just ]

        with-swapvar %rule [
            swap parser-accept-input
        ]
    ]

    define type [
        quote^
        with-swapvar %rule [
            swap parser-set-token-type
        ]
    ]

    define %tag [
        with-swapvar %rule [
            swap parser-set-token-tag
        ]
    ]
    define tag [ quote^ %tag ]

    define anything [ combo-anything ]

    define start  [ with-swapvar %rule [ parser-start-token ] ]
    define finish [ with-swapvar %rule [ parser-finish-token ] ]
    define append [ with-swapvar %rule [ parser-append-token ] ]

    define prepend [
        quote^ eval
        with-swapvar %rule [ swap parser-prepend-datum ]
    ]

    rule-body eval

    [] %rule parser-save-rule

]

export-global parse-rule

;;; vi: ft=scheme

