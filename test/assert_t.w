
export #t

define equal! [
    const %msg
    upquote const %expr
    %expr eval
    equal? if [ drop drop ] [ %msg stack-dump error ]
    while [stack-empty not] [drop]
]

define test! [
    const %msg
    upquote const %expr
    %expr eval
    #t equal? if [ drop drop ] [ %msg stack-dump error ]
    while [stack-empty not] [drop]
]

