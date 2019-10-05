
; [a b c ...] := expr
; or(?)
; => [a b c ...]

; [a] := expr
; and
; expr => [a]
; are like
; expr const a

import syntax/attribute

; [add-args ...] [named-arg ...] %make-assign-consts
; => [quote named-arg updo adddef add-args ...]
define %make-assign-consts [
    while [list-empty? not] [
        swap
        [
            swap [] swap list-push quote quote list-push
            swap quote definition-add updo uplevel
        ]
        swap list-append
        swap list-pop swap bury list-push
        quote quote list-push
        swap
    ]
    drop
]

; expr ... => [name ...]
; Put values from the stack into names
; e.g. 1 2 3 => [a b c]
; now a = 1, b = 2, c = 3
lexical (%make-assign-consts)
define => [ [] upquote %make-assign-consts eval ]

lexical (%make-assign-consts)
define := [
    const args
    updo evaluate

    [] args %make-assign-consts
    eval
]

export-name :=
export-name =>

; vi: ft=scheme


