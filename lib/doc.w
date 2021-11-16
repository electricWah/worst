
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

import data/dict

dict-empty const %docs
dict-empty const %tags

define documentation-set [
    import data/map
    const body
    const name
    %docs name body dict-set drop
    body pairs->map quote tags map-get bury drop drop false? if [drop] [
        list-iterate [
            const tag
            %tags tag dict-get false? if [drop []] []
            name list-push
            dict-set
            drop
        ]
    ]
]

define doc-for [ upquote upquote documentation-set ]

; doc-eval [
;   key [program] ...
; ]
define doc-eval [
    import data/map
    const name
    upquote pairs->map const defs
    %docs name dict-get
    bury drop drop
    false? if [] [
        ; const docs
        ; iterate pairwise over doc defs
        #f ; key
        swap list-iterate [
            swap false? if [
                ; use as key
                drop
            ] [
                const key
                defs key map-get bury drop drop
                false? if [ drop drop ] [ eval ]
                #f ; put blank key back
            ]
        ]
        drop
        #t
    ]
]

; TODO fix docs
define documentation [ upquote drop ]

define has-documentation? [ %docs swap dict-exists dig drop ]

define documented-names [ %docs dict-keys swap drop ]

define doc-tags [ %tags ]

define doc-tag? [ %tags swap dict-exists dig drop ]

export {
    doc-for
    doc-eval
    documentation
    documentation-set
    has-documentation?
    documented-names

    doc-tags
    doc-tag?
}

; vi: ft=scheme

