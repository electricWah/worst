
; Usable import/export. Look at all these features it will have!

; import name
; import {
;     module/a
;     module/b {
;         reimport
;         just (id-a id-b ...)
;         not (bad-id-a)
;         rename (x my-x ...)
;         prefix test-
;         suffix -ok
;         override
;     }
; }

; export name
; export #t ; all
; export (name ...)


; Very basic import/export
define import [
    upquote symbol->string quote module-import uplevel
]
define export-name [
    upquote definition-resolve swap definition-export
]

export-name import
export-name export-name

