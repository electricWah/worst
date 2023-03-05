
#f make-place const current-docs

define doc [
    current-docs upquote place-set drop
]
export doc

; define (dispatch (in-definition-attributes)) doc [
;     swap upquote value-doc-set swap
; ]

; define default-attributes [
;     current-docs place-get
;     false? if [] [ bury swap dig value-doc-set swap ]
;     default-attributes
; ]

; export default-attributes

