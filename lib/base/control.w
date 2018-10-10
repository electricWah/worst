
;;; vi: ft=scheme

define while [
    quote^ list->definition local %body
    define %looper [
        %body eval-definition
        '%looper call-when
    ]
    %looper
]

define do/n [
    quote^ list->definition local %body
    define %loop [
        0 equal?! [drop] [
            %body eval-definition
            1 negate add
            %loop
        ] %if
    ]
    %loop
]

export-global do/n
export-global while

