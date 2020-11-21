
; define-attribute cil/luagen []

; Data types: expressions, variables, etc
import cil/luagen/data

; Set up an evaluation state including gensym, emit stack, and indentation
; and evaluate code within it
import cil/luagen/eval

; Turn expressions and statements into Lua code
import cil/luagen/emit

; Interpreter wrapper for eval which gives access to interpreter methods
; (e.g. quote) and generates a usable builtin
import cil/luagen/interp

; Lua expression eDSL
import cil/luagen/expr

; Primitive control structures such as if, loop, etc
import cil/luagen/control

export-all

; vi: ft=scheme

