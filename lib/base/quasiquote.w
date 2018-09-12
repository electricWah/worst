
;;; vi: ft=scheme

; quasiquote [list ...]
; quasiquote, unquote, and unquote-splicing
define quasiquote [
    ; debug "<QQ"
    quote^ %quasiquote
    ; debug "QQ>"
]
define %quasiquote [
    swapvar qq-in
    define next [[] qq-in list-pop-head swap qq-in drop]
    [] swapvar qq-acc
    define accum [[] qq-acc swap list-push-tail qq-acc drop]
    define accum-list [[] qq-acc swap list-append qq-acc drop]
    define qq-run [
        [] qq-in list-empty? swap qq-in drop [
            [] qq-acc
        ] [
            next cond [
                [list?] [
                    ; debug "+ QQ list"
                    drop %quasiquote accum
                    ; debug "- QQ list"
                ]
                ['unquote equal? swap drop] [
                    ; debug "+ QQ unquote"
                    drop drop
                    next eval
                    accum
                    ; debug "- QQ unquote"
                ]
                ['unquote-splicing equal? swap drop] [
                    ; debug "+ QQ splice"
                    drop drop
                    next eval
                    accum-list
                    ; debug "- QQ splice"
                ]
                [
                    ; debug "+ QQ default"
                    accum
                    ; debug "- QQ default"
                ]
            ]
            qq-run
        ] %if
    ] qq-run
]

export-global quasiquote
export-global %quasiquote

