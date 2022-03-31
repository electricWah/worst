
import worst/interpreter
import syntax/cond

; doc "input-port reader-new -> reader"
define reader-new [

    ; TODO no buffering and just read

    const input-port

    define eof [ #f #f pause ] ; no args
    define syntax-error [ #f pause ] ; 1 arg
    define yield [ #t pause ] ; 1 arg

    define read-next [ next-char read-next/char ]
    define next-char [ input-port port-read-char swap drop ]
    define read-next/char [
        cond [
            [false?] [drop eof]
            [whitespace?] [drop]
            [equals? ";"] [
                while [ next-char cond [
                    [false?] [eof]
                    [equals? "\n"] [drop #f]
                    [#t] [drop #t]
                ] ] []
            ]
            [equals? "#"] [
                drop
                next-char cond [
                    [equals? "t"] [drop #t yield]
                    [equals? "f"] [drop #f yield]
                    [#t] [ ["unknown hash thingy"] swap list-push syntax-error ]
                ]
            ]
            [equals? "\""] [
                drop
                new-string-port
                while [ next-char cond [
                    [false?] [eof]
                    [equals? "\""] [drop port-read-all swap drop yield #f]
                    [equals? "\\"] [
                        drop next-char cond [
                            [false?] [eof]
                            [equals? "n"] [drop "\n" port-write-string]
                            [equals? "e"] [drop "\e" port-write-string]
                            [#t] [port-write-string]
                        ]
                        #t
                    ]
                    [#t] [ port-write-string #t ]
                ] ] []
            ]
            [#t] [ ["unknown char"] swap list-push syntax-error ]
        ]
    ]

    define reader-loop [ while [#t] [ read-next ] ]

    interpreter-empty
    interpreter-inherit-definitions
    quote reader-loop interpreter-call
    const interp

    interp
]

; reader reader-next -> reader ( read-val #t | #f )
define reader-next [
    quote read-next interpreter-call
    interpreter-run drop
    interpreter-stack-pop const ok
    interpreter-stack-pop const res
    res ok
]

export #t

