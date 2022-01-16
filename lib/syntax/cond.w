
doc [
    title "Choose a block of code to run based on the matching condition"
    description "It's basically an if/elseif/else block."
    usage "cond [ [-> bool] { if-true ... } ... ]"
    example "cond [[#f] [0] [#t] [1]]"
    example "6 cond [[equals? 2] [200] [equals? 6] [600] [#t] [-1]]"
    ; section eval
    ; tags []
]
define cond [
    upquote
    while [
        list-empty? if [ "cond: no matching condition" error ] []
        list-pop swap const %%conds
        eval
        not
        %%conds swap
    ] [
        list-pop drop
    ]
    list-pop swap drop
    eval
]

export cond

; vi: ft=scheme


