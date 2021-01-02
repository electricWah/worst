
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

; list list-eval
; eval list in a temporary stack and return it as a new list
; combining no-op (if nothing in list is a symbol)
; with eval (for every 
define list-eval [
    const %list-eval-body
    [] interpreter-stack-swap
    const %list-eval-stack
    %list-eval-body eval
    %list-eval-stack interpreter-stack-swap
    list-reverse
]
export-name list-eval

; [v0 v1 v2 ... vN] i list-join -> [v0 i v1 i v2 i ... vN]
define list-join [
    const i
    list-empty? if [ ] [
        const l
        [ l list-iterate [i] drop ] list-eval
    ]
]
export-name list-join

; Remove the first N elements from the list and put them in their own list
; [ v0 ... vN vN+1 ... vM ] n list-split -> [ vN+1 ... vM ] [ v0 ... vN ]
define list-split [
    [] bury do-times [ list-pop bury swap dig list-push swap ]
    swap list-reverse
]
export-name list-split

; l list-choose [elem -> elem | #f]
define list-choose [
    upquote const %filter
    const %list
    [ %list list-iterate [%filter eval false? if [drop] [] ] ] list-eval
]
export-name list-choose

; n list-imake [ i -> el ] -> [ el0 el1 ... eln ]
define list-imake [
    const %n
    upquote const %body
    [] 0 while [%n ascending? swap drop] [
        const %i
        const %acc
        %i %body eval
        %acc swap list-push
        %i 1 add
    ]
    drop
    list-reverse
]
export-name list-imake

; vi: ft=scheme

