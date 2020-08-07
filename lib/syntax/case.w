
; case - it's cond, but it's called case.

; case {
;   (-> bool) { if true ... } ; any number of these...
;   #t { default }
; }

documentation [
    title "A syntax block remeniscent of if-elseif-else chains"
    usage "case { (-> bool) { if true } ... #t { default } }"
    example
    "5 case ((equals? 6) (\"It's 6\") (equals? 5) (\"Five!\") #t (\"???\"))"
    tags (syntax)
]
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

