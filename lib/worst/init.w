
define default-attributes [documentation-attribute]

import {
    worst/misc
    syntax/case
    data/list
}

command-line-arguments list-pop drop ; $0
case [
    (list-empty?) {
        drop
        import ui
        worst-repl
    }
    #t {
        list-pop swap drop
        open-file/read read-port->list eval
    }
]

; define egg (bean bnuy)
; define () pwgjpwg {"erigr "}
; pwgjpwg

; stack-dump

