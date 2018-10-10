
;;; vi: ft=scheme

; with-stty [stty options ...] [body ...]
; run [body ...] with given stty options, then reset
define with-stty [
    quote^ local %options
    quote^ list->definition local %body

    ; take current stty values
    "stty" make-command "-g" command-add-argument
    command-stdout-pipe command-spawn
    process-stdout-port 0 1024 make-u8vector port-read
    1 negate add ; remove \n
    u8vector-truncate u8vector->string
    local %stty-restore
    drop drop drop

    ; stty with given options
    "stty" make-command
    %options [ symbol->string command-add-argument ] list-iter
    command-stdout-inherit command-spawn
    process-wait
    drop drop

    %body eval-definition

    ; restore stty
    "stty" make-command %stty-restore command-add-argument
    command-stdout-inherit command-spawn
    process-wait
    drop drop

]

export-global with-stty

