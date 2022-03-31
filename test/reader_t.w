
import worst/misc
import assert_t
import worst/reader

define reads-as! [
    const src
    upquote const res

    new-string-port src port-write-string
    reader-new

    [] swap
    while [reader-next] [
        swap const reader
        list-push
        reader
    ]
    false? not if [ error ] [
        drop drop
        list-reverse
    ]

    res equal? if [ drop drop ] [ src "not equal" stack-dump error ]

]

; TODO read errors for chars other than t f
"#t" reads-as! (#t)
"#f" reads-as! (#f)
"#t#f" reads-as! (#t #f)
"#t #f#t\n#f" reads-as! (#t #f #t #f)

" ;skippo\n#t " reads-as! (#t)

"\"awooo\"" reads-as! ("awooo")
"\"\\\"\"" reads-as! ("\"")
"\"a b \\\"c\\\" d\"" reads-as! ("a b \"c\" d")
" \"awooo\"\"ban\nana\" \"coconut\"" reads-as! ("awooo" "ban\nana" "coconut")
"; boing\n\"hello\"\n" reads-as! ("hello")

; TODO next
"a thingy \n yea" reads-as! (a thingy yea)

"()" reads-as! (())
"()  {}  \n[]" reads-as! (() () ())

; " 4 45\n45.6" ; TODO not i32
" 4 45\n567" reads-as! (4 45 567)

"1;skip\n;skips again" reads-as! (1)
" ;skippo\n1 " reads-as! (1)
";skippy\n1; skips\n2 ;also skip 3\n" reads-as! (1 2)

; "1 2 #!3 #\n4 !# 5"  reads-as! (1 2 5) ; TODO shebang?
"(\"\\\"\")" reads-as! (("\""))

