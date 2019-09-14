
; Documentation for builtins.

; import doc/check
; check-docs

import doc

doc-for abort [
    title "Raise an error"
    description "Stops evaluation, citing the top value as a reason."
    usage "reason abort"
    example "\"too cool\" abort"
    example "\"abort is very basic\" abort"
    section errors
    tags [errors]
]

doc-for add [
    title "Add two numbers together"
    description "Replaces the top two values with their sum."
    usage "a b add"
    example "4 5 add equals? 9"
    example "-30 2 add equals? 28"
    section numeric-ops
    tags [numeric binops]
]

doc-for and [
    title "Boolean and of two values"
    description "If either of the top two values are false, gives false.
    Otherwise gives the lower value."
    usage "a b and"
    example "6 \"hello\" and equals? 6"
    example "9 #f and equals? #f"
    section logic-ops
    tags [booleans binops]
]

doc-for ansi [
    title "Terminal formatting DSL"
    description "Check out the examples"
    usage "ansi [ body ... ]"
    example "ansi [ bright red fg \"Red!\\n\" print reset ]"
    section ui
    tags [ui output]
]

doc-for %before-define [
    title "Handler for adding definitions"
    description "define calls this function before adding a definition.
    Used by syntax/attributes. This is a hack."
    internal
    section syntax
    tags [internal definition]
]

doc-for bury [
    title "Stack rotation: a b c -> b c a"
    description "Moves the top item on the stack to position 3"
    example "1 2 3 bury"
    section stack
    tags [stackops]
]





; doc-for abs [
;     title "Absolute value of a number"
;     description "Replaces the top value with its absolute value."
;     usage "n abs"
;     example "1 abs equals? 1"
;     example "-50 abs equals? 50"
;     section numeric-ops
;     tags [numeric arithmetic unary-op]
; ]

; doc-for negate [
;     title "Negate a number"
;     description "Replaces the top value with its negation."
;     usage "n negate"
;     example "1 negate equals? -1"
;     example "-50 negate equals? 50"
;     section numeric-ops
;     tags [numeric arithmetic unary-op]
; ]

; doc-for bool? [
;     title "Type predicate for boolean values"
;     description "Gives whether the top value is a boolean or not."
;     usage "value bool?"
;     example "#t bool? equals? #t"
;     example "\"not a bool\" bool? equals? #f"
;     section predicates
;     tags [type boolean]
; ]

; doc-for const [
;     title "Define a constant"
;     description "Gives the top value a name for future use."
;     usage "value const name"
;     example "6 const six"
;     example "six six add equals? 12"
;     section syntax
;     tags [definition stack]
; ]

; these aren't builtins

define undocumented [
    upquote [undocumented] documentation-set
]

undocumented builtin-quote
undocumented %abort-to-repl
undocumented %%repl
undocumented %run

doc-for WORST_LIBDIR [
    title "Location of import library"
    description "Copied from the environment variable of the same name;
    import looks here to load files."
    internal
    see-also import
    section modules
    tags [internal definition]
]

