
define default-attributes [documentation-attribute]

import {
    worst/misc
    worst/multi
    syntax/case
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
        const path
        path open-file/read
        false? if [
            ; TODO nicer error
            drop path pause
        ] [
            ; TODO load module
            read-port->list eval
        ]
    }
]

; define egg (bean bnuy)
; define () pwgjpwg {"erigr "}
; pwgjpwg

; stack-dump

