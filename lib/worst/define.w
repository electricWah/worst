
; define*-block? -> bool
; Currently in the modifier block in a define* ?
define define*-block? [#f]

; define* name [modifiers...]  [body...]
; modifiers... is a program that runs given body and name on the stack
define define* [
    upquote ; name
    upquote ; mods
    upquote ; body
    swap
    define define*-block? [#t]
    eval
    swap updo definition-add
]

; [body...] val static name
; Like const, but shared between invocations.
define static [
    define*-block? if [] [
        "static should be used in a define* modifier block" abort
    ]
    ; basically prepend [quote v const name] to body
    swap
    upquote list-push
    quote const list-push
    swap list-push
    quote quote list-push
]

; !!! Need alias
; lexical name
; Lexically scope the given definition name
define lexical [
    upquote const name
    ; [quote def quote name definition-add body ...]
    quote definition-add list-push
    name list-push
    quote quote list-push
    name definition-resolve swap drop list-push
    quote quote list-push
]

export define*-block?
export define*
export static
export lexical

; vi: ft=scheme


