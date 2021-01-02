
; TODO first do expr as its own type

; redefine cil/eval
; also some helpers
; set up an API, basically, for client code
;   cil/expect-value[s] etc
;   doesn't need data structures or anything because it can just be lists

; for integrating eval-program with other things like ->string
; define it as just a regular list and integrate it like that
; but for now just wrap it

; can't define all eval parts at once
; so do them in their own modules:
; gensym, indentation, ...
; also each module is an installer for itself, since e.g.
; if gensym was just defined as a builtin it would reuse the same id counter

import cil/luagen/lualib
import cil/bootstrap/base

; gensym:
; cil/set-new-id-name cil/new-id-name cil/new-id
; [

;     define cil/set-new-id-name [
;         const %cil/new-id-name
;         quote %cil/new-id-name definition-copy-up
;     ]
;     define cil/new-id-name [ upquote updo cil/set-new-id-name ]
;     define cil/new-id [
; ]
; cil/eval-interpreter->builtin
; quote cil/install-gensym
; definition-add



[
    cil/interp-resolve
    [
        0 luavar indentation_v
        "    " luavar indentation_value
        lua-interp-definition indentation_value_const [ indentation_value ]
        "%cil/indentation-value" cil/interp-define

        ; lua-definition indent [
        ;     [ [ indentation_v 1 + ] cil/lua-expr => indentation_v ]
        ;     [ [ indentation_v 1 - ] cil/lua-expr => indentation_v ]
        ;     cil/lua-if-then-else
        ; ]
        ; const %interp/indent
        ; %interp/indent lua-callable interp/indent
        ; define indent+ [ #t interp/indent ]
        ; define indent- [ #f interp/indent ]

        lua-definition ind [ [ indentation_v 1 + ] cil/lua-expr => indentation_v ]
        lua-definition ind [ [ indentation_v 1 - ] cil/lua-expr => indentation_v ]
        "cil/indent<" cil/interp-define
        "cil/indent>" cil/interp-define
    ] []
    cil/lua-if-then-else
]
cil/eval-interpreter->builtin
quote cil/install-indent
definition-add

; [
;     ; gensym
;     0 luavar gensym_v

;     lua-definition gensym [
;         [ gensym_v 1 + ] cil/lua-expr => gensym_v
;         gensym_v
;     ]
;     lua-callable interp/gensym

;     ; indentation
;     0 luavar indentation_v
;     "    " luavar indentation_value

; ;     lua-definition indent [
; ;         [ [ indentation_v 1 + ] cil/lua-expr => indentation_v ]
; ;         [ [ indentation_v 1 - ] cil/lua-expr => indentation_v ]
; ;         cil/lua-if-then-else
; ;     ]
; ;     const %interp/indent
; ;     %interp/indent lua-callable interp/indent

; ;     define indent+ [ #t interp/indent ]
; ;     define indent- [ #f interp/indent ]

;     lua-definition ind [ [ indentation_v 1 + ] cil/lua-expr => indentation_v ]
;     lua-definition ind [ [ indentation_v 1 - ] cil/lua-expr => indentation_v ]
;     lua-callable cil/indent<-builtin
;     lua-callable cil/indent>-builtin

;     lua-definition new_var [
;         cil/expect-value const name
;         interp/gensym lua-tostring const gsi
;         [ name gsi .. ] cil/lua-expr
;         cil/interp-string->symbol
;     ]
;     lua-callable interp/new-var

; ]
; cil/eval-interpreter->builtin
; quote cil/eval-program
; definition-add

; vi: ft=scheme

