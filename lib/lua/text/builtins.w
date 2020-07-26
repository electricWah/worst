
; precedence:
; 0     variables, a.b, f(), a:b(), the a in a[b], (expr)
; 1     ^   (right associative - take care)
; 2     not # - (unary)
; 3     * / %
; 4     + -
; 5     ..  (right associative but probably doesn't matter)
; 6     < > <= >= ~= ==
; 7     and
; 8     or
; 10    return, assignment, the b in a[b]

define and [ 2 lua-expect-values " and " 7 lua-binop ]
export-name and



; vi: ft=scheme

