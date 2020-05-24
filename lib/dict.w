
; Top version is slower

; import syntax/object

; define-object-constructor dict [
;     init [ map-empty ]
;     method has [ %get swap map-exists dig drop ]
;     method get [ %get swap map-get dig drop ]
;     method get! [ get swap drop ]
;     method set [ %get bury map-set %set ]
;     method keys [ %get map-keys swap drop ]
;     method ->map [ %get ]
; ]
; export-name dict

import syntax/attribute

map-empty
 quote has [ swap map-exists dig drop ] map-set
quote get [ swap map-get dig drop ] map-set
quote get! [ swap map-get bury drop drop ] map-set
quote set [ bury map-set drop ] map-set
quote remove [ swap map-remove drop ] map-set
quote keys [ map-keys swap drop ] map-set
quote ->map [] map-set
const %dict-methods

lexical (%dict-methods)
define dict [
    upquote const name
    [
        upquote
        map-get
        false? if [drop ("not a dict method") swap list-push abort] [
            bury drop drop eval
        ]
    ]
    %dict-methods list-push
    map-empty list-push

    name
    updo definition-add
]
export-name dict

; vi: ft=scheme

