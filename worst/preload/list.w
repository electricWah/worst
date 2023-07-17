
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

