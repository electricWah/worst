
define abort [ quote abort current-error-handler ]
export-name abort

; evaluate = quote; call if symbol
define evaluate [ upquote symbol? if [quote call] [[] quote eval] uplevel ]
export-name evaluate

; a equals? b => a bool
define equals? [ upquote equal? swap drop ]
export-name equals?

; updo thing => quote thing uplevel
define updo [ upquote quote uplevel uplevel ]
export-name updo

define tailcall [
    upquote
    definition-resolve
    updo current-context-set-code
]

define while [
    upquote quote %%cond definition-add
    upquote quote %%body definition-add
    [
        %%cond if [%%body %%loop] [[]] current-context-set-code
    ] const %%loop
    %%loop current-context-set-code
]
export-name while

define print [ current-output-port swap port-write-string drop ]
export-name print

; vi: ft=scheme

