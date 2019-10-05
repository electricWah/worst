
import doc

documentation [
    title "Choose a block of code to run based on the matching condition"
    description "It's basically an if/elseif/else block."
    usage "cond [ [-> bool] { if-true ... } ... ]"
    example "cond [[#f] [0] [#t] [1]]"
    example "6 cond [[equals? 2] [200] [equals? 6] [600] [#t] [-1]]"
    ; section eval
    ; tags []
]
define cond [

    import syntax/variable

    upquote variable %conds
    while [
        %conds get
        list-empty? if [ "cond: no matching condition" abort ] []
        list-pop swap %conds set
        eval
        not
    ] [
        %conds get
        list-pop drop %conds set
    ]
    %conds get list-pop swap drop
    eval
]

export-name cond

; vi: ft=scheme


