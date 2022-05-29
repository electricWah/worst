
; define WORST_LIBPATH [["./lib"]]

define module-resolve-port [
    const module
    WORST_LIBPATH
    while [list-empty? not] [
        list-pop swap const %l

        "/" string-append
        module string-append
        ".w" string-append

        open-file/read
        can-read? if [ [] ] [ drop %l ]
    ]
    drop
    can-read? if [] [
        drop
        module
        ".w" string-append
        open-embedded-file/read
        can-read? if [] [
            module "module not found" error
        ]
    ]

]

define not [false? swap drop]

import {
    worst/misc
    data/list
    ui
}

worst-repl

; define egg (bean bnuy)
; define () pwgjpwg {"erigr "}
; pwgjpwg

; stack-dump

