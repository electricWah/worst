
; Redefine all the cil stuff so builtins can be safely overridden

; Function object properties
; arity
; purity?
; name?
; dynamic-ness (only in interp builtin mode)
; original (list) source
; list of references to other function objects
; function() end string source
; - not necessary as a string as referenced names are not emitted yet
; compiled function (not necessary?)
; compiled interp builtin function

define-attribute cil/lua-builtin [
    before [ swap cil/eval-interpreter->builtin swap ]
]

; First, quicker quote
cil/lua-builtin
define quote [ %interp-quote ]
export-name quote

; cil/lua-builtin
cil/escaping-emit-mode
lexical (const)
define const [
    upquote const name
    [ name const cil/new-id-name cil/expect-value ] eval
    const x
    quote x definition-resolve swap drop
    name updo definition-add
]
export-name const

; cil/escaping-emit-mode
; lexical (clone)
; define clone [
;     [ quote v const cil/new-id-name cil/expect-value ] eval
;     clone
; ]
; export-name clone

; cil/escaping-emit-mode
; lexical (dig)
; define dig [
;     [ quote v const cil/new-id-name 3 cil/expect-values ] eval
;     dig
; ]
; export-name dig

; cil/lua-builtin
; define lua/getmetatable [
;     [ cil/expect-value const v "getmetatable" .* #t (v) ] cil/lua-expr
; ]
; export-name lua/getmetatable

cil/escaping-emit-mode
lexical (add)
define add [
    number? if [
        swap number? if [
            swap add #t
        ] [ swap #f ]
    ] [ #f ]
    if [] [ [+] cil/lua-expr ]
]
cil/escaping-emit-mode define mul [ [*] cil/lua-expr ]
export-name add
export-name mul

; lua/extern name inargs outargs
define lua/extern [
    upquote const name
    upquote const inargs
    upquote const outs

    [
        quote quote name quote const quote cil/new-id-name
        inargs quote cil/expect-values
    ] list-eval
    const arglist
    [ name ->string quote .* outs arglist ] list-eval const luaexpr

    [ luaexpr quote cil/lua-expr ] list-eval
    name
    updo definition-add
]
export-name lua/extern

lua/extern getmetatable 1 #t

cil/escaping-emit-mode
define iteri [
    cil/reenter-emit-mode [1 negate add]
    0 swap 1
    upquote
    cil/lua-for-iter
]
export-name iteri

cil/escaping-emit-mode
lexical (define)
define define [
    interpreter-dump-stack
    upquote const %dname
    upquote const %dbody

    %dbody
    %dname ->string
    cil/lua-function-def
    const %function

    cil/escaping-emit-mode
    lexical (%function)
    define d [ %function cil/lua-function-call ]
    interpreter-dump-stack

    quote d %dname definition-rename
    %dname definition-copy-up
]
export-name define

; vi: ft=scheme

