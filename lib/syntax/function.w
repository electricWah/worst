
; function: "Normal" function definition and invocation syntax
; function f(a b) { a b add }
; f(1 2)

define function [
    upquote const func-name

    ; args body next-arg => + arg | #f
    define next-arg [
        swap list-empty? if [ swap #f ] [ list-pop bury swap dig #t ]
    ]

    ; [arg ...] [body...] push-args => [...] [const arg body...]
    define push-args [
        while [next-arg] [
            list-push
            quote const list-push
        ]
        swap drop
    ]

    upquote ; args
    upquote ; body
    push-args
    [upquote eval] swap list-append

    func-name updo definition-add
]
export function

; vi: ft=scheme


