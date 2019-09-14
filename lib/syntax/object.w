
; import doc ; oops, doc requires this!
import syntax/attributes
import syntax/variable
import list

define object-constructor [
    @[lexical list-quasiquote]
      ; documentation [
      ;   title "Create a named value with accessor and mutator methods"
      ;   description "Turns the enclosing define form into a constructor"
      ;   usage "define ctor [@[object-constructor [define method [...] ...]]]"
      ;   example "
    ; define variable [
      ;   @[object-constructor [
      ;       method get [%get]
      ;       method set [%set]
      ;   ]]
    ; ]

    ; 6 variable n
    ; n get ->string print
    ; 12 n set"
    ; ]
    import syntax/variable
    import list

    define-attribute?
    if [] ["object-constructor must be used as an attribute" abort]

    list-empty? if [drop] [
        "object-constructor: define body must be empty" abort
    ]
    const dname

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
        *[init-expr get]
        ^[
            make-place const P
            upquote const name
        ]
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
            name updo definition-add
        ]
    ]
    dname swap
]
export object-constructor

; vi: ft=scheme

