
;;; vi: ft=scheme

define integer? [
    number? if [
        denominator 1 equal?! swap drop
    ] [false]
]

define number->fraction-pair [
    numerator swap denominator
    swap drop
]

export-global integer?
export-global number->fraction-pair


