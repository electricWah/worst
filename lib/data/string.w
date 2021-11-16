
; string pattern string-contains-match -> string #t|#f
define string-contains-match? [
    string-global-matches
    list-empty? not swap drop
]
export string-contains-match?

; vi: ft=scheme

