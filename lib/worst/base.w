
define abort [ quote abort current-error-handler ]
export-name abort

; evaluate = quote; call if symbol
define evaluate [ upquote symbol? quote call quote when uplevel ]
export-name evaluate

; a equals? b => a bool
define equals? [ upquote equal? swap drop ]
export-name equals?

define abs [ 0 ascending? swap drop if [negate] [] ]
define max [ ascending? if [swap] [] drop ]
define min [ ascending? if [] [swap] drop ]
export-name abs
export-name max
export-name min

; updo thing => quote thing uplevel
define updo [ upquote quote uplevel uplevel ]
export-name updo

define tailcall [
    upquote
    definition-resolve
    updo current-context-set-code
]

; n do-times [body...]
define do-times [
    upquote quote %%do-times-body definition-add

    while [0 swap greater-than bury swap drop swap] [
        -1 add const %%do-n
        %%do-times-body
        %%do-n
    ]
    drop
]
export-name do-times

define print [ current-output-port swap port-write-string drop ]
export-name print

define definition-exists [
    updo definition-get not not
]
export-name definition-exists

; name new-name definition-rename
define definition-rename [
    const new-name
    const name
    name updo definition-get
    false? if ["definition-rename: does not exist" abort] []
    swap drop new-name
    updo definition-add
    name updo definition-remove
]
export-name definition-rename

; name definition-copy-up
; copies the definition into the parent scope
define definition-copy-up [
    const name
    name updo definition-get
    false? if ["definition-copy-up: does not exist" abort] []
    swap
    quote definition-add
    quote uplevel
    uplevel
]
export-name definition-copy-up

; define-gensym name :-> name -> symbol
define define-gensym [
    upquote const name
    [
        ; silly but simple
        const counter
        const name
        counter
        place-get
        1 add place-set
        place-get swap drop
        ->string
        name swap string-append
        string->symbol
    ]
    0 make-place list-push
    name symbol->string list-push
    name definition-add
    name definition-copy-up
]
export-name define-gensym

; TODO gensym

define interpreter-stack-swap [
    const new
    interpreter-stack const old
    new interpreter-stack-set
    old
]
export-name interpreter-stack-swap

; vi: ft=scheme

