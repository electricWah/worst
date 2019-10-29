
; list index list-ref! -> list value
define list-ref! [list-ref swap drop]
export-name list-ref!

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
export-name list-iterate

; [a1 a2 ...] [b1 b2 ...] list-zip -> [[a1 b1] [a2 b2] ...]
define list-zip [
    import syntax/variable
    [] variable acc
    while [list-empty? not] [
        list-pop const a
        swap
        list-pop const b
        [] a list-push b list-push
        acc get swap list-push acc set
        swap
    ]
    drop drop acc get list-reverse
]
export-name list-zip

; [l...] list-map [ body : l -> l' ] -> [l' ...]
define list-map [
    import syntax/variable
    upquote const body
    [] variable %acc
    list-iterate [
        body eval
        %acc get swap list-push %acc set
    ]
    %acc get list-reverse
]
export-name list-map

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
export-name list-quasiquote

; vi: ft=scheme

