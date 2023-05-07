
define package [

    ; add to WORST_LIBPATH basically
    define path [
        upquote module-search-path-prepend
    ]

    ; same as ui/ansi but maybe only allow above defs
    updo current-defenv
    defenv-new-locals
    current-defenv
    defenv-merge-locals
    upquote swap value-set-defenv updo eval
]
export package

