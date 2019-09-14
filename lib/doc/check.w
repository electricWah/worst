
define check-docs [
    import syntax/variable
    import syntax/assign
    import doc

    define check-doc [
        const name

        define warn [
            ansi [
                bold red fg "Warning: " print
                reset print
                ": " print
                reset yellow fg name ->string print
                "\n" print
                reset
            ]
        ]

        define sets [ updo upquote drop #t upquote call set ]
        define incr [
            updo upquote drop
            upquote const n
            n call get
            1 add
            n call set
        ]

        #f variable has-title
        #f variable has-description
        #f variable has-usage
        #f variable has-section
        0 variable num-tags

        0 variable examples
        #f variable no-check

        name has-documentation? if [
            doc-eval [
                define title [sets has-title]
                define description [sets has-description]
                define usage [sets has-usage]
                define section [sets has-section]
                define tags [ upquote list-length num-tags set drop ]
                define example [incr examples]
                define internal [ #t no-check set ]
                define undocumented [ #t no-check set ]
            ]

            no-check get if [ ] [
                has-title get if [ ] [ "no title" warn ]
                has-description get if [ ] [ "no description" warn ]
                has-usage get if [ ] [ "no usage" warn ]
                has-section get if [ ] [ "no section" warn ]

                ; num-tags get 1 lt? swap drop if [
                ;     warn("need more tags: want 1, have "
                ;         swap ->string string-append)
                ; ] [ drop ]

                ; examples get 2 lt? swap drop if [
                ;     warn("need more examples: want 2, have "
                ;         swap ->string string-append)
                ; ] [ drop ]
            ]
            ; drop
        ] [
            "missing documentation" warn
            drop
        ]
    ]

    defined-names variable names
    define next [ names get list-empty? if [#f] [list-pop swap names set] ]
    while [next equals? #f not] [check-doc] drop drop
]
export check-docs

; vi: ft=scheme


