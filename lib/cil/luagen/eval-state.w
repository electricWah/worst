
define cil/eval-state-setup [
    [] make-place const %stmts
    0 make-place const %indentation

    lexical (%indentation)
    define cil/indent> [ %indentation place-get 1 add place-set drop ]
    lexical (%indentation)
    define cil/indent< [ %indentation place-get 1 negate add place-set drop ]

    lexical (%stmts %indentation)
    define cil/emit-statement [
        %indentation place-get swap drop list-imake [ drop "    " ]
        swap list-append
        %stmts place-get dig list-push place-set drop
    ]

    lexical (%stmts)
    define %cil/statements [ %stmts place-get swap drop list-reverse ]

    quote cil/indent> definition-copy-up 
    quote cil/indent< definition-copy-up 
    quote cil/emit-statement definition-copy-up 
    quote %cil/statements definition-copy-up 

]
export-name cil/eval-state-setup

; vi: ft=scheme

