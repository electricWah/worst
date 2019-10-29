
; Bunch of stuff that isn't necessary in worst.rkt but is here
; I'll be honest, not sure whether to put as much as possible here
; or make bare rworst more useful

define-racket-builtin list?
    (lambda (c s) (values c (cons (list? (stack-top s)) s)))
define-racket-builtin string?
    (lambda (c s) (values c (cons (string? (stack-top s)) s)))
define-racket-builtin number?
    (lambda (c s) (values c (cons (number? (stack-top s)) s)))
define-racket-builtin boolean?
    (lambda (c s) (values c (cons (boolean? (stack-top s)) s)))
define-racket-builtin vector?
    (lambda (c s) (values c (cons (vector? (stack-top s)) s)))

export-name list?
export-name string?
export-name number?
export-name boolean?
export-name vector?

define-racket-builtin interpreter-dump-stack
    (lambda (c s) (eprintf "Stack:\n~S\n" s) (values c s))
define-racket-builtin interpreter-stack
    (lambda (c s) (values c (cons s s)))
define-racket-builtin interpreter-stack-set
    (lambda (c s) (values c (stack-top s list?)))
export-name interpreter-dump-stack
export-name interpreter-stack
export-name interpreter-stack-set

; define-racket-builtin greater
;     (lambda (c s) (values c
;                           (cons (> (stack-top s number?)
;                                    (stack-top (cdr s) number?))
;                                 s)))

; export-name greater

define-racket-builtin add
    (lambda (c s) (values c
                          (cons (+ (stack-top s number?)
                                   (stack-top (cdr s) number?))
                                (cddr s))))

export-name add

define-racket-builtin string->symbol
    (lambda (c s) (values c (cons (string->symbol (stack-top s string?)) (cdr s))))

export-name string->symbol

define-racket-builtin port-has-char?
    (lambda (c s) (values c (cons (char-ready? (stack-top s input-port?)) s)))
define-racket-builtin port-peek-char
    (lambda (c s) (values c (cons (peek-char (stack-top s input-port?)) s)))
define-racket-builtin port-read-char
    (lambda (c s) (values c (cons (read-char (stack-top s input-port?)) s)))
define-racket-builtin port-write-string
    (lambda (c s)
      (display (stack-top s string?) (stack-top (cdr s) output-port?))
      (values c (cdr s)))

define-racket-builtin port-write-value
    (lambda (c s)
      (write (stack-top s) (stack-top (cdr s) output-port?))
      (values c (cdr s)))

export-name port-has-char?
export-name port-peek-char
export-name port-read-char
export-name port-write-string
export-name port-write-value

define-racket-builtin definition-exists
    (lambda (c s)
      (values c (cons (hash-has-key? (context-definitions c)
                                     (stack-top s symbol?)) s)))

