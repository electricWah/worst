
feature-enabled? os if [
    "WORST_LIBPATH" environment-variable
    false? if [ drop () ] [ ":" string-split ]
] [ () ]
make-place const global-module-search-path

define module-search-path-prepend [
    const path
    global-module-search-path place-get path list-push
    global-module-search-path swap place-set
    drop
]
export module-search-path-prepend

define module-search-path [ global-module-search-path place-get ]
export module-search-path

current-defenv make-place const global-default-module-definitions
define default-module-definitions [
    quote current-default-module-definitions updo definition-resolve
    false? if [
        drop global-default-module-definitions place-get
    ] [ eval ]
]
export default-module-definitions

define module-search-load->string [
    const modname
    #f
    ; maybe check feature-enabled? os
    module-search-path list-iter [
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
]

; relative to current file using current-script-path set in prelude
define module-relative-load->string [
    const path
    quote current-script-path updo definition-resolve eval
    string->fs-path fs-path-parent const dir
    dir path string->fs-path fs-path-concat
    file-open-options file-open-options-set-read
    file-open
    error? if [ path "module not found" error ] [ file-port->string ]
]

define make-current-module [
    const imported-name
    make-hashmap
    quote exports defenv-empty make-place hashmap-insert
    quote imported-name imported-name hashmap-insert
]

define module-exports [ quote exports hashmap-get ]
define module-imported-name [ quote imported-name hashmap-get ]

define eval-module-list->defenv [
    const imported-name
    const modbody

    imported-name make-current-module const current-module

    interpreter-empty
    default-module-definitions
    quote current-module current-module value->constant defenv-insert-local
    interpreter-defenv-set

    modbody interpreter-eval-list-next
    interpreter-run
    const ret
    interpreter-complete? if [
        ; TODO caching
        drop
        current-module module-exports place-get
    ] [
        "Error in " print imported-name value->string print ": " print
        ret value->string println
        (module error) error
    ]
]

make-hashmap make-place const module-cache

define module-import [
    updo current-module const modctx
    defenv-empty make-place const all-imports
    <list> is-type if [] [ () swap list-push ]
    list-iter [
        const imported-name
        module-cache place-get imported-name hashmap-get false? if [
            drop
            imported-name
            <symbol> is-type if [
                symbol->string module-search-load->string
            ] [
                <string> is-type if [
                    modctx false? if [
                        drop module-relative-load->string
                    ] [
                        ; technically possible, but only from a relative import
                        ; so keep track of that (using imported-name?)
                        "cannot import string from within a module" error
                    ]
                ] [
                    "import: unknown type" error
                ]
            ]
            read-string->list
            imported-name eval-module-list->defenv const env
            module-cache place-get imported-name env hashmap-insert
            module-cache swap place-set drop
            env
        ] [ ]
        all-imports place-get
        swap defenv-merge-locals
        all-imports swap place-set drop
    ]
    all-imports place-get
    updo current-defenv-merge-locals
]
export module-import

define import [ upquote updo module-import ]
export import

; keep old export to export new export
quote export definition-resolve quote old-export definition-add

quote current-defenv definition-resolve const get-defenv
define export [
    get-defenv uplevel const export-env
    upquote
    <list> is-type if [] [ () swap list-push ]
    const exports
    updo current-module
    false? if [ "export: not in a module" println error ] [
        module-exports const cme
        cme place-get
        exports list-iter [
            const x
            export-env x defenv-lookup false? if [
                "export: not defined: " x value->string string-append
                clone println error
            ] [ ]
            const def
            x def defenv-insert-local
        ]
        cme swap place-set drop
    ]
]
; export new import using old export (further exports in this file will break)
old-export export

global-default-module-definitions current-defenv place-set drop

