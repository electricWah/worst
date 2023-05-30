
define check-docs [
    import ui/ansi

    0 make-place const warnings
    define check-doc [
        const def
        const name

        define incr [ clone place-get 1 add place-set drop ]

        define warn [
            warnings incr
            ansi [
                bold red fg "Warning: " print
                reset print
                ": " print
                reset yellow fg name println-value
                reset
            ]
        ]

        #f make-place const has-title
        #f make-place const has-description
        #f make-place const has-usage
        #f make-place const has-section
        0 make-place const num-tags

        0 make-place const examples
        #f make-place const no-check

        def value-doc false? not if [
            [
                define title [has-title #t place-set drop]
                define description [has-description #t place-set drop]
                define usage [has-usage #t place-set drop]
                define section [has-section #t place-set drop]
                define tags [ upquote list-length num-tags swap place-set drop ]
                define example [examples incr]
                define internal [ #t no-check place-set drop ]
                define undocumented [ #t no-check place-set drop ]
                current-defenv
            ] eval
            value-set-defenv eval


            no-check place-get if [ ] [
                has-title place-get if [ ] [ "no title" warn ]
                has-description place-get if [ ] [ "no description" warn ]
                has-usage place-get if [ ] [ "no usage" warn ]
                has-section place-get if [ ] [ "no section" warn ]

                ; num-tags place-get 1 lt? swap drop if [
                ;     warn("need more tags: want 1, have "
                ;         swap value->string string-append)
                ; ] [ drop ]

                ; examples place-get 2 lt? swap drop if [
                ;     warn("need more examples: want 2, have "
                ;         swap value->string string-append)
                ; ] [ drop ]
            ]
            ; drop
        ] [
            "missing documentation" warn
            drop
        ]
    ]

    updo current-defenv const defenv
    defenv defenv-names-all list-iter [
        clone defenv swap defenv-lookup
        check-doc
    ]
    warnings place-get gt? 0 if [
        ansi [ print-value bright red fg " warnings." println reset ]
    ] [ ]
]
export check-docs

import ui/cli
cli-run [ check-docs ]

