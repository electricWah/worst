
; read all profiled functions and output dtrace-formatted to stderr
define profile-dtrace [
    [] make-place const %%profile-trace-log
    modifying-body (%%profile-trace-log list-push)
    define write-profile-trace-enter-call [
        place-get dig list-push place-set drop
    ]
    modifying-body (%%profile-trace-log list-push)
    define write-profile-trace-clock [
        place-get dig list-push place-set drop
    ]
    
    upquote const %%dtrace-body
    profile (trace #t)
    define profile-toplevel [ %%dtrace-body updo eval ]
    profile-toplevel

    define write-str [
        "\n" string-append
        current-error-port swap port-write-string
        drop
    ]
    define write-stack-entry [
        "[^+>]" string-global-matches "" string-join
        " worst`" swap string-append write-str
    ]
    define write-ns [
        1000000 mul
        ; 1 modulo
        0.5 add clone 1 modulo negate add ; round
        value->string write-str
    ]
    define write-clear [ "" write-str ]

    [] ; current stack

    %%profile-trace-log place-get swap drop list-reverse
    list-iterate [
        swap const stack
        number? if [
            stack list-iterate [ write-stack-entry ]
            write-ns
            write-clear

            stack list-pop drop
        ] [
            stack swap list-push
        ]
    ]
    drop

]
export-name profile-dtrace

; Not accurate
; as the interpreter cannot trace worst function execution time correctly

define trace-to-port [
    const %%trace-port
    %%trace-port interpreter-set-trace-port
    upquote updo eval
    #f interpreter-set-trace-port
    %%trace-port
]
export-name trace-to-port

define trace-to-file [ open-output-file updo trace-to-port port-close ]
export-name trace-to-file

; vi: ft=scheme


