
'command-line '%command-line uplevel/named <root> %rename-definition
define command-line [
    %command-line
    "--worst-args"
    define inner [
        swap list-pop-head 2 dig equal?
        [] [
            1 dig drop
            inner
        ] %if
    ]
    inner
    drop drop
]

export-global command-line

;;; vi: ft=scheme

