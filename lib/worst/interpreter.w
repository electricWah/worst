
; Go through all current definitions and define them for this interpreter
define interpreter-inherit-definitions [
    import data/map
    all-definitions map-iterate [ swap interpreter-definition-add ]
]

export-name interpreter-inherit-definitions

