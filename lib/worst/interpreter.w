
import data/pairs

; Go through all current definitions and define them for this interpreter
define interpreter-inherit-definitions [
    updo all-definitions pairs-iter [ interpreter-definition-add ]
]
export interpreter-inherit-definitions

