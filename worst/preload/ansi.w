
define ansi [

    0 const black
    1 const red
    2 const green
    3 const yellow
    4 const blue
    5 const magenta
    6 const cyan
    7 const white

    define bright [ upquote eval 8 add ]

    define reset        [ "\e[0m" print ]
    define bold         [ "\e[1m" print ]
    define faint        [ "\e[2m" print ]
    define italic       [ "\e[3m" print ]
    define underline    [ "\e[4m" print ]
    define slow-blink   [ "\e[5m" print ]
    define fast-blink   [ "\e[6m" print ]
    define reverse      [ "\e[7m" print ]

    define cursor-save    [ "\e[s" print ]
    define cursor-restore [ "\e[u" print ]

    define erase-line-from-cursor [ "\e[0K" print ]
    define erase-line-to-cursor [ "\e[1K" print ]
    define erase-line [ "\e[2K" print ]

    define fg [
        value->string
        "\e[38;5;" swap string-append
        "m" string-append
        print
    ]

    define bg [
        value->string
        "\e[48;5;" swap string-append
        "m" string-append
        print
    ]

    updo current-defenv
    defenv-new-locals
    current-defenv
    defenv-merge-locals
    upquote swap value-set-defenv
    updo eval
]

