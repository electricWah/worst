
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

import syntax/attributes
import syntax/variable
import syntax/assign
import dict
import list

dict %docs
dict %tag-docs

define documentation-set [
    @[lexical %docs
      lexical %tag-docs
      lexical list-iterate
    ]
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

define doc-for [
    @[lexical %docs
      lexical documentation-set
    ]
    upquote upquote documentation-set
]

define doc-eval [
    @[lexical %docs]
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

define documentation [
    @[lexical documentation-set]
    define-attribute? if [] ["documentation must be used as an attribute" abort]
    const body const name
    name upquote documentation-set
    name body
]


define has-documentation? [
    @[lexical %docs]
    %docs has
]

define documented-names [
    @[lexical %docs]
    %docs keys
]

define doc-tags [
    @[lexical %tag-docs]
    %tag-docs ->hash-table
]

define doc-tag? [
    @[lexical %tag-docs]
    %tag-docs has
]

export doc-for
export doc-eval
export documentation
export documentation-set
export has-documentation?
export documented-names

export doc-tags
export doc-tag?

; vi: ft=scheme

