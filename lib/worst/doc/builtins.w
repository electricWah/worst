
import worst/doc/for

doc-for swap [
    title "Swap the top two values on the stack."
    example [1 2 swap]
]

doc-for drop [
    title "Remove the top value on the stack."
    example ["ignore me" drop]
]

export (swap drop)

