
; define f [ @[attribute ...] body... ]

; Redefines define to look for @ as the first item in the body.
; Evaluates the list immediately following the @ upon definition.
; TODO: Specify multiple @[attribute ...] forms.

define define-attribute? [#f]
export define-attribute?

; TODO if the attr messes with the define body, it might ruin the @ parsing
; so first collect all of the @ forms from the head and only then eval them all
define %before-define [
    [] quote %before-define definition-add
    define define-attribute? [#t]
    list-empty? if [] [
        list-head equals? @ if [
            drop
            list-pop drop
            list-pop
            eval
        ] [drop]
    ]
]
export %before-define

; ; lexical-alias newname oldname
; Clone the definition referred to by oldname and define it as newname.
define lexical-alias [
    define-attribute? if [
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
    define-attribute? if [] ["lexical must be used as an attribute" abort]
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
    define-attribute? if [] ["static must be used as an attribute" abort]
    ; basically prepend [quote v const name] to body
    swap
    upquote list-push
    quote const list-push
    swap list-push
    quote quote list-push
]
export static

; vi: ft=scheme

