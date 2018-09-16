
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

; Like take-definition but keeps it there
define get-definition [
    clone 'take-definition uplevel
    clone 2 dig 'add-definition uplevel
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
            def 'get-definition 'uplevel uplevel
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
    [] body list->definition
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
export-global place->swapvar
export-global get-definition
export-global enclose
export-global define/enclose

;;; vi: ft=scheme

