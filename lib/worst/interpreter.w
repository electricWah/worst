
; Go through all current definitions and define them for this interpreter
define interpreter-inherit-definitions [
    updo all-definitions list-iter [
        clone
        quote definition-resolve updo uplevel ; TODO naming scopes or something
        swap interpreter-definition-add
    ]
]
export interpreter-inherit-definitions

