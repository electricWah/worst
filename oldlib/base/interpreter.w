
;;; vi: ft=scheme

define interpreter-get-reader [
    '<> make-reader
    interpreter-swap-reader
    clone swapvar %reader
    interpreter-swap-reader
    %reader
]

export-global interpreter-get-reader

