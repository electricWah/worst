
; Top version is slower

; import syntax/object

; define-object-constructor dict [
;     init [ dict-empty ]
;     method has [ %get swap dict-exists dig drop ]
;     method get [ %get swap dict-get dig drop ]
;     method get! [ get swap drop ]
;     method set [ %get bury dict-set %set ]
;     method keys [ %get dict-keys swap drop ]
;     method ->map [ %get ]
; ]
; export-name dict

import syntax/attribute

dict-empty
 quote has [ swap dict-exists dig drop ] dict-set
quote get [ swap dict-get dig drop ] dict-set
quote get! [ swap dict-get bury drop drop ] dict-set
quote set [ bury dict-set drop ] dict-set
quote remove [ swap dict-remove drop ] dict-set
quote keys [ dict-keys swap drop ] dict-set
quote ->map [] dict-set
const %dict-methods

lexical (%dict-methods)
define dict [
    upquote const name
    [
        upquote
        dict-get
        false? if [drop ("not a dict method") swap list-push abort] [
            bury drop drop eval
        ]
    ]
    %dict-methods list-push
    dict-empty list-push

    name
    updo definition-add
]
export-name dict

; vi: ft=scheme

