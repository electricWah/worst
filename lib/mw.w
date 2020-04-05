
;;; Mw: Metaworst
; A slightly more accessible language on top of Worst.

;; Core words
; Just four: (module function macro <-)
; - importing and exporting:
; module {
;   import (name ...) ; WORST_LIBDIR/name.mw
;   export (name ...)
; }
; - defining functions and macros:
; function name(arg ...) { body ... }
; name(arg ...)
; macro name(arg ...) { body ... }
; name arg ...
; - name binding:
; (name ...) <- expr

;; To do
; [#] Interpreter in Worst
; [ ] Create Mw builtins
; [ ] Lexical-ify everything (compile Worst to lexical Worst)
; [ ] Compile Metaworst to Worst

; interpreting the base words
import mw/base
export-name module
export-name function
export-name macro
export-name <-


; vi: ft=scheme

