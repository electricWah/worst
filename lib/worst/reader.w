
doc "input-port reader-new -> reader"
define reader-new [

    new-string-port const %stdin-buffer
    ; buffers stdin line-by-line into %stdin-buffer
    define buffered-input-port [
        %stdin-buffer
        port-peek-char
        false? swap drop if [
            current-input-port port-read-line swap drop
            port-write-string
        ] [
        ]
    ]

    ; read-one -> value #t | continue? #f
    define read-one [
        while [
            buffered-input-port
            port-peek-char
            cond [
                [false?] [
                    ; leave loop, don't continue
                    drop drop drop #f #f #f
                ]
                ; newline: leave loop, maybe continue
                ["\n" equal? swap drop] [
                    drop port-read-char drop drop
                    #t #f #f
                ]
                ; drop whitespace
                ; [ "%s" string-contains-match? ] [
                ;     drop port-read-char drop drop
                ;     #t
                ; ]
                ; anything else: read a value, leave loop
                [#t] [
                    drop
                    port-read-value
                    swap drop
                    #t #f
                ]
            ]
        ] []
    ]

    interpreter-empty
    interpreter-inherit-definitions
    const interp

    interp
]

define reader-next [
]



