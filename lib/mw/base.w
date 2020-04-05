
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

lexical (export-name)
define module [
    import lib/dict

    upquote const %modbody

    [] variable %exports

    ; override: .w -> .mw
    define resolve-import-path [
        symbol? if [
            ->string
            WORST_LIBDIR "/" string-append
            swap string-append
            ".mw" string-append
        ] [ "resolve-import-path: invalid" abort ]
    ]

    define export [
        upquote list? if [
            %exports get
            list-append
            %exports set
        ] [
            "export name: not implemented"
        ]
    ]

    define import [
        upquote list? if [
            list-iterate [
                resolve-import-path
                read-file eval
            ]
        ] [
            "import name: not implemented"
        ]
    ]

    ; body name %maybe-export
    define %maybe-export [
        %imports has if [
            %imports get false? if [
                drop
                definition-resolve
                %imports set
            ] [
                "export: duplicate definition" abort
            ]
        ] []
    ]

    dict %imports
    %modbody eval
    %imports ->map updo definition-import-map
    quote %imports definition-remove
    ; now %imports should refer to the importing module
    %exports list-iterate [ #f %imports set ]
]
export-name module

import syntax/assign
quote := quote <- definition-rename
export-name <-

; butchered from syntax/function
; function name (args ...) { body ... }
; name(args ...)
define function [
    import list
    upquote const name
    upquote const args
    upquote const body

    body
    ; make body get args
    args list-iterate [
        list-push
        quote const list-push
    ]
    ; eval arg list
    quote eval list-push
    quote upquote list-push

    name updo definition-add

    quote %maybe-export definition-resolve swap drop
    false? if [drop] [ drop name %maybe-export ]
]
export-name function

; mildly different to function
; macro name (args ...) { body ... }
; name args ...
define macro [
    import list
    upquote const name
    upquote const args
    upquote const body

    body
    ; make body get args
    args list-reverse list-iterate [
        list-push
        quote const list-push
        quote upquote list-push
    ]

    name updo definition-add
    quote %maybe-export definition-resolve swap drop
    false? if [drop] [ drop name %maybe-export ]
]
export-name macro


; vi: ft=scheme

