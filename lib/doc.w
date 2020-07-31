
; doc
; - define docs
; - help command
; - doc to html
; - ensure everything has docs
; - iterate through docs and write to html

; doc-for def-name [
;   title "what this function does"
;   usage "usage statement"
;   description "why you would use this function"
;   example "an example (may be specified more than once)"
;   warn "gotchas"
;   section section-name-for-categorising
;   tags [tags for searching or something]
;   ... any more...
; ]

dict-empty const %docs
dict-empty const %tags

lexical (%docs %tags)
define documentation-set [
    const body
    const name
    %docs name body dict-set drop
    name doc-eval [
        define tags [
            upquote list-iterate [
                const tag
                %tags tag dict-get false? if [drop []] []
                name list-push
                dict-set
                drop
            ]
        ]
    ]
]

define doc-for [ upquote upquote documentation-set ]

lexical (%docs)
define doc-eval [
    const name
    upquote const defs
    %docs name dict-get bury drop drop false? if [] [
        const docs
        define title [upquote drop]
        define description [upquote drop]
        define usage [upquote drop]
        define example [upquote drop]
        define section [upquote drop]
        define tags [upquote drop]
        define see-also [upquote drop]
        define internal []
        define undocumented []
        defs docs list-append eval
    ]
]

define-attribute documentation [
    args (doc-body)
    before [
        const name const def-body
        name doc-body documentation-set
        def-body name
    ]
]


lexical (%docs)
define has-documentation? [ %docs swap dict-exists dig drop ]

lexical (%docs)
define documented-names [ %docs dict-keys swap drop ]

lexical (%tags)
define doc-tags [ %tags ]

lexical (%tags)
define doc-tag? [ %tags swap dict-exists dig drop ]

export-name doc-for
export-name doc-eval
export-name documentation
export-name documentation-set
export-name has-documentation?
export-name documented-names

export-name doc-tags
export-name doc-tag?

; vi: ft=scheme

