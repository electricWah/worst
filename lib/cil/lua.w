
; Lua backend for cil
; Written in Lua so it can bootstrap

; Add Lua-only utility stuff into the EvalContext
; like assignments, return, etc
import cil/lua/base

; cil/eval->lua-chunk
import cil/lua/eval-chunk

; Lua function/method calls, unary/binary operations, etc
import cil/lua/expr

; if, while, etc
import cil/lua/control

; i [body...] cil/lua-interpreter-eval
; evaluates body but inputs are i:stack_pop() and outputs are i:stack_push()
import cil/lua/interpreter

; interp is the hipper, younger version of interpreter that is also pure Worst
; It defines interpreter-only stuff like interpreter-quote (aka upquote)
; and interpreter-call
; and define-lua-builtin, a wrapper which lets you use the extra bits
; and outputs a buitin function
; import cil/lua/interp

; Extern/declare Lua functions
; import cil/lua/declare

; Built-in syntax and functions for eval wrapper
; import cil/lua/stdlib

; eval->lua [ block ]
import cil/lua/eval

; Use eval->lua to define interpreter builtins.
; Also this is where define-lua-builtin happens
; also this is where define and syntax are born I guess
; import cil/lua/define

export-all

; vi: ft=scheme

