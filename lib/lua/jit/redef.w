
; Turn a normal function into a definition
; func in-n lua-function-wrap-definition -> def
; stack_push_all(func(unpack(stack_pop(in_n))))
[
    [
        2 lua-expect-values
        const arglen
        const func
        [
            ; lua-eval-interpreter can't do dynamic stack lengths
            2 lua-expect-values
            const interp
            const istack

            ; interp:stack_pop(istack)
            define interp-stack-pop [
                interp
                quote stack_pop
                [istack] list-eval
                1
                lua-method-call
            ]

            ; local args = {}
            [] quote args lua-expr->variable const args

            ; for i = 0, arglen, 1 do
            1 arglen 1 [
                1 lua-expect-values drop ; drop i

                ; table.insert(args, interp:stack_pop()) end

                quote table.insert
                [args interp-stack-pop] list-eval
                0
                lua-funcall
            ]
            lua-for-iter

            ; local rets = {func(unpack(args))}
            [
                ; func(unpack(args))
                func
                [
                    ; unpack(args)
                    quote unpack [args] list-eval
                    #t lua-funcall
                ]
                list-eval
                #t lua-funcall
            ]
            list-eval
            quote rets lua-expr->variable
            const rets

            ; for i, r in ipairs(rets) do interp:stack_push(r) end
            quote ipairs [rets] list-eval #t lua-funcall
            2
            [
                2 lua-expect-values
                const ret
                const i

                interp
                quote stack_push
                [istack ret] list-eval
                0 lua-method-call
            ]
            lua-for-in
        ]
        lua-eval->function-expr
    ]
    lua-eval-interpreter
]
lua-eval->lua-function
quote lua-function-wrap-definition
definition-add
export-name lua-function-wrap-definition

; symbols must be injected because lua doesn't have access to them
; (technically it does, but only through require(), which may fail at runtime
;  if this is accidentally used outside of jit - so don't let it happen)

[
    [
        ; quote %expr
        2 lua-expect-values
        const value-symbol
        const expr-symbol
        [
            ; {[%expr] = prec, [value] = v}
            2 lua-expect-values
            const prec
            const v

            map-empty
            expr-symbol
            prec 10 " or " 8 lua-binop
            map-set

            value-symbol
            v map-set
        ]
        lua-eval->function-expr
    ]
    lua-eval-interpreter
]
lua-eval->lua-function
quote %expr
quote value
dig eval
const make-lua-expr
make-lua-expr quote lua-expr?/lua definition-add
make-lua-expr 1 lua-function-wrap-definition
quote make-lua-expr definition-add
export-name make-lua-expr

; ; define lua-expr? [ map? if [ quote %expr map-exists swap drop ] [ #f ] ]
; [
;     [
;         ; quote %expr
;         1 lua-expect-values const expr-symbol
;         [
;             ; return type(v) == "table" and v[quote %expr] ~= nil
;             1 lua-expect-values const v

;             quote type [v] list-eval #t lua-funcall
;             "table"
;             quote == 6 lua-binop

;             v
;             expr-symbol lua-index
;             lua-nil
;             quote ~= 6 lua-binop

;             " and " 7 lua-binop

;             v swap
;         ]
;         lua-eval->function-expr
;     ]
;     lua-eval-interpreter
; ]
; lua-eval->lua-function
; quote %expr swap eval
; const lua-expr?
; lua-expr? quote lua-expr?/lua definition-add
; lua-expr? 1 lua-function-wrap-definition
; quote lua-expr? definition-add
; export-name lua-expr?

; vi: ft=scheme

