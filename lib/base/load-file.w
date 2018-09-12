
[
    make-open-file-options open-file-read swap open-file
    file-info file-info-length swap drop 0 swap make-u8vector
    swap file-port 2 dig port-read
    u8vector-truncate u8vector->string
    2 negate dig drop drop
] quote file->string define

[
    file->string string->list make-place
    clone
    [
        [] place-swap
            list-empty? not swap drop
        2 negate dig place-swap drop drop
    ] swap list-push-head quote has-next? define
    [
        [] place-swap
            list-pop-head
        2 negate dig place-swap drop drop
    ] swap list-push-head quote get-next define

    [
        get-next
        quote interpreter-read-char
        quote uplevel
        uplevel
        read-all
    ] quote read-one define

    [
        has-next?
        quote read-one quote call-when
        uplevel
    ] quote read-all define
    read-all
    quote interpreter-read-eof
    uplevel
    quote interpreter-eval-read
    uplevel
] quote load-file define

;;; vi: ft=scheme

