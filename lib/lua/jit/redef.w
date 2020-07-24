
[ with-interpreter i [ i read i push ] ]
lua-eval->lua-function
quote quote
definition-add
export-name quote

[
    with-interpreter i [
        i into-parent
        i read
        i push
    ]
]
lua-eval->lua-function
quote upquote
definition-add
export-name upquote

[
    with-interpreter i [
        i pop const c

        [ with-interpreter ii [ c i push ] ]
        lua-eval->function-expr

        i read
        i def-add
    ]
]
lua-eval->lua-function
quote const
definition-add
export-name const

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
            with-interpreter interp [
                ; local args = {}
                [] lvar args

                ; for i = 1, arglen, 1 do
                lfor-range i 1 [arglen] 1 [
                    ; 1 lua-expect-values drop ; drop i

                    ; table.insert(args, interp:stack_pop()) end
                    "table.insert" lfcall 0 (args interp pop)
                ]
                ; lua-for-iter

                ; local rets = {func(unpack(args))}
                [
                    ; func(unpack(args))
                    func lfcall #t ("unpack" lfcall #t (args))
                ]
                list-eval
                lvar rets

                ; for i, r in ipairs(rets) do interp:stack_push(r) end
                lfor-in [i ret] ["ipairs" lfcall #t (rets)] [
                    ret interp push
                ]
            ]
        ]
        lua-eval->function-expr
    ]
    lua-eval-interpreter
]
lua-eval->lua-function
quote lua-function-wrap-definition
definition-add
; export-name lua-function-wrap-definition


; [
;     with-interpreter interp [
;         interp pop const prec
;         interp pop const v

;         dict-empty

;         "value" interp ->symbol v dict-set

;         "%expr" interp ->symbol
;         prec 10 " or " 8 lua-binop
;         dict-set
;         ; TODO give this a metatable
;     ]
; ]
; lua-eval->lua-function
; quote make-lua-expr definition-add
; export-name make-lua-expr

[
    with-interpreter interp [
        dict-empty
        lvar metatable

        [
            with-interpreter i [
                i pop const prec
                i pop const v

                dict-empty

                quote prec
                prec 10 " or " 8 lua-binop
                dict-set

                quote value
                v dict-set

                quote declared #t dict-set

                const e

                "setmetatable" lfcall 1 (e metatable)

                i push
            ]
        ]
        lua-eval->function-expr
        "make-lua-expr" interp ->symbol
        interp def-add

        [
            1 lua-expect-values const s
            "getmetatable" lfcall 1 (s)
            metatable
            "==" 6 lua-binop
        ]
        lua-eval->function-expr
        lvar isexpr

        [
            with-interpreter i [
                1 i ref const s
                isexpr lfcall 1 (s)
                i push
            ]
        ]
        lua-eval->function-expr
        "lua-expr?" interp ->symbol
        interp def-add

        [
            with-interpreter i [
                metatable i pop/type
                quote value lua-dot
                i push
            ]
        ]
        lua-eval->function-expr
        "lua-expr-unwrap" interp ->symbol
        interp def-add

        [
            with-interpreter i [
                1 metatable i ref/type
                quote prec lua-dot
                i push
            ]
        ]
        lua-eval->function-expr
        "lua-expr-precedence" interp ->symbol
        interp def-add

        [
            with-interpreter i [
                1 metatable i ref/type
                quote declared lua-dot
                #t "==" 6 lua-binop
                i push
            ]
        ]
        lua-eval->function-expr
        "lua-expr-declared?" interp ->symbol
        interp def-add

        [
            with-interpreter i [
                "boolean" #t make-lua-expr
                i pop/type const d
                metatable i pop/type
                clone
                quote declared lua-dot
                d make-lua-assignment lua-emit-statement
                i push
            ]
        ]
        lua-eval->function-expr
        "lua-expr-set-declared" interp ->symbol
        interp def-add

    ]
]
lua-eval->lua-function
eval
export-name lua-expr?
export-name make-lua-expr
export-name lua-expr-precedence
export-name lua-expr-unwrap
export-name lua-expr-declared?
export-name lua-expr-set-declared

; ; define lua-expr? [ dict? if [ quote %expr dict-exists swap drop ] [ #f ] ]
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

