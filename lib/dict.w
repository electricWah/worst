
import syntax/object

define-object-constructor dict [
    init [ hash-table-empty ]
    method has [ %get swap hash-table-exists dig drop ]
    method get [ %get swap hash-table-get dig drop ]
    method get! [ get swap drop ]
    method set [ %get bury hash-table-set %set ]
    method keys [ %get hash-table-keys swap drop ]
    method ->hash-table [ %get ]
]
export-name dict

; vi: ft=scheme

