
"Use syntax/attribute instead!" abort


; define f [ @[attribute ...] body... ]

; Redefines define to look for @ as the first item in the body.
; Evaluates the list immediately following the @ upon definition.
; TODO: Specify multiple @[attribute ...] forms.

define %define-attribute? [#f]
export %define-attribute?

; Monkey-patch definition-add to check for attributes.
[
    ; <definition-add> 'definition-add <definition-add> eval
    ; give definition-add its original definition again in this context
    clone quote definition-add swap eval

    ; interpreter-dump-stack
    define %%define-attribute []
    define %define-attribute? [#t]
    swap

    ; actual attribute stuff here
    ; TODO if the attr messes with the define body, it might ruin the @ parsing
    ; so for now it only deals with one @[] block
    list-empty? if [] [
        list-head equals? @ if [
            drop
            list-pop drop
            list-pop
            eval
        ] [drop]
    ]
    swap

    ; use the original definition-add in the parent context
    ; can't just uplevel it because that will just call this version again
    quote definition-add definition-get swap drop quote eval uplevel
]
; grab a copy of <definition-add> and put it in this def
quote definition-add definition-resolve swap drop list-push
; now actually define it
quote definition-add definition-add
export definition-add

; TODO make this work
define define-attribute [
    upquote name
    upquote body

    [quote %%define-attribute definition-resolve swap drop not not if []]
    [abort]
    " must be used as an attribute" name ->string string-append
    list-push
    [] swap list-push
    list-append
    interpreter-dump-stack

    swap quote definition-add
]
export define-attribute

; ; lexical-alias newname oldname
; Clone the definition referred to by oldname and define it as newname.
define lexical-alias [
    %define-attribute? if [
        updo upquote const newname
        updo upquote const origname
        ; [quote def quote name definition-add body ...]
        quote definition-add list-push
        newname list-push
        quote quote list-push
        origname definition-resolve swap drop list-push
        quote quote list-push
    ] [
        "lexical-alias must be used as an attribute" abort
    ]
]
export lexical-alias

; ; !!! Need alias
; ; lexical name
; ; Lexically scope the given definition name
define lexical [
    %define-attribute? if [] ["lexical must be used as an attribute" abort]
    upquote const name
    ; [quote def quote name definition-add body ...]
    quote definition-add list-push
    name list-push
    quote quote list-push
    name definition-resolve swap drop list-push
    quote quote list-push
]
export lexical

; [body...] val static name
; Like const, but shared between invocations.
define static [
    %define-attribute? if [] ["static must be used as an attribute" abort]
    ; basically prepend [quote v const name] to body
    swap
    upquote list-push
    quote const list-push
    swap list-push
    quote quote list-push
]
export static

; vi: ft=scheme

