

; do [body] -> body
define do [quote^ eval]

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

export-global do
export-global with-lexical-define
export-global local
export-global swapvar
export-global with-swapvar
export-global place->swapvar

;;; vi: ft=scheme

