
; value variable name
; name get => value
; new-value name set

; maybe slightly slower
; define variable [
;     make-place const P
;     upquote const name
;     [
;         const P
;         define get [ place-get swap drop ]
;         define set [ swap place-set drop ]
;         define <- [
;             ; Big hack made of guesswork
;             ; Basically var <- expr...
;             ; so it works with functions, i.e. var <- cool(3)
;             quote evaluate quote uplevel quote uplevel quote uplevel uplevel
;             place-set drop
;         ]
;         upquote
;         definition-get false? if [
;             name "not recognised" abort
;         ] [
;             swap drop P swap eval
;         ]
;     ]
;     P list-push
;     name updo definition-add
; ]
; export-name variable

define variable [
    make-place const value
    upquote const name
    [
        upquote
        quote get equal? if [ drop drop place-get swap drop ] [
            drop quote set equal? if [ drop drop swap place-set drop ] [
                [] swap list-push "undefined" error
            ]
        ]
    ]
    value list-push

    name
    updo definition-add
]
export variable

; vi: ft=scheme


