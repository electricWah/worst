
define %output [string->u8vector %output-port 1 dig port-write drop]

0 swapvar genuid%
define gensym% [
    with-swapvar genuid% [clone 1 add]
    datum-describe->string swap drop
    ^' symbol->string
    swap string-append string->symbol
]

define is? [^' equal? swap drop]
define is?! [^: is? swap drop]

define tag: [
    [] swap list-push-head
    ^' list-push-head
]

define list-head [list-pop-head clone 2 negate dig list-push-head swap]
define list-tail [list-pop-tail clone 2 negate dig list-push-tail swap]

define take-tag [list-pop-head]

define expression-type [list-tail]
define ensure-expression [expression? if [] [make-const]]

define expression? [
    list? if [
        take-tag cond [
            [is?! const] [true]
            [true] [false]
        ]
    ] [false]
]

; ty make-var -> ['var id ty]
define typed-var [
    [var] gensym% var list-push-tail
    swap list-push-tail
]

define var? [list-head is?! var]
define var-type [list-tail]
define write-var [list-pop-head drop list-pop-head symbol->string %output drop]

define make-const [
    cond [
        [string?] [ 'string ]
        [int?] [ 'int ]
        [true] [ "Can't use this as a constant" abort]
    ]
    local ty
    [const] swap list-push-tail
    ty list-push-tail
]

define const? [list-head is?! const]
define write-const [
    list-pop-head drop
    list-pop-tail drop
    list-pop-head swap drop
    datum-describe->string write-string drop
]

define make-binop [
]

define add [make-binop int int +]

; expr add-assignment -> [assign var expr]; var
define add-assignment [
    local expr
    expr expression-type typed-var local var
    write-indentation
    "local " write-string
    var var->write
    " = " write-string
    expr write-expression
    write-newline
    var
]

; ... arg extern name [argty ...] retty
define extern [
    ^' symbol->string local name
    ^' local argtypes
    ^' local returns

    [] swapvar arguments
    define add-arg [ with-swapvar arguments [ swap list-push-tail ] ]

    ; argtypes
    define string [
        ensure-expression
        expression-type is? string if [drop add-arg] [ "Wrong argument type" abort]
    ]
    argtypes eval

    [invoke]
    name list-push-tail
    swapvar-take arguments list-push-tail
    returns list-push-tail

    returns is?! void if [ expression->statement ] [ add-assignment ]

    ; moosh args with the stack, do a make-assignment
]


define write-string [%output]

define write-expression [
    cond [
        [var?] [ write-var ]
        [const?] [ write-const ]
    ]
]

define expression->statement [
    write-indentation
    write-expression
    write-newline
]

define local% [
    
]

0 swapvar indentation%
define write-indentation [] ; TODO

define print [extern print [string] void expression->statement]

; define if [
;     ^' ^'
;     ...
; ]

;;; vi: ft=scheme

