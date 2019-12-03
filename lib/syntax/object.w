
define define-object-constructor [
      ; documentation [
      ;   title "Create a named value with accessor and mutator methods"
      ;   description "Turns the enclosing define form into a constructor"
      ;   example "
    ; define-object-constructor variable [
      ;       method get [%get]
      ;       method set [%set]
    ; ]

    ; 6 variable n
    ; n get ->string print
    ; 12 n set"
    ; ]
    import syntax/variable
    import list

    upquote const dname

    [] variable methods
    [] variable init-expr

    upquote
    [
        define method [
            upquote const name
            upquote const body
            list-quasiquote [
                *[methods get]
                ~ define ~[name] ~[body]
            ] methods set
        ]
        define init [
            upquote init-expr set
        ]
        eval
    ] eval

    list-quasiquote [
        ^[upquote const %name]
        *[init-expr get] ^[make-place const P]
        ~[list-quasiquote [
            ^[const P]
            *[methods get]
            ^[upquote
                define %get [ P place-get swap drop ]
                define %set [ P swap place-set drop ]
                definition-get false? if [
                    "not recognised" abort
                ] [
                    swap drop eval
                ]
            ]
        ]]
        ^[
            P list-push
            %name updo definition-add
        ]
    ]
    dname
    updo definition-add+attributes
]
export-name define-object-constructor

; vi: ft=scheme

