
; lua-declare: add Lua externs and maybe types

; lua-declare "print" (args (str))
; lua-declare "print" (args (str c) as printc)
; lua-declare "print" (args (str x y) as printxy)
; lua-declare "print" (args (str x y c) as printxyc)
; lua-declare "pget" (args (x y) return (pgetv))

; TODO tupled
; TODO return list with named return values

define lua-declare [
    upquote const sname
    upquote const extargs
    ; #f make-place const %tupled
    [] make-place const %args
    sname string->symbol make-place const %defname
    0 make-place const %return
    [
        ; define tupled [ %tupled #t place-set drop ]
        define args [ %args upquote place-set drop ]
        define as [ %defname upquote place-set drop ]
        define return [ %return upquote place-set drop ]
        define pure [return #t]
        extargs eval
    ] eval

    %defname place-get const defname drop

    %return place-get
    list? if [ list-length ] [ clone ] const retlen
    const return drop

    %args place-get
    list-length const arglen
    const args
    drop

    [ cil/lua-function-call ]
    retlen list-push
    sname list-push
    quote cil/expect-values/list list-push
    arglen list-push
    
    defname updo definition-add
]
export-name lua-declare

; vi: ft=scheme

