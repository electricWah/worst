
#f make-place const current-docs

make-unique const value-doc-meta-key
define value-doc-set [ value-doc-meta-key swap value-insert-meta-entry ]
export value-doc-set
define value-doc [ value-doc-meta-key value-meta-entry ]
export value-doc

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

