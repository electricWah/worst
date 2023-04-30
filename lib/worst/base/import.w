
feature-enabled? os if [
    "WORST_LIBPATH" environment-variable
    false? if [ drop () ] [ ":" string-split ]
] [ () ]
const WORST_LIBPATH
export WORST_LIBPATH

current-defenv make-place const global-default-module-definitions
define default-module-definitions [
    quote current-default-module-definitions updo dynamic-resolve-local
    false? if [
        drop global-default-module-definitions place-get
    ] [ eval ]
]
export default-module-definitions

define module-search-load->list [
    const modname
    #f
    ; maybe check feature-enabled? os
    WORST_LIBPATH list-iter [
        const path
        false? if [
            drop
            path string->fs-path
            modname ".w" string-append string->fs-path
            fs-path-concat
            file-open-options file-open-options-set-read
            ; try opening
            file-open error? if [ drop #f ] [ ]
        ] []
    ]
    false? if [
        ; still not found, try embedded
        drop
        modname ".w" string-append string->fs-path
        embedded-file-open
        error? if [ ] [ embedded-file-port->string ]
    ] [
        file-port->string
    ]
    false? if [ drop modname "module not found" error ] [
    ]
    read-string->list
]

define eval-module-list->defenv [
    const imported-name
    const modbody

    defenv-empty make-place const module-exports
    interpreter-empty
    default-module-definitions
    quote module-exports module-exports defenv-insert-local
    quote current-module-imported-name imported-name value->constant defenv-insert-local
    interpreter-defenv-set

    modbody interpreter-eval-list-next
    interpreter-run
    const ret
    interpreter-complete? if [
        ; TODO caching
        drop
        module-exports place-get
    ] [
        "Error in " print imported-name value->string print ": " print
        ret value->string println
        (module error) error
    ]
]

make-hashmap make-place const module-cache

define module-import [
    defenv-empty make-place const all-imports
    <list> is-type if [] [ () swap list-push ]
    list-iter [
        <symbol> is-type if [
            const imported-name
            module-cache place-get imported-name hashmap-get false? if [
                drop
                imported-name symbol->string module-search-load->list
                imported-name eval-module-list->defenv const env
                module-cache place-get imported-name env hashmap-insert
                module-cache swap place-set drop
                env
            ] [ ]

            all-imports place-get
            swap defenv-merge-locals
            all-imports swap place-set drop
        ] [ "import non-symbol" TODO ]
    ]
    all-imports place-get
    updo current-defenv-merge-locals
]
export module-import

define import [ upquote updo module-import ]
export import

; keep old export to export new export
quote export definition-resolve quote old-export definition-add

define export [
    upquote
    <list> is-type if [] [ () swap list-push ]
    const exports
    quote module-exports dynamic-resolve-local
    false? if [ "export: not in a module" println error ] [
        const module-exports
        module-exports place-get
        exports list-iter [
            const x
            x dynamic-resolve-local false? if [
                "export: not defined: " x value->string string-append
                clone println error
            ] [ ]
            const def
            x def defenv-insert-local
        ]
        module-exports swap place-set drop
    ]
]
; export new import using old export (further exports in this file will break)
old-export export

global-default-module-definitions current-defenv place-set drop

