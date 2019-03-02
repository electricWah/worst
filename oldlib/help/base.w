
define-record-type* &help [info usage see-also]

make-hash-table swapvar *all-help*

define/enclose define-help [&help *all-help*] [
    quote^ local %help-topic-name
    quote^ ; help-body
    false swapvar %current-info
    false swapvar %current-usage
    false swapvar %current-see-also
    define info [ quote^ %current-info drop ]
    define usage [ quote^ %current-usage drop ]
    define see-also [ quote^ %current-see-also drop ]
    eval
    with-swapvar *all-help* [
        %help-topic-name
        quasiquote [
            unquote [false %current-info]
            unquote [false %current-usage]
            unquote [false %current-see-also]
        ] &help from-list
        hash-table-set
    ]
]

define/enclose help [&help *all-help*] [
    [Help topic? (hint: type "help" again)] ; mild hack: interactive prompt
    quote^ local topic
    drop
    with-swapvar *all-help* [
        topic hash-table-exists
        swap drop swap
    ]
    if [
        with-swapvar *all-help* [
            topic hash-table-get
            swap drop swap
        ]

        &help get info
        topic symbol->string " - " string-append
        swap string-append
        print-string/n

        &help get usage false equal? if [drop drop] [
            drop "Usage:" print-string/n
            datum-describe->string print-string/n drop
        ]

        &help get see-also false equal? if [drop drop] [
            drop "See also:" print-string/n
            datum-describe->string print-string/n drop
        ]
        drop
    ] [
        "No help defined for `"
        topic symbol->string string-append
        "'" string-append
        print-string/n
    ]
]

export-global define-help
export-global help

;;; vi: ft=scheme

