
import worst/doc/attribute

; doc-for name [ doc ... ]
; doc-for name "docstring"
define doc-for [
    upquote const name
    upquote const doc
    
    name definition-resolve
    doc value-doc-set
    swap
    updo definition-add
]
export (doc-for)

