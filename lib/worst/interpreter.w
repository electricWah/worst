
import data/map

; Go through all current definitions and define them for this interpreter
define interpreter-inherit-definitions [
    all-definitions map-iterate [ swap interpreter-definition-add ]
]
export interpreter-inherit-definitions

