
; toplevel in here only
define doc-for [
    upquote const name
    upquote const doc
    
    name definition-resolve
    doc
    value-doc-set
    name quote definition-add quote uplevel uplevel
]

doc-for swap [
    title "Swap the top two values on the stack."
    example [1 2 swap]
]

doc-for drop [
    title "Remove the top value on the stack."
    example ["ignore me" drop]
]

