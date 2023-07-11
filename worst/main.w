
command-line-arguments list-pop drop ; $0
list-empty? if [
    drop
    worst-repl
] [
    list-pop const path const args
    path
    file-open-options file-open-options-set-read
    file-open
    error? if [
        drop args path string->symbol cli-module-run
    ] [
        ; jank to get ui/cli and import "relative.w" working
        path const current-script-path
        read-port->list eval
    ]
]

