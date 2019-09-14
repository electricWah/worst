
; Import and export. Loaded automatically by worsti

import syntax/variable
import syntax/attributes
import syntax/assign

define load-eval-file [
    @[lexical syntax-read]
    define run [
        define %%load-eval-file []
        read-eval-loop
        ; leaves <eof> on the stack
        ; also need something here to inhibit tail-call
        drop
    ]
    define quote-read-syntax? [
        builtin-quote %%load-eval-file
        builtin-quote definition-exists uplevel
        swap drop
    ]
    open-input-file [] swap list-push
    builtin-quote source-input-port definition-add
    run
]

hash-table-empty variable %import-files

; Less basic import
define import-file [
    @[lexical variable
      lexical updo
      lexical %import-files
    ]

    resolve-import-path const %import-path

    %import-files get
    %import-path hash-table-exists if [
        drop drop
    ] [
        drop drop

        [] variable %on-import-file-finished

        ; TODO allow export before definition
        define %export [
            const newname const defname
            %on-import-file-finished get
            [ quote definition-add quote uplevel uplevel ] list-append
            newname list-push
            quote quote list-push
            defname definition-resolve swap drop list-push
            quote quote list-push
            %on-import-file-finished set
        ]

        define export [ upquote clone %export ]

        %import-path load-eval-file

        %import-files get
        %import-path
        %on-import-file-finished get
        hash-table-set
        %import-files set

    ]

    %import-files get %import-path hash-table-get swap drop swap drop eval
]

define import [ upquote quote import-file uplevel ]

define import-forget [
    @[lexical %import-files]
    %import-files get upquote resolve-import-path hash-table-remove
    %import-files set
]

define import-forget-all [
    @[lexical %import-files]
    hash-table-empty %import-files set
]

export load-eval-file
export import-file
export import
export import-forget
export import-forget-all

; vi: ft=scheme

