
; attribute ...
; define func [...]

; Attributes are definitions you can use to augment the next define form.

define define-attribute [
    import list
    import syntax/variable

    upquote const name
    upquote const def-body

    [] variable %args

    define args [
        [[]]
        upquote list-iterate [
            const arg
            [upquote list-push quote quote list-push list-append]
            [] arg list-push quote const list-push
            list-push
            list-append
        ]
        %args set
    ]

    [] variable %before
    define before [ upquote %before set ]

    def-body eval

    %args get

    [list-append] %before get list-push list-append
    [quote %before-define updo definition-get swap drop [] or bury drop drop
        list-append
        quote %before-define updo definition-add
    ]
    list-append

    name updo definition-add+attributes
]
export-name define-attribute

define definition-add+attributes [
    quote %before-define updo definition-get swap drop false? if [drop] [eval]
    quote %before-define updo definition-remove
    updo definition-add
]
export-name definition-add+attributes

define define [
    upquote upquote swap
    updo definition-add+attributes
]
export-name define

define-attribute lexical [
    args (names)
    before [
        import list
        swap
        names
        list-iterate [
            const name
            quote definition-add list-push
            name list-push
            quote quote list-push
            name definition-resolve
            false? if ["lexical: not defined: " dig ->string string-append abort] []
            swap drop list-push
        ]
        swap
    ]
]
export-name lexical

; vi: ft=scheme

