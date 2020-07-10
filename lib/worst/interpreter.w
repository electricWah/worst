
; An interpreter context.
; Doesn't have its own stack or driving loop.
; You could keep the stack in the stack itself, underneath the context,
; or perhaps in a variable.
; Try this for a loop:
; while [ ; ctx -> ctx
;   ctx-code-read
;   if [
;     symbol? if [
;       ctx-resolve if [
;         ; eval (builtin) or ctx-def-enter (definition)
;       ] [
;         ; undefined
;       ]
;     ] [
;       ; push to a stack somewhere
;     ]
;     #t
;   ] [ #f ]
; ] []
; 

import syntax/struct

define-struct-type ctx [
    fields (get set) [ defs body childs parent ]
    literal-constructor ctx-make
]

lexical (ctx-make ctx-defs-set)
define ctx-empty [
    ctx-make [
        body []
        childs []
        parent #f
    ]
    dict-empty ctx-defs-set
]

; TODO destroy the below

; define ctx-empty [
;     import list
;     list-quasiquote [
;         ~[dict-empty] ^[[] [] #f]
;     ]
; ]
; export-name ctx-empty

; define ctx-defs [ import list 0 list-ref! ]
; export-name ctx-defs
; define ctx-defs-set [ 0 swap list-set ]
; export-name ctx-defs-set
; define ctx-body [ import list 1 list-ref! ]
; export-name ctx-body
; define ctx-body-set [ 1 swap list-set ]
; export-name ctx-body-set
; define ctx-childs [ import list 2 list-ref! ]
; export-name ctx-childs
; define ctx-childs-set [ 2 swap list-set ]
; export-name ctx-childs-set
; define ctx-parent [ import list 3 list-ref! ]
; export-name ctx-parent
; define ctx-parent-set [ 3 swap list-set ]
; export-name ctx-parent-set

; ctx thing ctx-resolve -> result #t
;                    or -> #f ; couldn't resolve
; e.g.
; 6 ctx-resolve if [ "got" ] [ "no 6" ]
define ctx-resolve [
    const name
    ctx-defs name dict-exists if [
        dict-get
        bury drop drop
        #t
    ] [
        drop drop
        ctx-parent false? if [
            drop drop name #f
        ] [
            name ctx-resolve dig drop
        ]
    ]
]
export-name ctx-resolve

; ctx val name -> ctx
define ctx-def-add [
    const name
    const val

    ctx-defs name val dict-set
    ctx-defs-set
]
export-name ctx-def-add

; ctx [body ...] -> ctx
define ctx-def-enter [
    swap
    ctx-empty swap ctx-parent-set
    swap ctx-body-set
]
export-name ctx-def-enter

; ctx -> ctx val #t
;     -> ctx #f
define ctx-body-read [
    ctx-body list-empty? if [ drop #f ] [
        list-pop bury ctx-body-set
        swap
        #t
    ]
]
export-name ctx-body-read

; context-next
; ctx -> ctx val #t
;     -> ctx #f
define ctx-code-read [
    ; ctx -> ctx
    define ctx->child-innermost [
        while [
            ; ctx -> ctx child #t
            ;     -> ctx #f
            ctx-childs list-empty? if [ drop #f ] [
                ; ctx childs
                list-pop
                ; ctx childs child
                bury
                ; child ctx childs
                ctx-childs-set
                ; child ctx
                ctx-parent-set
                ; child
                #t
            ]
        ] [ ]
    ]

    while [
        ctx->child-innermost
        ctx-body-read if [ #t #f ] [
            ctx-parent false? if [ #f ] [
                swap drop #t
            ]
        ]
    ] [ ]
]
export-name ctx-code-read

; context-uplevel
; ctx -> ctx ok? ; should it error?
define ctx-into-parent [
    ctx-parent false? if [ ] [
        swap #f ctx-parent-set swap

        ; ctx-child-push : new-child ctx -> ctx
        ctx-childs
        dig list-push
        ctx-childs-set

        #t
    ]
]
export-name ctx-into-parent

import syntax/object
define-object-constructor make-interpreter [
    init [ctx-empty]
    method resolve [ %get swap ctx-resolve if [ swap drop #t ] [ drop #f ] ]
    method def [ %get bury ctx-def-add %set ]
    method enter [ %get swap ctx-def-enter %set ]
    method read [ %get ctx-body-read if [ swap %set #t ] [ #f ] ]
    method next [ %get ctx-code-read if [ swap %set #t ] [ drop #f ] ]
    method parent [ %get ctx-into-parent if [ %set #t ] [ #f ] ]

    method defined? [
        const name
        %get ctx-defs swap drop
        name dict-exists
        bury drop drop
    ]
]
export-name make-interpreter

; vi: ft=scheme

