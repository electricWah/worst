
; lua-declare: add Lua externs and maybe types

; lua-declare [
;     extern "print" (args (str))
;     extern "print" (args (str c) as printc)
;     extern "print" (args (str x y) as printxy)
;     extern "print" (args (str x y c) as printxyc)
;     extern "pget" (args (x y) return (pgetv))
; ]

define lua-declare [
    upquote const %declbody
    define extern [
        upquote const sname
        upquote const extargs
        #f make-place const %tupled
        [] make-place const %args
        sname string->symbol make-place const %defname
        0 make-place const %return
        [
            define tupled [ %tupled #t place-set drop ]
            define args [ %args upquote place-set drop ]
            define as [ %defname upquote place-set drop ]
            define return [ %return upquote place-set drop ]
            define pure [return #t]
            extargs eval
        ] eval

        %tupled place-get swap drop const tupled
        %args place-get swap drop const args
        %defname place-get swap drop const defname
        %return place-get swap drop const return

        tupled if [
            [ %targs list-eval list-reverse list-iterate [] ]
        ] [
            [
                quote quote defname
                quote const
                quote cil/new-id-name

                args list-length swap drop quote cil/expect-values
            ]
            list-eval
        ]
        const arglist

        [ sname quote .* return arglist ] list-eval
        const luaexpr

        [
            tupled if [ quote upquote quote const quote %targs ] []
            luaexpr quote cil/lua-expr 
        ] list-eval
        defname
        quote definition-add
        quote uplevel
        quote uplevel
        uplevel
    ]
    %declbody eval
]
export-name lua-declare

; vi: ft=scheme

