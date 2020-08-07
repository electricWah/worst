
; A wrapper around the base codegen from other jit modules

define lswap [ 2 lua-expect-values swap ]
export-name lswap

; fname lfuncall retcount (args...) -> local r,... = fname(args)
define lfcall [
    string? if [ string->symbol ] []
    const fname
    upquote const retc
    upquote list-eval const args
    fname args retc lua-funcall
]
export-name lfcall

; objname lmcall method-name retcount (args ...)
define lmcall [
    const obj
    upquote const mname
    upquote const retcount
    upquote list-eval const args

    obj mname args retcount lua-method-call
]
export-name lmcall

define lvar [
    upquote const name
    name lua-expr->variable
    const real
    quote real name definition-rename
    name definition-copy-up
]
export-name lvar

; lfor-range i start end step [body]
; where start end step :: number or [expr]
define lfor-range [
    upquote const %varname
    upquote const %start
    upquote const %end
    upquote const %step
    upquote const %body

    ; body -> [ 1 lua-expect-values const varname body ]
    %body
    %varname list-push
    quote const list-push
    quote lua-expect-values list-push
    1 list-push
    const %body

    %start eval
    %end eval
    %step eval
    %body
    lua-for-iter
]
export-name lfor-range

; lfor-in [a b ...] [in-expr] [ body ]
define lfor-in [
    upquote const %vars
    upquote const %in-expr
    upquote const %body

    %vars list-length const %varlen drop

    ; body -> [ n lua-expect-values const b const a ... ]
    %body
    %vars list-reverse list-iterate [
        list-push
        quote const list-push
    ]
    quote lua-expect-values list-push
    %varlen list-push
    const %body

    %in-expr eval
    %varlen
    %body
    updo lua-for-in
]
export-name lfor-in

define lextvar [
    const name
    [] name list-push #f make-lua-expr
]
export-name lextvar

; with-interpreter iname [
;   iname pop -> v
;   n iname ref -> v
;   v iname push ->
;   body name iname definition-add ->
;   str iname ->symbol -> sym
; ]
define with-interpreter [
    upquote const %iname
    upquote const %body
    2 lua-expect-values
    const %interp
    const %istack

    [
        define pop [ %interp lmcall stack_pop 1 (%istack) ]
        define pop/type [
            const ty
            %interp lmcall stack_pop 1 (%istack ty)
        ]
        define push [
            const v
            %interp lmcall stack_push 0 (%istack v)
        ]
        define ref [
            1 lua-expect-values const n
            %interp lmcall stack_ref 1 (%istack n)
        ]
        define ref/type [
            const ty
            1 lua-expect-values const n
            %interp lmcall stack_ref 1 (%istack n ty)
        ]
        define ->symbol [
            1 lua-expect-values
            #t make-lua-expr const v
            ; use tiny hack in interpreter.lua
            ; interp.Symbol.new(v)
            %interp quote data.Symbol.new lua-dot
            lfcall 1 (v)
        ]
        define empty-list [
            %interp quote data.List.empty lua-dot lfcall 1 ()
        ]
        define def-add [
            2 lua-expect-values
            const name
            const def
            %interp lmcall define 0 (name def)
        ]
        define read [
            %interp lmcall body_read 1 ()
        ]
        define into-parent [ %interp lmcall into_parent 0 () ]
        define call-symbol [
            const s
            %interp lmcall call (%istack s)
        ]
        upquote call
    ]
    %iname definition-add

    %body eval
]
export-name with-interpreter

; vi: ft=scheme

