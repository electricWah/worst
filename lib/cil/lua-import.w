
; Override import to find Lua files to replace their equivalent Worst modules

; Gives `import modpath` additional behaviour.
; If the file modpath.w.lua exists, it is luaded instead of modpath.w
; It is loaded and treated as a Lua chunk with one argument (the interpreter)

; Example contents:
;; local interp = ...
;; local base = require "base"
;; interp:define(base.Symbol.new "hello", function(i)
;;    i:stack_push("Hello " .. i:stack_pop("string"))
;; end)

lexical-alias import-file import-file/orig
define import-file [

    define resolve-eval-lua-import [
        weakly define WORST_LUA_CACHE_PATH [[]]

        WORST_LUA_CACHE_PATH WORST_LIBPATH list-append
        const WORST_LIBPATH

        lexical-alias import-path->file-name orig
        define import-path->file-name [ orig ".lua" string-append ]

        ; TODO what if one of these fails?
        resolve-import-path false? if [] [
            const filepath
            filepath
            open-input-file false? if [] [
                port-read-all false? if [] [
                    swap drop
                    lua-load-string
                    false? if [
                        drop
                        [] swap list-push
                        filepath list-push
                        abort
                    ] [ ]
                ]
            ]
        ]
    ]

    const %%import
    %%import resolve-eval-lua-import false? if [
        drop %%import import-file/orig
        current-context-definitions
    ] [ eval current-context-definitions ]

    quote current-context-define-all uplevel

]
export-name import-file

; vi: ft=scheme

