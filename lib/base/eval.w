
; symbol eval -> symbol call
; <definition> eval -> <definition> eval-definition ; TODO
; [code ...] eval -> list->definition eval-definition
define eval [
    define eval-list [
        list->definition eval-definition
    ]
    symbol?
    'eval-list 'call
    2 dig
    'swap call-when drop
    call
]

; do [body] -> body
define do [quote^ list->definition eval-definition]

; with-lexical-define name [definition] [body]
; define name [definition], but only lexically inside [body]
; basically with s/name/name-{gensym}/ on body before calling it
; aka, let
define with-lexical-define [
    quote^ gensym clone             ; name~ name~ name
    quote^ quote^ swap              ; def body name~ name~ name
    2 dig %define                   ; body name~ name
    [quote] 3 dig list-push-tail    ; [name] name~ body
    [equal? if] list-append
    [drop drop quote]
    3 dig list-push-tail list-push-tail
    [drop] list-push-tail
    list-map
    eval
]

; value local name
; name the top value so you can use it again later (involves cloning)
define local [
    quote^
    ['] 2 dig list-push-tail
    swap '%define uplevel
]

; value swapvar name
; defines 'name' as a function that swaps its current value
; with the one on top of the stack
; e.g.
; 8 swapvar eight
; 9 eight ; => 8, and now eight = 9
define swapvar [
    quote^
    swap make-place
    [swap place-swap swap drop] swap list-push-head
    swap '%define uplevel
]

define with-swapvar [
    quote^ local %%swapvar-name
    [] %%swapvar-name call
    quote^ eval
    %%swapvar-name call drop
]

define place->swapvar [
    quote^
    swap
    [swap place-swap swap drop] swap list-push-head
    swap '%define uplevel
]

; Clones swapvar value
; swapvar-get <swapvar-name> => value
define swapvar-get [
    quote^ local %name
    0 %name call
    clone %name call drop
]

; resolve-definition
define get-definition* [
    datum-describe->string print-string/n
    'defined? uplevel [
        "defined" print-string/n
        'get-definition uplevel
    ] [
        "not defined" print-string/n
        'get-definition* 'uplevel uplevel
    ] %if
]

; enclose [def ...] [body ...]
; Take every def in the list and put it at the front of body
; then turn body into a definition
; - basically, make a closure with the given names defined
; TODO walk up call stack to find definition
define enclose [
    quote^ ; defs
    quote^ swapvar body
    define build-defs [
        list-empty? [drop] [
            list-pop-tail local def
            ; def datum-describe->string print-string/n drop
            def 'resolve-definition uplevel
            ; datum-describe->string print-string/n
            with-swapvar body [
                'add-definition list-push-head
                def list-push-head
                'quote list-push-head
                swap list-push-head
            ]
            build-defs
        ] %if
    ]
    build-defs
    [] body
    ; datum-describe->string print-string/n
    list->definition
]

define define/enclose [
    quote^ local %defname
    'enclose uplevel
    %defname 'add-definition uplevel
]

export-global eval
export-global do
export-global with-lexical-define
export-global local
export-global swapvar
export-global with-swapvar
export-global swapvar-get
export-global place->swapvar
export-global enclose
export-global define/enclose

;;; vi: ft=scheme