define-racket-builtin definition-get
    (lambda (c s)
      (values c (cons (hash-ref (context-definitions c)
                                (stack-top s symbol?) #f) s)))

define-racket-builtin definition-remove
    (lambda (c s)
      (let* ([name (stack-top s symbol?)]
             [defs (hash-remove (context-definitions c) name)])
        (values
          ; TODO context-copy or something?
          (make-context
            #:body (context-body c)
            #:definitions defs
            #:children (context-children c)
            #:parent (context-parent c))
          (cdr s))))

define-racket-builtin defined-names
  (lambda (c s)
    (let ([names
            (let loop : (Listof Symbol)
              ([c c] [acc : (Immutable-HashTable Symbol #t)
                          (make-immutable-hash)])
              (let ([a (for/fold
                         ([acc : (Immutable-HashTable Symbol #t) acc])
                         ([(k v) (in-hash (context-definitions c))])
                         (hash-set acc k #t))]
                    [p (context-parent c)])
                (if p (loop p a)
                    (hash-keys a))))])
      (values c (cons names s))))

export-name definition-exists
export-name definition-get
export-name definition-remove
export-name defined-names

define-racket-builtin current-context-remove-children
    (lambda (c s)
      (values
        ; TODO context-copy or something?
        (make-context
          #:body (context-body c)
          #:definitions (context-definitions c)
          #:parent (context-parent c))
        s))

export-name current-context-remove-children

define-racket-builtin hash-table?
    (lambda (c s) (values c (cons (hash? (stack-top s)) s)))
define-racket-builtin hash-table-empty
    (lambda (c s) (values c (cons (hash) s)))
define-racket-builtin hash-table-keys
    (lambda (c s) (values c (cons (hash-keys (stack-top s hash?)) s)))
define-racket-builtin hash-table-exists
    (lambda (c s)
      (let ([k (stack-top s)]
            [h (stack-top (cdr s) hash?)])
        (values c (cons (hash-has-key? h k) s))))
define-racket-builtin hash-table-get
    (lambda (c s)
      (let ([k (stack-top s)]
            [h (stack-top (cdr s) hash?)])
        (values c (cons (hash-ref h k #f) s))))
define-racket-builtin hash-table-set
    (lambda (c s)
      (let ([v (stack-top s)]
            [k (stack-top (cdr s))]
            [h (stack-top (cddr s) hash?)])
        (values c (cons (hash-set h k v) (cdddr s)))))
define-racket-builtin hash-table-remove
    (lambda (c s)
      (let ([k (stack-top s)]
            [h (stack-top (cdr s) hash?)])
        (values c (cons (hash-remove h k) (cddr s)))))

export-name hash-table?
export-name hash-table-empty
export-name hash-table-exists
export-name hash-table-get
export-name hash-table-set
export-name hash-table-keys
export-name hash-table-remove

; Places - A place is a mutable storage location
; capable of storing exactly one item.
; Multiple copies of a place all reference the same object.

; 'place' is already defined
(struct mplace ([v : Any]) #:mutable #:transparent) racket-eval
define-racket-builtin place?
    (lambda (c s) (values c (cons (mplace? (stack-top s)) s)))
define-racket-builtin make-place
    (lambda (c s) (values c (cons (mplace (stack-top s)) (cdr s))))
define-racket-builtin place-get
    (lambda (c s) (values c (cons (mplace-v (stack-top s mplace?)) s)))
define-racket-builtin place-set
    (lambda (c s)
      (let ([v (stack-top s)]
            [p (stack-top (cdr s) mplace?)])
        (set-mplace-v! p v)
        (values c (cdr s))))

export-name place?
export-name make-place
export-name place-get
export-name place-set

define-racket-builtin list-ref
    (lambda (c s)
      (let ([k (stack-top s exact-nonnegative-integer?)]
            [l (stack-top (cdr s) list?)])
        (values c (cons (list-ref l k) s))))
define-racket-builtin list-set
    (lambda (c s)
      (let ([val (stack-top s)]
            [k (stack-top (cdr s) exact-nonnegative-integer?)]
            [l (stack-top (cddr s) list?)])
        (values c (cons (list-set l k val) (cdddr s)))))

export-name list-ref
export-name list-set

define-racket-builtin list->vector
    (lambda (c s) (values c (cons (list->vector (stack-top s list?)) (cdr s))))
define-racket-builtin vector->list
    (lambda (c s) (values c (cons (vector->list (stack-top s vector?)) (cdr s))))
define-racket-builtin vector-length
    (lambda (c s) (values c (cons (vector-length (stack-top s vector?)) s)))
define-racket-builtin vector-ref
    (lambda (c s)
      (let ([k (stack-top s exact-nonnegative-integer?)]
            [v (stack-top (cdr s) vector?)])
        (values c (cons (vector-ref v k) s))))
define-racket-builtin vector-set!
    (lambda (c s)
      (let ([val (stack-top s)]
            [k (stack-top (cdr s) exact-nonnegative-integer?)]
            [v (stack-top (cddr s) vector?)])
        (vector-set! v k val)
        (values c (cddr s))))

export-name list->vector
export-name vector->list
export-name vector-length
export-name vector-ref
export-name vector-set!

; define-racket-builtin list-join
;     (lambda (c s) (values c (cons (apply append (stack-top s list?)) (cdr s))))
; export-name list-join

; vi: ft=scheme


