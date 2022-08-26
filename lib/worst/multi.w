
define (dispatch ((i64? i64?) stack-matches?)) add [ i64-add ]
define (dispatch ((f64? f64?) stack-matches?)) add [ f64-add ]
define (dispatch ((i64? i64?) stack-matches?)) sub [ i64-sub ]
define (dispatch ((f64? f64?) stack-matches?)) sub [ f64-sub ]
define (dispatch ((i64? i64?) stack-matches?)) mul [ i64-mul ]
define (dispatch ((f64? f64?) stack-matches?)) mul [ f64-mul ]
define (dispatch ((i64? i64?) stack-matches?)) div [ i64-div ]
define (dispatch ((f64? f64?) stack-matches?)) div [ f64-div ]

define (dispatch ((i64?) stack-matches?)) negate [ i64-negate ]
define (dispatch ((f64?) stack-matches?)) negate [ f64-negate ]
define (dispatch ((i64?) stack-matches?)) abs [ i64-abs ]
define (dispatch ((f64?) stack-matches?)) abs [ f64-abs ]

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

define (dispatch ((embedded-file-port?) stack-matches?))
port->string [ embedded-file-port->string ]
define (dispatch ((file-port?) stack-matches?))
port->string [ file-port->string ]

export #t

