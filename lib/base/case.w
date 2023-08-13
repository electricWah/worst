
; case {
;   (-> bool) { if true ... } ; any number of these...
;   #t { default }
; }

define [
    doc [
        title "A syntax block remeniscent of if-elseif-else chains"
        usage "case { (-> bool) { if true } ... #t { default } }"
        example
        "5 case ((equals? 6) (\"It's 6\") (equals? 5) (\"Five!\") #t (\"???\"))"
        tags (syntax)
    ]
]
case [
    updo current-defs const env
    upquote
    while (list-empty? not) {
        list-pop const %if
        list-pop const %then
        const %cases
        %if env value-set-ambients eval if [
            %then env value-set-ambients eval []
        ] [
            %cases
        ]
    }
    drop
]
export case

