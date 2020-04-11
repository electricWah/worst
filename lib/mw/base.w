
import list

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

lexical (export-name import list-iterate if)
define module [
    import dict
    import syntax/variable

    upquote const %modbody

    [] variable %exports

    ; resolve-import-path .w -> .mw
    define resolve-mw-import-path [
        symbol? if [
            ->string
            WORST_LIBDIR "/" string-append
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
    define %maybe-export [
        %imports has if [
            %imports get false? if [
                drop
                swap
                %imports set
            ] [
                ("export: duplicate definition") abort
            ]
        ] [
            drop
        ]
    ]

    lexical (import)
    define import [
        upquote
        list? if [
            list-iterate [
                resolve-mw-import-path
                read-file eval
            ]
        ] [
            ("import name: not implemented") swap list-push abort
        ]
    ]

    dict %imports
    %modbody eval

    ; take imports and make them available in the calling scope for module
    %imports keys list-iterate [
        %imports get swap
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

    quote %imports definition-remove
    ; now %imports should refer to the importing module
    %exports get list-iterate [ #f %imports set ]
]
export-name module

import syntax/assign
quote := quote <- definition-rename
export-name <-

; butchered from syntax/function
; function name (args ...) { body ... }
; name(args ...)
lexical (import list-iterate if)
define function [
    upquote const name
    upquote const args
    upquote const body

    body
    ; make body get args and eval them
    args list-iterate [
        list-push
        quote const list-push
    ]
    quote eval list-push
    quote upquote list-push

    ; add meta information:
    ; [%meta function <name> <args>] drop
    quote drop list-push
    [function %meta]
    name list-push
    args list-push
    list-reverse
    list-push

    const body

    quote %maybe-export definition-resolve swap drop
    false? if [drop] [
        drop body name %maybe-export
    ]

    body name updo definition-add
]
export-name function

; mildly different to function
; macro name (args ...) { body ... }
; name args ...
lexical (import list-iterate if)
define macro [
    upquote const name
    upquote const args
    upquote const body

    body
    ; make body get args
    ; macro name(arg) { } -> [ quote evaluate uplevel const arg ]
    args list-reverse list-iterate [
        [const uplevel evaluate quote]
        swap list-push
        list-reverse
        swap list-append
    ]

    ; add meta
    ; [%meta macro <name> <args>] drop
    quote drop list-push
    [macro %meta]
    name list-push
    args list-push
    list-reverse
    list-push

    const body

    quote %maybe-export definition-resolve swap drop
    false? if [drop] [
        drop body name %maybe-export
    ]

    body name updo definition-add
]
export-name macro


; vi: ft=scheme

