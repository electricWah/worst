
; list index list-ref! -> list value
define list-ref! [list-ref swap drop]

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

; [ k v ... ] list-iterate-pairs [ k v -> ... ]
define list-iterate-pairs [
    upquote quote %body definition-add
    while [list-empty? not] [
        list-pop const %k
        list-empty? if [] [
            list-pop const %v
            const %l
            %v %k %body
            %l
        ]
    ]
    drop
]

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

; list list-eval
; eval list in a temporary stack and return it as a new list
; combining no-op (if nothing in list is a symbol)
; with eval (for every 
define list-eval [
    const %list-eval-body
    [] stack-swap
    const %list-eval-stack
    %list-eval-body eval
    %list-eval-stack stack-swap
    list-reverse
]

; [v0 v1 v2 ... vN] i list-join -> [v0 i v1 i v2 i ... vN]
define list-join [
    const i
    list-empty? if [ ] [
        const l
        [ l list-iterate [i] drop ] list-eval
    ]
]

; [list] list-partition [el -> bool] -> [el : #f] [el : #t]
define list-partition [
    upquote quote %%partition definition-add
    [] [] dig
    list-iterate [
        %%partition if [bury swap dig list-push swap] [list-push]
    ]
    list-reverse swap
    list-reverse
]

; Remove the first N elements from the list and put them in their own list
; [ v0 ... vN vN+1 ... vM ] n list-split -> [ vN+1 ... vM ] [ v0 ... vN ]
define list-split [
    [] bury do-times [ list-pop bury swap dig list-push swap ]
    swap list-reverse
]

; l list-choose [elem -> elem | #f]
define list-choose [
    upquote const %filter
    const %list
    [ %list list-iterate [%filter eval false? if [drop] [] ] ] list-eval
]

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

; Sort a list using a mapping function and greater-than for its output
; this should be a general sort and also keep original order
; [v0 v1 ...] list-psort [v -> m] [m0 m1 -> m0>m1] -> [v0 v1 but sorted ...]
define list-psort [
    upquote quote %%list-psort-proj definition-add
    upquote quote %%list-psort-comp definition-add
    const %input-list

    ; not so fast method: first map over input to get proj -> elems
    map-empty
    %input-list list-iterate [
        const %v
        %v %%list-psort-proj
        map-get false? if [ drop [] ] []
        %v list-push map-set
    ]
    const %rlookup

    ; then sort the reverse lookup
    ; quicksort: take first element, split by comparator, [<= x] + x + [> x]
    define %%sort [
        list-empty? if [] [
            list-pop
            const %x
            list-partition [%x %%list-psort-comp swap drop]
            %%sort
            swap %%sort
            %x list-push
            list-append
        ]
    ]
    %rlookup map-keys swap drop
    %%sort
    list-reverse
    
    ; now the keys are sorted, extract the values again
    [] ; acc
    swap
    list-iterate [
        %rlookup swap map-get bury drop drop
        list-iterate [list-push]
    ]
]

; Sort a list using a given greater-than comparison function
; [v0 v1 ...] list-gtsort [a b -> a b {b > a?}] -> [v0 v1 but sorted ...]
define list-gtsort [
    upquote quote %%list-gtsort-body definition-add
    list-psort [] [%%list-gtsort-body]
]

export #t

; vi: ft=scheme

