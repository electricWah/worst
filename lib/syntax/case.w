
; case - it's cond, but it's called case.

; case {
;   (-> bool) { if true ... } ; any number of these...
;   #t { default }
; }

define case [
    upquote
    while (list-empty? not) {
        list-pop const %if
        list-pop const %then
        const %cases
        %if eval if [
            %then eval []
        ] [
            %cases
        ]
    }
    drop
]
export-name case

; vi: ft=scheme

