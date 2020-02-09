
define abort [ quote abort current-error-handler ]
export-name abort

; evaluate = quote; call if symbol
define evaluate [ upquote symbol? if [quote call] [[] quote eval] uplevel ]
export-name evaluate

; a equals? b => a bool
define equals? [ upquote equal? swap drop ]
export-name equals?

define max [ greater-than if [swap] [] drop ]
define min [ greater-than if [] [] drop ]
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

define while [
    upquote quote %%cond definition-add
    upquote quote %%while-body definition-add
    [
        %%cond if [%%while-body %%loop] [[]] current-context-set-code
    ] const %%loop
    %%loop current-context-set-code
]
export-name while

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

; vi: ft=scheme

