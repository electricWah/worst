
make-hash-table make-place local *record-commands*

[
    local record-commands
    local %field-names
    local %def-name
    [
        place->swapvar record-commands
        local %type
        list-length local %field-count
        local %field-names

        ; TODO put these in a hash and look it up instead of cond
        ; so you can extend the methods if you want

        quote^
        false record-commands swap
        hash-table-exists if [
            hash-table-get
            2 ~dig drop record-commands drop
            define rquote ['quote 'uplevel 'uplevel uplevel]
            eval-definition
        ] [ "Don't understand this record type subcommand" abort ]
    ]
    record-commands list-push-head
    %def-name make-record-type list-push-head
    %field-names list-push-head
    %def-name '%define uplevel
]
*record-commands* list-push-head
'%define-record-type* %define

[
    place->swapvar record-commands
    list->definition
    false record-commands
    2 ~dig hash-table-set
    record-commands drop
]
*record-commands* list-push-head
'%define-record-command %define

define define-record-type* [quote^ quote^ '%define-record-type* uplevel]

define define-record-command [quote^ quote^ '%define-record-command uplevel]

define-record-command from-list [
    list-length %field-count equal? if [drop drop] [
        drop drop
        %field-names "Not enough fields given" abort
    ]
    %type make-record
    swap
    define build [
        list-empty? [ drop ] [
            list-pop-head
            2 dig swap
            record-slot-add
            swap
            build
        ] %if
    ]
    build
]

define-record-command is? [
    record? if [
        record-type %type equal? 2 ~dig drop drop
    ] [ false ]
]

; TODO merge swap, get, set

define-record-command swap [
    rquote local %field-name
    define find-index [
        local i
        list-empty? [
            drop %field-names %field-name "No such field" abort
        ] [
            list-pop-head
            %field-name equal? [
                drop drop drop i
            ] [
                drop drop
                i 1 add
                find-index
            ] %if
        ] %if
    ]
    %field-names 0 find-index
    record-slot-swap
]

define-record-command get [
    rquote local %field-name
    define find-index [
        local i
        list-empty? [
            drop %field-names %field-name "No such field" abort
        ] [
            list-pop-head
            %field-name equal? [
                drop drop drop i
            ] [
                drop drop
                i 1 add
                find-index
            ] %if
        ] %if
    ]
    %field-names 0 find-index
    local idx
    false idx record-slot-swap
    clone 2 ~dig
    idx record-slot-swap
    drop swap
]

define-record-command set [
    rquote local %field-name
    define find-index [
        local i
        list-empty? [
            drop %field-names %field-name "No such field" abort
        ] [
            list-pop-head
            %field-name equal? [
                drop drop drop i
            ] [
                drop drop
                i 1 add
                find-index
            ] %if
        ] %if
    ]
    %field-names 0 find-index
    record-slot-swap
    drop
]

; define-record-type &record [field1 field2 ...]
; &record from-list : [field1 field2 ...] -> &record
; &record is? : * -> * bool
; &record get field1 : &record -> &record field1
; &record set field1 : &record val -> &record
; &record swap field1 : &record val1 -> &record val2

export-global %define-record-type*
export-global define-record-type*

;;; vi: ft=scheme

