
; Lua backend for cil
; Written in Lua so it can bootstrap

; Add Lua-only utility stuff into the EvalContext
; like assignments, return, etc
import cil/lua/base

; cil/eval->lua-chunk
import cil/lua/eval

; Lua function/method calls, unary/binary operations, etc
import cil/lua/expr

; if, while, etc
import cil/lua/control

; eval -> string
; either as chunk or function
; import cil/lua/write

export-all

; vi: ft=scheme

