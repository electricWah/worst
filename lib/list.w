
import syntax/attributes

; [l...] list-iterate [ body ... ]
define list-iterate [
    import syntax/variable
    variable %l
    upquote quote %body definition-add
    while [%l get list-empty? not] [
        list-pop swap %l set
        %body
    ]
    drop
]
export list-iterate

; list-quasiquote( ^[literal-list] *[list-expr] ~[single-value-expr] ... )
define list-quasiquote [
    import syntax/variable
    [] variable %acc
    define %append [ %acc get swap list-append %acc set ]
    define ^ [ upquote %append ]
    define * [ upquote updo eval %append ]
    define ~ [ upquote updo eval [] swap list-push %append ]
    upquote eval
    %acc get
]
export list-quasiquote

; vi: ft=scheme
