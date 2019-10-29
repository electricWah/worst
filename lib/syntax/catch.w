
; catch [ body ] [ on error ]
; a basic error handling syntax

define catch [
    upquote const %catch-body
    upquote const %catch-on-error

    define %catch []
    define current-error-handler [
        quote %catch updo definition-exists
        swap drop if [
            %catch-on-error
        ] [
            [
                [updo current-error-handler] updo current-context-set-code
                updo current-context-remove-children
            ]
        ]
        updo eval
    ]
    %catch-body eval
]
export-name catch

; vi: ft=scheme

