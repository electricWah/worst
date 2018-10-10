
;;; vi: ft=scheme

; bool if-true if-false %if
define %if [
    swap 2 dig      ; cond true-arm false-arm
    ; either
    ; true T F -> F T drop -> T
    ; false T F -> T F drop -> F
    'swap call-when drop
    list->definition eval-definition
]

; bool if [true-arm] [false-arm] -> (as expected)
define if [quote^ quote^ %if]

; value cond [arms ...]
; cond [
;  [one] ["It was one"]
;  ["two"] ["It was the string 'two'"]
;  ; default
;  ["It was something else aaaag"]
; ]

; arms %cond
define %cond [
    list-length
    0 equal? [] [
        drop
        1 equal? [drop drop list-pop-tail swap drop eval] [
            drop drop
            list-pop-head swap list-pop-head
            swapvar %cond-do-if
            swapvar %cond-body
            eval
            not [
                drop
                [] %cond-body
                %cond
            ] [
                drop
                [] %cond-do-if
                'eval uplevel
            ] %if
        ] %if
    ] %if
]

define cond [quote^ %cond]

define %case [
    list-length
    0 equal? [] [
        drop
        1 equal? [drop drop list-pop-tail swap drop eval] [
            drop drop
            list-pop-head
        ]
    ]
]

define not [ false equal? swap drop ]
define not! [ not swap drop ]
define equal?! [ equal? swap drop ]
define and! [ and local ok drop drop ok ]

export-global %if
export-global if
export-global %cond
export-global cond
export-global not
export-global not!
export-global equal?!
export-global and!

