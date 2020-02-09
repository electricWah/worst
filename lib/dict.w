
import syntax/object

define-object-constructor dict [
    init [ map-empty ]
    method has [ %get swap map-exists dig drop ]
    method get [ %get swap map-get dig drop ]
    method get! [ get swap drop ]
    method set [ %get bury map-set %set ]
    method keys [ %get map-keys swap drop ]
    method ->map [ %get ]
]
export-name dict

; vi: ft=scheme

