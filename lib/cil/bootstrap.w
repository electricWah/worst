
; cil/bootstrap: cil/luagen recompiled into lua

; eval-program does a lot of the work
; no data types required?
; very last thing that happens is eval code
;  or a little wrapper that evals code and then wraps up

; import cil/bootstrap/data

import cil/bootstrap/init

import cil/bootstrap/builtins

import cil/bootstrap/eval

; vi: ft=scheme

