
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

; import mw/mdefn
; export-name mdefn-eval

; import mw in the repl will break everything
; as it will redefine several words such as if and import.
; Detect a repl and import mw/repl instead.
quote %%repl definition-resolve false? if [
    drop drop
    ; ???
] [
    drop drop
    ; module {
    ;     import {
    ;         builtins
    ;         ; lexical-module
    ;         ui/repl
    ;     }
    ; }

    ; function cool(a b) {
    ;     a + b
    ;     a - b
    ; }

    ; (a b) <- cool(5 6)

    ; a b

    ; cool()

;     read-eval-loop()
]

;; Lexical-ify Worst
; Might as well just compile it to itself

; vi: ft=scheme

