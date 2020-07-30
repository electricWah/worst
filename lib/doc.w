
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

import syntax/attribute
import syntax/variable
import syntax/assign
import dict

dict %docs
dict %tag-docs

lexical (%docs %tag-docs)
define documentation-set [
    const body
    const name
    name body %docs set
    name doc-eval [
        define tags [
            upquote list-iterate [
                const tag
                tag %tag-docs has if [
                    %tag-docs get
                    name list-push
                    %tag-docs set
                ] [
                    [] name list-push
                    %tag-docs set
                ]
            ]
        ]
    ]
]

define doc-for [ upquote upquote documentation-set ]

lexical (%docs)
define doc-eval [
    const name
    upquote const defs
    name %docs has if [
        %docs get!
        defs swap list-append
        define title [upquote drop]
        define description [upquote drop]
        define usage [upquote drop]
        define example [upquote drop]
        define section [upquote drop]
        define tags [upquote drop]
        define see-also [upquote drop]
        define internal []
        define undocumented []
        eval
    ] [ drop drop ]
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
define has-documentation? [ %docs has ]

lexical (%docs)
define documented-names [ %docs keys ]

lexical (%tag-docs)
define doc-tags [ %tag-docs ->map ]

lexical (%tag-docs)
define doc-tag? [ %tag-docs has ]

export-name doc-for
export-name doc-eval
export-name documentation
export-name documentation-set
export-name has-documentation?
export-name documented-names

export-name doc-tags
export-name doc-tag?

; vi: ft=scheme

