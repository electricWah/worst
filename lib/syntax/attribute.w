
; attribute ...
; define func [...]

; Attributes are definitions you can use to augment the next define form.

define define-attribute [
    import syntax/variable

    upquote const attr-name
    upquote const attr-body

    ; name def-body "attribute" interpreter-dump-stack drop drop drop

    [[]] variable %args

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

    [
        [ upquote upquote swap updo definition-add ]
        quote define definition-add
        attr-body eval
    ] eval

    %args get

    [list-append] %before get list-push list-append

    [
        quote %before-define updo definition-get swap drop [] or bury drop drop
        list-append
        quote %before-define updo definition-add
    ]
    list-append

    [
        quote %current-attributes updo definition-get
        swap drop [] or bury drop drop
    ]
    list-append
    [ quote quote attr-name ]
    list-eval
    list-append
    [
        list-push
        quote %current-attributes updo definition-add
    ]
    list-append

    attr-name updo definition-add+attributes
]
export-name define-attribute

define definition-add+attributes [
    quote %before-define definition-resolve
    false? if [drop []] []
    swap updo definition-remove
    eval
    quote %current-attributes updo definition-remove
    updo definition-add
]
export-name definition-add+attributes

define default-attributes [
    upquote
    [ quote %before-define definition-copy-up ]
    list-append eval
    quote %before-define definition-copy-up
]
export-name default-attributes

define define [
    upquote upquote swap
    updo definition-add+attributes
]
export-name define

define-attribute lexical [
    args (names)
    before [
        swap
        names symbol? if [call] []
        ; interpreter-dump-stack
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

; lexical-alias oldname newname
define-attribute lexical-alias [
    args (old new)
    before [
        swap
        quote definition-add list-push
        new list-push
        quote quote list-push
        old definition-resolve
        false? if ["lexical-alias: not defined: "
                   dig ->string string-append
                   abort] []
        swap drop list-push
        swap
    ]
]
export-name lexical-alias

; don't override the define if already defined
define-attribute weakly [
    ; You are not expected to like this code
    before [
        definition-resolve not if [ ] [ drop quote %%weakly-defined ]
    ]
]
export-name weakly

define-attribute modifying-body [
    args (%modifier)
    before [
        const %name
        %modifier eval
        %name
    ]
]
export-name modifying-body

; vi: ft=scheme

