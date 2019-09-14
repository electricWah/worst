
; import doc
import syntax/attributes
import syntax/object

define dict [
    @[
    ; documentation [
    ;     title "Mutable hash-table object constructor"
    ;     usage "dict name"
    ;     example "
    ; dict test
    ; 5 6 test set
    ; 5 test get"
    ;     section types
    ; ]
    object-constructor [
        init [ hash-table-empty ]
        method has [ %get swap hash-table-exists dig drop ]
        method get [ %get swap hash-table-get dig drop ]
        method get! [ get swap drop ]
        method set [ %get bury hash-table-set %set ]
        method keys [ %get hash-table-keys swap drop ]
        method ->hash-table [ %get ]
    ]
    ]
]
export dict

; vi: ft=scheme

