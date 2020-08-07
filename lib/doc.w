
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

lexical (%docs)
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

