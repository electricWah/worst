
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

; define add [ quote + binop ]
; define compileme [ 1 2 add ]
; function compileme()
;   -> add says "not emitted yet" and compiles into
;       function add(a, b) return a + b end
;   and emits in parent

; function add(a, b) { a + b }
; function compileme() { add(1, 2) }


; further work:
; requires some kind of compilation state e.g.
; function compile_me(st)
;   real_deps = []
;   for d in my_deps do
;       real_deps[d] = st:com(d)
;   local state = st:new_inner_state()
;   state:emit("function()...")
;   state:emit("local x = " .. real_deps["foo"] .. "(0)")
; end
; which replaces the ad-hoc dynamic cil/ functions
; and can just output whatever it wants, really

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

; TODO iteri

; define extern

; define foo [ body ]
; cil/override-eval-mode
; define define [
;     upquote const %dname
;     upquote const %dbody
; ]

; vi: ft=scheme

