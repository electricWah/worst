
define-record-type* &help [info usage see-also]

'&help take-definition local *help*

make-hash-table make-place local *all-help*

[
    '&help add-definition
    place->swapvar %all-help

    quote^ local %help-topic-name
    quote^ ; help-body
    false swapvar %current-info
    false swapvar %current-usage
    false swapvar %current-see-also
    define info [ quote^ %current-info drop ]
    define usage [ quote^ %current-usage drop ]
    define see-also [ quote^ %current-see-also drop ]
    eval
    false %all-help
    %help-topic-name
    quasiquote [
        unquote [false %current-info]
        unquote [false %current-usage]
        unquote [false %current-see-also]
    ] &help from-list
    hash-table-set %all-help drop
]
*help* list-push-head
*all-help* list-push-head
'define-help %define

[
    '&help add-definition
    place->swapvar %all-help
    [Help topic? (hint: type "help" again)] ; mild hack: interactive prompt
    quote^
    swap drop
    false %all-help
    swap
    hash-table-exists
    if [
        hash-table-get swap
        symbol->string " - " string-append swap
        &help get info 2 dig swap string-append print-string/n
        "Usage:" print-string/n
        &help get usage datum-describe->string print-string/n drop
        "See also:" print-string/n
        &help get see-also datum-describe->string print-string/n drop
        drop
    ] [
        symbol->string "No help defined for `"
        swap string-append
        "'" string-append
        print-string/n
    ]
    %all-help drop
]
*help* list-push-head
*all-help* list-push-head
'help %define

export-global define-help
export-global help

;;; vi: ft=scheme

