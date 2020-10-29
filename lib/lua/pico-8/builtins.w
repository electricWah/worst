
; define print [ #t make-lua-expr const s "print" lfcall 0 (s) ]

lua/extern print 1 0
lua/extern bxor 2 #t
lua/extern pset 3 0

export-all

; vi: ft=scheme

