
import syntax/dict
; import mw/mdefn

; module {
;   import (name ...)
;   export (name ...)
;   ...
;   ; TODO:
;   import name (only (name ...) prefix (name- ...) ...)
; }
;   A module form provides the only interface between different files.
;   It creates a dictionary to be filled by imported definitions and then
;     redefines them in its parent context.
;   An inner export assigns a dummy value in the importing module's dictionary.
;   Any function or module definitions check the importing module's dictionary
;     for the dummy value and replace it with the real definition.
;   An inner import loads the specified file so it can fill the dictionary,
;     and checks for dummy values (exported values that were never defined).
;   Duplicate definitions are an error.

dict %mw-imports

lexical (%mw-imports export-name import list-iterate if)
define module [
    import syntax/variable
    import syntax/dict

    ; %mw-imports: cache dict (path -> dict (symbol -> def))
    ; %exports: list of symbols this module will be exporting.
    ; %module-imports: dict of defs this module will be importing.
    ; %imports: dict added to by inner import when exporting
    ;  which is added to %mw-imports on first import
    ;  and absorbed into %module-imports

    upquote const %modbody

    [] variable %exports

    ; resolve-import-path .w -> .mw
    define resolve-mw-import-path [
        symbol? if [
            ->string
            WORST_LIBDIR "/mw/" string-append
            swap string-append
            ".mw" string-append
        ] [ ("resolve-import-path: invalid") abort ]
    ]

    define export [
        upquote list? if [
            %exports get
            list-append
            %exports set
        ] [
            ("export name: not implemented") abort
        ]
    ]

    ; body name %maybe-export
    ; only exports if it is being imported (%imports exists)
    define %maybe-export [
        const name const body
        body name
        quote %imports definition-resolve swap drop
        false? swap drop if [] [
            %imports has if [
                %imports get false? if [
                    drop
                    swap
                    %imports set
                ] [
                    ("export: duplicate definition") abort
                ]
            ] [
                drop drop
            ]
            body name
        ]
    ]

    define %before-definition-add [ ]
    define %after-definition-add [
        definition-resolve swap
        %maybe-export
        drop drop
    ]

    define reload [
        %mw-imports keys list-iterate [
            %mw-imports remove
        ]
    ]

    dict %module-imports

    lexical (import)
    define import [
        upquote
        list? if [
            list-iterate [
                resolve-mw-import-path const path
                path %mw-imports has if [
                    %mw-imports get swap drop
                    dict-keys
                    list-iterate [ dict-get %module-imports set ]
                    drop
                ] [
                    dict %imports
                    read-file eval

                    dict-empty
                    %imports keys list-iterate [
                        %imports get false? if [
                            drop
                            ("export: not defined") swap list-push abort
                        ] [
                            dict-set
                        ]
                    ]

                    path swap %mw-imports set
                    path %mw-imports get swap drop
                    dict-keys
                    list-iterate [ dict-get %module-imports set ]
                    drop
                ]
            ]
        ] [
            ("import name: not implemented") swap list-push abort
        ]
    ]

    define imports->map [
        dict ret
        %module-imports keys list-iterate [
            const k
            k %module-imports get
            ret set
            k %module-imports remove
        ]
        ret ->map
    ]

    %modbody eval

    ; take imports and make them available in the calling scope for module
    %module-imports keys list-iterate [
        %module-imports get swap
        quote definition-add
        ; add uplevels until it works
        ; (.. > module > modbody eval > list-iterate > while > .)?
        quote uplevel
        quote uplevel
        quote uplevel
        quote uplevel
        quote uplevel
        quote uplevel
        uplevel
    ]

    ; prepare to export using %maybe-export
    %exports get list-iterate [ #f %imports set ]
]
export-name module

import syntax/assign
quote := quote <- definition-rename
export-name <-

lexical (import)
define mdefn-eval [
    ; import mw/mdefn
    mdefn-cbody swap drop
    ; quote eval quote uplevel uplevel
    quote eval uplevel
]
; export-name mdefn-eval

; butchered from syntax/function
; function name (args ...) { body ... }
; name(args ...)
lexical (list-iterate)
define function [
    ; import mw/mdefn
    ; import overrides this
    quote on-mdefn-created definition-remove

    upquote const name
    upquote const args
    upquote const body

    ; prepend evaling args to body to get cbody
    body
    args list-iterate [
        list-push
        quote const list-push
    ]
    quote eval list-push
    quote upquote list-push
    const cbody

;     mdefn-empty
;     mdefn-kind-set-function
;     name mdefn-name-set
;     args mdefn-args-set
;     body mdefn-body-set
;     cbody mdefn-cbody-set

;     quote on-mdefn-created definition-resolve swap drop
;     false? swap drop if [] [ on-mdefn-created ]

;     [quote mdefn-eval uplevel] swap list-push
;     const fbody

    cbody const fbody

    fbody name 
    quote %before-definition-add definition-resolve swap drop
    false? swap drop if [] [ %before-definition-add ]

    updo definition-add+attributes

    quote %after-definition-add definition-resolve swap drop
    false? swap drop if [] [ name %after-definition-add ]
]
export-name function

; mildly different to function
; macro name (args ...) { body ... }
; name args ...
; if any of args are in an additional list, do not evaluate it.
; i.e. macro function([name] args body) { ... }
lexical (list-iterate)
define macro [
    ; import mw/mdefn
    quote on-mdefn-created definition-remove

    upquote const name
    upquote const args
    upquote const body

    body
    ; make body get args
    ; macro name(arg) { } -> [ quote evaluate uplevel const arg ]
    args list-reverse list-iterate [
        list? if [
            list-reverse list-iterate [
                [const uplevel quote quote]
                swap list-push
                list-reverse
                swap list-append
            ]
        ] [
            [const uplevel evaluate quote]
            swap list-push
            list-reverse
            swap list-append
        ]
    ]
    const cbody

;     mdefn-empty
;     mdefn-kind-set-macro
;     name mdefn-name-set
;     args mdefn-args-set
;     body mdefn-body-set
;     cbody mdefn-cbody-set

;     quote on-mdefn-created definition-resolve swap drop
;     false? swap drop if [] [ on-mdefn-created ]

;     [quote mdefn-eval uplevel] swap list-push
;     const fbody

    cbody const fbody

    fbody name 
    quote %before-definition-add definition-resolve swap drop
    false? swap drop if [] [ %before-definition-add ]

    updo definition-add+attributes

    quote %after-definition-add definition-resolve swap drop
    false? swap drop if [] [ name %after-definition-add ]
]
export-name macro

; vi: ft=scheme

