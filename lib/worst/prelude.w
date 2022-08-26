
; Not really a standard library, more like a random bag of helpful stuff

define ' [ upquote ]
; updo thing => quote thing uplevel
define updo [ upquote quote uplevel uplevel ]

; a b clone2 => a b a b
define clone2 [ swap clone dig clone bury ]

define (dispatch ((i64? i64?) stack-matches?)) add [ i64-add ]
define (dispatch ((f64? f64?) stack-matches?)) add [ f64-add ]
define (dispatch ((i64? i64?) stack-matches?)) sub [ i64-sub ]
define (dispatch ((f64? f64?) stack-matches?)) sub [ f64-sub ]
define (dispatch ((i64? i64?) stack-matches?)) mul [ i64-mul ]
define (dispatch ((f64? f64?) stack-matches?)) mul [ f64-mul ]
define (dispatch ((i64? i64?) stack-matches?)) div [ i64-div ]
define (dispatch ((f64? f64?) stack-matches?)) div [ f64-div ]

define (dispatch (i64?)) negate [ i64-negate ]
define (dispatch (f64?)) negate [ f64-negate ]
define (dispatch (i64?)) abs [ i64-abs ]
define (dispatch (f64?)) abs [ f64-abs ]

define (dispatch ((i64? i64?) stack-matches?)) le [ i64-le ]
define (dispatch ((f64? f64?) stack-matches?)) le [ f64-le ]
define (dispatch ((i64? i64?) stack-matches?)) lt [ i64-lt ]
define (dispatch ((f64? f64?) stack-matches?)) lt [ f64-lt ]
define (dispatch ((i64? i64?) stack-matches?)) ge [ i64-ge ]
define (dispatch ((f64? f64?) stack-matches?)) ge [ f64-ge ]
define (dispatch ((i64? i64?) stack-matches?)) gt [ i64-gt ]
define (dispatch ((f64? f64?) stack-matches?)) gt [ f64-gt ]

define (dispatch ((list? list?) stack-matches?)) append [ list-append ]
define (dispatch ((string? string?) stack-matches?)) append [ string-append ]

; maybe these should eval, so you can do [5 le? (4 3 add)]
define equal? [ clone2 equal ]
; a <op> b => a bool
define equals? [ clone upquote equal ]
define lt? [ clone upquote lt ]
define le? [ clone upquote le ]
define gt? [ clone upquote gt ]
define ge? [ clone upquote ge ]

define not [ false? swap drop ]
define abs [ lt? 0 if [negate] [] ]
define max [ clone2 lt if [swap] [] drop ]
define min [ clone2 lt if [] [swap] drop ]

define (dispatch (embedded-file-port?))
port->string [ embedded-file-port->string ]
define (dispatch (file-port?))
port->string [ file-port->string ]

define read-port->list [ port->string read-string->list ]

define ->string [ value->string ]

define print [ current-output-port swap port-write-string port-flush drop ]
define print-value [ ->string print ]
define println [ ->string "\n" string-append print ]

define read-line [ current-input-port buffered-port-read-line swap drop ]

export #t

