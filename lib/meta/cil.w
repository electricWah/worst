
; meta/cil: turn worst into some other language
; Codegen Instruction Language
; Compiler/Interpreter Library
; Cornish Intermediate Linguini

; Compile Worst programs into other languages
; Worst -(cil generator)-> { IL <-> opt } -(target generator)-> target

; Random notes from before implementation:

; The CIL generator takes regular Worst code,
; performs a few correctness checks (stack size pre/post-conditions etc),
; and spits out IL.

; IL: intermediate language
; uses almost exclusively definitions implemented by the target generator
; The most basic control structures and data operations, expressed in
; as neutral way as possible:
; side effecting ops on their own
; cil/cond, cil/while, cil/loopiter, cil/loopindex,
; cil/function, cil/funcall, cil/operation, etc

; opt steps include removing duplicate assignments, etc.

; Or the other way around, from lowest level up:
; - emitting instructions/statements
; - gluing those together to form expressions
; - creating control structures out of statements and expressions
; - creating functions and programs out of all of the above
; These do not form a strict chain of command; this way, each layer can
; generate code in a straightforward manner but also (alternatively) in a way
; that reaches down the layers to generate something faster.

import meta/cil/data
import meta/cil/eval

; Turn a list into a function
; import meta/cil/function

; import meta/cil/cf

; Lua (and JIT-ish)
; Writing code strings
import meta/cil/lua/emit
; eDSL for lua-looking snippets
import meta/cil/lua/expr
; Wrapper for chunks that can be loaded as builtins
import meta/cil/lua/interp

; Multi-mode definitions know whether they are supposed to eval or emit code
import meta/cil/mode

; Loops (necessary to be lua-specific?)
import meta/cil/lua/cf

import meta/cil/lua/function
import meta/cil/lua/declare
import meta/cil/lua/redef

export-all

; vi: ft=scheme

