
; CIL 
; Codegen Instruction Language
; Compiler/Interpreter Library
; Cornish Intermediate Linguini

; Write Worst programs that can turn themselves into other languages.

; Generate Lua code using regular Worst
import cil/luagen
; not necessary if cil/bootstrap does this instead

; TODO this next
; Use cil/luagen to reimplement cil/luagen as Lua
; With some clever attribute usage and cil/redef, it could be quite readable.
import cil/bootstrap

; cil/target/lua (and other languages)
;   which defines specifically how to output things as the target code
; cil/porcelain
;   which overrides existing functions like define, if, iteri, etc
;   and turns them into wrappers around cil/ stuff
; interactions between the two:
;   some targets may want to override define so you can't just put it anywhere
;   it may be that cil/target/... needs to explicitly re-export cil/redef
;   or whatever libraries use cil/target/... should do that by itself
;   or perhaps there should be a final wrapper which takes redef and a specific
;   target and compiles an eval wrapper for speed

export-all

; vi: ft=scheme

