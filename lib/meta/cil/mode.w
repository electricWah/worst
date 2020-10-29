
; Emit modes for a function doing multiple things.
; mostly,
; - 'normal' when cil is not involved (#f)
; - 'emitting' when you expect thingio (emit)
; - maybe different modes for different optimisation levels?
; TODO probably just make this #t/#f
;   and set other things in different parameters.

define cil/emit-mode [
    quote %cil/emit-mode definition-resolve swap drop false? if [] [eval]
]
export-name cil/emit-mode

lexical (const)
define cil/set-emit-mode [
    const %cil/emit-mode
    quote %cil/emit-mode definition-copy-up
]
export-name cil/set-emit-mode

define-attribute cil/escaping-emit-mode [
    before [
        const %defname
        [
            cil/emit-mode
            #f cil/set-emit-mode
            const %cil/escaping-emit-mode-orig
        ] swap list-append
        const %defoverride
        
        %defname definition-resolve swap drop
        false? if [
            %defoverride
        ] [
            const %deforig
            lexical (%deforig %defoverride)
            define d [
                cil/emit-mode false? swap drop
                if [ %deforig ] [ %defoverride ]
                updo eval
            ]
            quote d definition-resolve
            swap drop
        ]
        %defname
    ]
]
export-name cil/escaping-emit-mode

define cil/reenter-emit-mode [
    [%cil/escaping-emit-mode-orig cil/set-emit-mode]
    upquote
    list-append
    eval
]
export-name cil/reenter-emit-mode

; vi: ft=scheme

