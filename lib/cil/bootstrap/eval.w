
; eval-program
; set up an API, basically, for client code
;   cil/expect-value[s] etc
;   doesn't need data structures or anything because it can just be lists

; for integrating eval-program with other things like ->string
; define it as just a regular list and integrate it like that
; but for now just wrap it

; eval-fragment can be replaced with a wrapper to push/pop an arg list
; because it is functionally the same, only necessary in current luagen

define luavar [
    upquote const name

    [] swap list-push
    const value

    [
        name const %cil/new-id-name
        cil/new-id #t cil/make-expr
    ] eval
    const variable

    value [variable] list-eval
    #t cil/emit-assignment

    quote variable definition-resolve swap drop
    name updo definition-add
]

define => [
    upquote call const target
    [] swap list-push
    [target] list-eval
    #f cil/emit-assignment
]

define luadefine [
    upquote const %ldefname
    upquote const %ldefcallname
    upquote const %ldefbody

    %ldefbody
    [ %ldefname const %cil/new-id-name cil/new-id ] eval ->string
    cil/function-def

    [ cil/function-call ] swap list-push
    %ldefcallname
    updo definition-add

]

[
    ; gensym
    0 luavar gensym_v

    luadefine gensym interp/gensym [
        [ gensym_v 1 + ] cil/lua-expr => gensym_v
        gensym_v
    ]

    ; indentation
    0 luavar indentation_v

    luadefine indent interp/indent [
        [ [ indentation_v 1 + ] cil/lua-expr => indentation_v ]
        [ [ indentation_v 1 - ] cil/lua-expr => indentation_v ]
        cil/lua-if-then-else
    ]
    define indent+ [ #t indent ]
    define indent- [ #f indent ]

]
cil/eval-interpreter->builtin
quote cil/eval-program
definition-add

; vi: ft=scheme

