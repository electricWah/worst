
; [a b c ...] := expr
; or(?)
; => [a b c ...]

; [a] := expr
; and
; expr => [a]
; are like
; expr const a

define := [
    const args
    upquote updo evaluate

    ; value name adddef
    ; like value const name
    define adddef [
        swap [] swap list-push quote quote list-push
        swap quote definition-add updo uplevel
    ]

    ; [add-args ...] [named-arg ...] make-add-args
    ; => [quote named-arg updo adddef add-args ...]
    define make-add-args [
        tail-call
        list-empty? if [
            tail-call
            drop
        ] [
            tail-call
            swap
            [updo adddef] swap list-append
            swap list-pop swap rot list-push
            quote quote list-push
            swap
            make-add-args
        ]
    ]

    [] args make-add-args
    eval
]
export :=

; vi: ft=scheme


