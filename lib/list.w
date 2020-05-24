
; list index list-ref! -> list value
define list-ref! [list-ref swap drop]
export-name list-ref!

; [l...] list-iterate [ body ... ]
define list-iterate [
    upquote quote %body definition-add
    while [list-empty? not] [
        list-pop swap const %l
        %body
        %l
    ]
    drop
]
export-name list-iterate

; [l...] list-map [ body : l -> l' ] -> [l' ...]
define list-map [
    upquote quote %body definition-add
    [] swap ; acc
    while [list-empty? not] [
        list-pop swap const %l
        swap const %acc
        %body
        %acc swap list-push
        %l
    ]
    drop
    list-reverse
]
export-name list-map


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

