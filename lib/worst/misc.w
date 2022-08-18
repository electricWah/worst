
define ->string [ value->string ]

define print-value [ ->string print ]
define print [ current-output-port swap port-write-string port-flush drop ]

define read-line [ current-input-port buffered-port-read-line swap drop ]

; path read-file -> list
define read-file [
    open-input-file false? if [ drop [] swap list-push "read-file" error ] []
    [] while [ swap port-read-value eof-object? not ] [ dig swap list-push ]
    drop drop
    list-reverse
]

; evaluate = quote; call if symbol
define evaluate [ upquote symbol? quote call quote when uplevel ]

; a equals? b => a bool
define equals? [ upquote equal? swap drop ]
define lt? [ clone upquote lt ]
define le? [ clone upquote le ]
define gt? [ clone upquote gt ]
define ge? [ clone upquote ge ]
; a == b => bool
; define == [ upquote equal? bury drop drop ]

define not [ false? swap drop ]
define abs [ 0 ascending? swap drop if [negate] [] ]
define max [ ascending? if [swap] [] drop ]
define min [ ascending? if [] [swap] drop ]

; updo thing => quote thing uplevel
define updo [ upquote quote uplevel uplevel ]

; n iteri [ n -> body ... ]
; do body n times with 0 .. n on the stack
define iteri [
    const %%iteri-maxn
    upquote quote %%iteri-body definition-add

    0
    while [%%iteri-maxn ascending? bury drop swap] [
        const %%iteri-n
        %%iteri-n %%iteri-body
        %%iteri-n 1 add
    ]
    drop
]

; n do-times [body...]
define do-times [
    upquote quote %%do-times-body definition-add

    while [0 swap ascending? bury swap drop swap] [
        const %%do-n
        %%do-times-body
        %%do-n
        1 negate add
    ]
    drop
]

define definition-exists [
    updo definition-get not not
]

; name new-name definition-rename
define definition-rename [
    const new-name
    const name
    name updo definition-get
    false? if [drop [] swap list-push "undefined" error] []
    swap drop new-name
    updo definition-add
    name updo definition-remove
]

; name definition-copy-up
; copies the definition into the parent scope
define definition-copy-up [
    const name
    name updo definition-get
    false? if [drop [] swap list-push "undefined" error] []
    swap
    quote definition-add
    quote uplevel
    uplevel
]

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

define stack-swap [
    const new
    stack-get const old
    new stack-set
    old
]

export #t

; vi: ft=scheme

