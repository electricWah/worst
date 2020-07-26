
; [v0 v1 ...] lua-declare-vars -> local v0, v1, ...
define lua-declare-vars [
    list-length 0 equal? not bury drop drop if [
        const vars
        ["local " vars ", " list-join list-iterate []] list-eval
        #f make-lua-expr
        lua-emit-statement
    ] [drop]
]
export-name lua-declare-vars

; expr name lua-expr->variable -> local name = expr; name
define lua-expr->variable [
    const name
    1 lua-expect-values const expr
    name lua-new-var
    const var
    var expr make-lua-assignment lua-emit-statement
    var
]
export-name lua-expr->variable

define lua-nil [ quote nil #t make-lua-expr ]
export-name lua-nil

; lhs rhs op prec lua-binop -> expr "lhs op rhs"
define lua-binop [
    const prec
    const op
    2 lua-expect-values
    lua-expr? if [] [#t make-lua-expr] const rhs
    lua-expr? if [] [#t make-lua-expr] const lhs

    [ lhs op rhs ] list-eval
    prec make-lua-expr
]
export-name lua-binop

; a b lua-dot -> a.b
define lua-dot [ quote . 0 lua-binop ]
export-name lua-dot

; val op prec lua-unop -> expr "op val"
define lua-unop [
    const prec
    const op
    1 lua-expect-values
    const val

    [ op val ] list-eval
    prec make-lua-expr
]
export-name lua-unop

; obj key lua-index -> expr "obj[key]"
define lua-index [
    2 lua-expect-values
    const key
    const obj
    [ obj "[" key "]" ] list-eval
    0 make-lua-expr
]
export-name lua-index

; multivalue, impure-safe version
; expr args retcount lua-funcall -> local r1, r2, ... = expr(args); r1, r2, ...
; single-value version
; expr args #t lua-funcall -> expr(args)
define lua-funcall [
    import list
    const rcount
    const args
    const func

    [ func "(" args ", " list-join list-iterate [] ")" ] list-eval
    0 make-lua-expr
    const fcall

    ; pure single expression
    rcount #t equal? bury drop drop if [
        fcall
    ] [
        ; multival assign
        rcount 0 equal? bury drop drop if [
            ; Don't do any assigning
            fcall lua-emit-statement
        ] [
            ; Make enough return values
            rcount list-imake [
                drop
                quote v lua-new-var
            ]
            ; interpreter-dump-stack
            const rets

            ; Emit the returns assignment as a statement
            [ "local " rets ", " list-join list-iterate [] " = " fcall ]
            list-eval
            #f make-lua-expr
            lua-emit-statement

            ; Drop the values on the stack
            rets list-iterate []
        ]
    ]
]
export-name lua-funcall

; obj name args retcount lua-method-call -> obj:name(args)
define lua-method-call [
    const rcount
    const args
    quote : 0 lua-binop
    args rcount lua-funcall
]
export-name lua-method-call

; vi: ft=scheme

