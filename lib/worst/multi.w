
define (dispatch ((f64? f64?) stack-matches?)) add [ f64-add ]
define (dispatch ((i64? i64?) stack-matches?)) add [ i64-add ]

define (dispatch ((list? list?) stack-matches?)) append [ list-append ]
define (dispatch ((string? string?) stack-matches?)) append [ string-append ]

export #t

