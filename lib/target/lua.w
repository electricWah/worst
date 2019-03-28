
define/q %output [string->u8vector %output-port 1 dig port-write drop]
export %output

0 swapvar genuid%
export genuid%
define/export gensym% [
    with-swapvar genuid% [clone 1 add]
    datum-describe->string swap drop
    ^' symbol->string
    swap string-append string->symbol
]

define/q is? [^' equal? swap drop]
define/q is?! [^: is? swap drop]
export is?
export is?!

define tag: [
    [] swap list-push-head
    ^' list-push-head
]

define/q list-head [list-pop-head clone 2 negate dig list-push-head swap]
define/q list-tail [list-pop-tail clone 2 negate dig list-push-tail swap]
export list-head
export list-tail

define/export expression-tag [list-head]

define/q expression-type [list-tail]
define/q ensure-expression [expression? if [] [make-const]]

define/q expression? [
    list? if [
        expression-tag cond [
            [is? const] [true]
            [is? var] [true]
            [is? invoke] [true]
            [true] [false]
        ]
        swap drop
    ] [false]
]
export expression?

; ty typed-var -> ['var id ty]
define/export typed-var [
    [var] gensym% var list-push-tail
    swap list-push-tail
]

define/export var? [list-head is?! var]
define/export var-type [list-tail]
define/export write-var [list-pop-head drop list-pop-head symbol->string %output drop]

define/q type-is!% [
    swap expression-type 2 dig equal? if [drop drop] ["Wrong type" abort]
]
define type-is! [ ^' type-is!% ]
export type-is!%
export type-is!

define/q ensure-expression/type! [
    ensure-expression ^' type-is!%
]
export ensure-expression
export ensure-expression/type!
export expression-type

define/export make-const [
    cond [
        [string?] [ 'string ]
        [int?] [ 'int ]
        [float?] [ 'float ]
        [bool?] [ 'bool ]
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
export const?
export write-const

define/export invocation? [list-head is?! invoke]
define/export write-invocation [
    list-pop-head drop
    list-pop-head write-string
    list-pop-head
    "(" write-string
    while [list-empty? not] [
        list-pop-head write-expression
        list-empty? if [] [", " write-string]
    ]
    ")" write-string
    drop
    drop
]

define make-binop [
]

; define add [make-binop int int +]
; define sub [make-binop int int -]

; expr add-assignment -> [assign var expr]; var
define/export add-assignment [
    expression-type typed-var local var
    local expr
    write-indentation
    "local " write-string
    var write-var
    " = " write-string
    expr write-expression
    write-newline
    var
]

; [list] list-map [el -> el] -> [list]
define/export list-map [
    ^' 'body% add-definition
    [] swapvar out%
    while [list-empty? not] [
        list-pop-head
        body%
        with-swapvar out% [swap list-push-tail]
    ]
    drop
    swapvar-take out%
]

; expr add-assignment -> [assign var expr]; var
define/export add-multival-assignment [
    expression-type local types
    local expr

    types list-map [typed-var] local vars

    write-indentation
    "local" write-string
    vars while [list-empty? not] [
        " " write-string
        list-pop-head write-var
        list-empty? if [] ["," write-string]
    ] drop
    " = " write-string
    expr write-expression
    write-newline

    vars while [list-empty? not] [
        list-pop-head swap
    ] drop
]


; ... arg invoke name [argty ...] retty
define/export invoke [
    ^' local name
    ^' local args
    ^' local returns
    args returns name invoke%
]

; [argty...] [retty...] name invoke%
define/export invoke% [
    local name
    local returns
    local argtypes
    without-luamode% [
        [] swapvar arguments
        define add-arg [ with-swapvar arguments [ swap list-push-tail ] ]

        ; argtypes
        [
            define string [
                ensure-expression/type! string add-arg
            ]
            define bool [
                ensure-expression/type! bool add-arg
            ]
            define int [
                ensure-expression/type! int add-arg
            ]
            define float [
                ensure-expression/type! float add-arg
            ]

            argtypes eval
        ] eval

        [invoke]
        name list-push-tail
        swapvar-take arguments list-push-tail
        returns list-push-tail

        returns is?! [] if [ expression->statement ] [ add-multival-assignment ]

        ; moosh args with the stack, do a make-assignment
    ]
]

define/q write-string [%output]
define/q write-newline ["\n" write-string]
define/q write-string/n [write-string write-newline]
export write-string
export write-newline
export write-string/n

define/q write-keyword% [ ^' symbol->string write-indentation write-string ]
export write-keyword%

define/q write-expression [
    cond [
        [var?] [write-var]
        [const?] [write-const]
        [invocation?] [write-invocation]
        [true] ["Unknown expression type" abort]
    ]
]

define expression->statement [
    write-indentation
    write-expression
    write-newline
]
export write-expression
export write-newline
export expression->statement

; n do-times [ body ]
define/q do-times [
    ^' local body%
    swapvar n%
    while [
        with-swapvar n% [1 negate add clone]
        0 greater-than
        swap drop swap drop
        not
    ] [
        body% eval
    ]
]
export do-times

0 swapvar indentation%
export indentation%
define/q write-indentation [
    without-luamode% [
        swapvar-get indentation% do-times [ "    " write-string ]
    ]
]
export write-indentation
define indented [
    with-swapvar indentation% [ 1 add ]
    ^' eval
    with-swapvar indentation% [ 1 negate add ]
]
export indented

; lua and worst both spell 'if' the same
; so have a magic 'if' that calls the correct one
; - most library functions should turn this off
true swapvar luamode%
define/q %luamode? [swapvar-get luamode%]
define/q without-luamode% [
    ; %luamode? datum-describe->string printe drop
    false luamode% local %tmp-luamode
    ^' eval
    ; %luamode? datum-describe->string printe drop
    %tmp-luamode luamode% drop
]
define/q with-luamode% [
    true luamode% local %tmp-luamode
    ^' eval
    %tmp-luamode luamode% drop
]
export luamode%
export %luamode?
export with-luamode%
export without-luamode%

define/q luaif% [
    without-luamode% [
        local %iftrue
        write-indentation "if " write-string
        ensure-expression/type! bool
        write-expression
        " then" write-string/n
        indented [ with-luamode% [ %iftrue eval ] ]
        write-indentation "end" write-string/n
    ]
]
export luaif%

define/export luaifelse% [
    without-luamode% [
        local %iffalse
        local %iftrue
        ; can do a literal if% if there is a bool
        stack-empty? if [ false ] [ bool? ]
        if [ %iftrue %iffalse if% ] [
            write-indentation "if " write-string
            ensure-expression/type! bool
            write-expression " then" write-string/n
            indented [ with-luamode% [ %iftrue eval ] ]
            write-indentation "else" write-string/n
            indented [ with-luamode% [ %iffalse eval ] ]
            write-indentation "end" write-string/n
        ]
    ]
]

define/q define/override/when [
    ^' local cond%
    ^' local name%
    ^' local def%
    without-luamode% [
        name% resolve-definition false? if [
            drop name% "No define to override" abort
        ] []
        local realdef%
        
        cond%
        [] def% list-push-tail list-push-tail
        [] realdef% list-push-tail list-push-tail
        [if% quote eval-definition uplevel] list-append
        name%
    ]
    'add-definition
    uplevel
]
export define/override/when

define/override/when [%luamode?] if [ ^' ^' luaifelse% ]
export if

define/export luawhile% [
    without-luamode% [
        local %cond
        local %body
        write-indentation "while true do" write-string/n
        indented [
            with-luamode% [ %cond eval-definition ]
            invoke "not" [bool] [bool]
            [ write-indentation "break" write-string/n ] luaif%
            with-luamode% [ %body eval-definition ]
        ]
        write-indentation "end" write-string/n
    ]
]

define/override/when [%luamode?] while [ ^' ^' swap luawhile% ]
export while

define/export require% [
    symbol->string local lib
    define/q + [ string-append ]
    lib-dir
        "/lua/lib/" +
        lib +
        ".w" +
    ^: eval-file
]

define/export require [ ^' require% ]

; TODO this is more like a regular "define" but extern things are similar
define/export extern% [
    without-luamode% [
        [
            define syntax-read [ 'quote 'uplevel '%def uplevel-in-named-context ]
        ] swap list-append
        [
            '%def context-set-name
            without-luamode%
        ] swap list-push-tail
    ]
    'define% uplevel
]

define/export extern [
    ^' ^' 'extern% uplevel
]

define/export extern/export [
    ^' ^' 'extern% 'uplevel uplevel
]

define/export luadefine% [
    '%def context-set-name
    without-luamode% [
        local def%
        clone local defname%
        symbol->string local name%
        stack-empty? if [] ["Cannot define: stack not empty" abort]
        [] swapvar args%
        "" swapvar defbody%
        [
            define/override/when [stack-empty?] ensure-expression/type! [
                ^' typed-var clone with-swapvar args% [ swap list-push-head ]
            ]
            define %output [ with-swapvar defbody% [ swap string-append ] ]
            define syntax-read [
                ^' uplevel-in-named-context
            ]
            indented [ with-luamode% [ def% eval ] ]
        ] eval
        "function " write-string name% write-string "(" write-string
        swapvar-get args%
        while [list-empty? not] [
            list-pop-tail write-expression
            list-empty? if [] [", " write-string]
        ]
        drop
        ")" write-string/n
        swapvar-take defbody% write-string
        [] swapvar retty%
        [] swapvar retval%
        while [stack-empty? not] [
            ensure-expression
            expression-type
            with-swapvar retty% [ swap list-push-head ]
            with-swapvar retval% [ swap list-push-head ]
        ]
        indented [
            write-keyword% return
            swapvar-take retval%
            while [list-empty? not] [
                list-length is?! 1 not local %comma
                " " write-string
                list-pop-head write-expression
                %comma if ["," write-string] []
            ]
            drop
            write-newline
        ]
        "end" write-string/n

        defname%
        [invoke]
        name% list-push-tail
        swapvar-take args% list-length
        do-times [
            list-pop-head
            expression-type swap drop
            list-push-tail
        ]
        list-push-tail
        swapvar-take retty% list-push-tail
        ; TODO uplevel define [luainvoke% name% argtypes% ret%]
    ]
    'define% uplevel
]

define/override/when [%luamode?] define [ ^' ^' luadefine% ]
export define

'stdlib ^: require%

;;; vi: ft=scheme

