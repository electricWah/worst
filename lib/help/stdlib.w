
define-help frogs [
    info "Frogs"
    usage [
        frogs
    ]
    see-also [more-frogs]
]

define-help help [
    info "Show help for the given definition"
    usage [
        help <symbol>
    ]
    see-also [help-topics define-help]
]

define-help define [
    info "Define a function in the current context"
    usage [define <symbol> <list>]
    ; example [define incr [1 add]]
    ; see-also [%define]
]

;;; vi: ft=scheme

