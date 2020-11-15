
; Import and export. Loaded automatically by worsti

import syntax/variable
import syntax/attribute

lexical (syntax-read)
lexical-alias quote %%quote
define load-eval-file [
    define run [
        define %%load-eval-file []
        read-eval-loop
        ; leaves <eof> on the stack
        ; also need something here to inhibit tail-call
        drop
    ]
    define quote-read-syntax? [
        %%quote %%load-eval-file
        %%quote definition-exists uplevel
        swap drop
    ]
    open-input-file false? if [ drop [] swap list-push abort ] []
    [] swap list-push
    %%quote source-input-port definition-add
    run
]

dict-empty const %import-files

; Less basic import
lexical (variable updo %import-files)
define import-file [
    const %import-path

    %import-files
    %import-path dict-exists if [
        drop drop
    ] [
        drop drop

        dict-empty const %exports

        ; TODO allow export before definition
        define export-as [
            const newname const defname
            defname definition-resolve const defn drop
            %exports newname defn dict-set drop
        ]

        ; TODO make this an attribute
        define export-name [ upquote clone export-as ]

        define export-all [
            updo current-context-definitions
            map-keys swap drop
            while [list-empty? not] [
                list-pop clone
                export-as
            ]
            drop
        ]

        %import-path
        resolve-import-path
        false? if [ ["module not found"] %import-path list-push abort ] []
        read-file eval
        ; %import-path load-eval-file ; TODO make this work again

        []
        %exports dict-keys list-iterate [
            const k
            k dict-get const v
            drop
            [
                quote quote v quote quote k
                [quote definition-add quote uplevel uplevel] list-iterate []
            ] list-eval
            swap bury list-append
            swap
        ]
        drop
        %import-files %import-path dig dict-set
        drop
    ]

    %import-files %import-path dict-get
    bury drop drop
    eval
]

lexical ()
define import [ upquote quote import-file uplevel ]

lexical (%import-files)
define import-forget [
    upquote const path
    %import-files path dict-remove drop
]

lexical (%import-files)
define import-forget-all [
    dict-empty %import-files set
]

export-name load-eval-file
export-name import-file
export-name import
export-name import-forget
export-name import-forget-all

; vi: ft=scheme

