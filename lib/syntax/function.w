
; function: "Normal" function definition and invocation syntax
; function f(a b) { a b add }
; f(1 2)

define function [
    upquote const func-name

    ; args body next-arg => + arg | #f
    define next-arg [
        swap list-empty? if [ swap #f ] [ list-pop rot swap rot rot #t ]
    ]

    ; [arg ...] [body...] push-next-arg => [...] [const arg body...]
    define push-args [
        tail-call
        next-arg if [
            list-push
            quote const list-push
            tail-call push-args
        ] [ swap drop ]
    ]

    upquote ; args
    upquote ; body
    push-args
    [upquote eval] swap list-append

    func-name updo definition-add
]
export function

; vi: ft=scheme


