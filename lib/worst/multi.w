
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

define (dispatch ((i64? i64?) stack-matches?)) comple [ i64-comple ]
define (dispatch ((f64? f64?) stack-matches?)) comple [ f64-comple ]
define (dispatch ((i64? i64?) stack-matches?)) complt [ i64-complt ]
define (dispatch ((f64? f64?) stack-matches?)) complt [ f64-complt ]
define (dispatch ((i64? i64?) stack-matches?)) compge [ i64-compge ]
define (dispatch ((f64? f64?) stack-matches?)) compge [ f64-compge ]
define (dispatch ((i64? i64?) stack-matches?)) compgt [ i64-compgt ]
define (dispatch ((f64? f64?) stack-matches?)) compgt [ f64-compgt ]

define (dispatch ((list? list?) stack-matches?)) append [ list-append ]
define (dispatch ((string? string?) stack-matches?)) append [ string-append ]

export #t

