
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

; [l...] list-map [ body : l -> l' ] -> [l' ...]
define list-map [
    upquote
    updo current-defenv value-set-defenv
    const %body
    [] swap ; acc
    list-iter [
        swap const acc
        %body eval
        acc swap list-push
    ]
    list-reverse
]
export list-map

; list list-find-index [element -> bool] -> i64|false
; (index of first element satisfying the function)
define list-find-first-index [
    upquote
    updo current-defenv value-set-defenv
    const filter

    const list
    list list-length const len

    0 while [
        lt? len if [
            const i
            list i list-get
            filter eval if [i #f] [i 1 add #t]
        ] [
            #f #f
        ]
    ] []
    false? if [drop drop #f] []
]
export list-find-first-index

; [a b b c d e e e f b a] list-deduplicate-sequential -> [a b c d e f b a]
; Uses equal to squash runs of equal values into one
define list-deduplicate-sequential [
    list-empty? if [[]] [
        [] swap ; acc
        clone 0 list-get const first ; marker
        first swap
        list-iter [
            equal? if [drop] [
                ; set marker to this, push to acc
                swap drop list-push
                clone 0 list-get
            ]
            ; could also list-get acc instead of keeping marker around
        ]
        drop ; marker
        ; first marker was never pushed to list
        list-reverse first list-push
    ]
]
export list-deduplicate-sequential

; v [ v1 v2 v3 ... ] -> (whether any vN == v)
define list-member [
    #f swap ; found
    while [list-empty? not] [
        swap drop ; found
        list-pop swap const l
        equal? if [#t []] [#f l]
        dig drop
    ]
    drop
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

; [list...] list-merge-sort-lt [ a b -> a b lt ] -> [sorted ascending list...]
define list-merge-sort-lt [
    upquote updo current-defenv value-set-defenv const compare

    define (with-dynamics (list-split-merge)) list-split-merge [
        clone list-length
        lt? 2 if [ drop ] [
            equals? 2 if [
                drop
                list-pop const b
                list-pop const a
                drop ; it's empty
                a b compare eval if [ a b ] [ b a ]
                [] swap list-push swap list-push
            ] [
                2 div list-split-at ; tail len will always be >= head
                list-split-merge const head
                list-split-merge const tail
                ; build up accumulator in reverse
                [] tail head
                while [
                    list-empty? not const h const head
                    list-empty? not const t
                    head
                    h t bool-and
                ] [
                    clone 0 list-get const h const head
                    clone 0 list-get const t const tail
                    h t compare eval if [
                        h list-push
                        tail head list-pop drop
                    ] [
                        t list-push
                        tail list-pop drop head
                    ]
                ]
                ; one of them is empty, append them both
                const head const tail
                list-reverse ; accumulator
                tail list-append head list-append
            ]
        ]
    ]
    list-split-merge
]

; list-a list-b list-set-differsection -> only-in-a only-in-b in-both
; calculates a - b, b - a, and a intersect b
; uses [compare], results are sorted, in-both uses items from list-a
define list-set-differsection [
    const list-b
    const list-a
    ; use gt: [equals? 1] below is reversed
    list-a list-merge-sort-lt [compare 0 gt] const list-a
    list-b list-merge-sort-lt [compare 0 gt] const list-b
    [] [] [] ; a b in-both
    list-a list-b
    while [
        list-empty? not const b
        swap list-empty? not const a
        swap
        a b bool-and
    ] [
        const list-b const list-a
        list-a 0 list-get const a
        list-b 0 list-get const b
        const in-both const in-b const in-a
        a b compare
        equals? 0 if [
            drop
            ; a = b, push to in-both
            in-a in-b
            in-both a list-push
            ; then drop from in-both lists
            list-a list-pop drop
            list-b list-pop drop
        ] [
            equals? 1 if [
                drop
                ; a < b, since they're sorted, a is not in list-b
                in-a a list-push
                in-b in-both
                ; drop from a
                list-a list-pop drop
                list-b
            ] [
                drop
                ; equals? -1
                ; ditto a > b
                in-a
                in-b b list-push
                in-both
                list-a
                list-b list-pop drop
            ]
        ]
    ]
    ; finally concatenate remainders
    const list-b const list-a
    const in-both const in-b
    list-a list-append ; in-a
    in-b list-b list-append
    in-both
]
export list-set-differsection

