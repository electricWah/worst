
; Bunch of stuff that isn't necessary in worst.rkt but is here
; I'll be honest, not sure whether to put as much as possible here
; or make bare rworst more useful

define-racket-builtin interpreter-dump-stack
    (lambda (c s) (eprintf "Stack:\n~S\n" s) (values c s))
define-racket-builtin interpreter-stack
    (lambda (c s) (values c (cons s s)))
define-racket-builtin interpreter-stack-set
    (lambda (c s) (values c (stack-top s list?)))
export interpreter-dump-stack
export interpreter-stack
export interpreter-stack-set

define-racket-builtin add
    (lambda (c s) (values c
                          (cons (+ (stack-top s number?)
                                   (stack-top (cdr s) number?))
                                (cddr s))))

export add

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

export port-has-char?
export port-peek-char
export port-read-char
export port-write-string
export port-write-value

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

export definition-exists
export definition-get
export definition-remove
export defined-names

define-racket-builtin current-context-remove-children
    (lambda (c s)
      (values
        ; TODO context-copy or something?
        (make-context
          #:body (context-body c)
          #:definitions (context-definitions c)
          #:parent (context-parent c))
        s))

export current-context-remove-children

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

export hash-table?
export hash-table-empty
export hash-table-exists
export hash-table-get
export hash-table-set
export hash-table-keys
export hash-table-remove

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

export place?
export make-place
export place-get
export place-set

; vi: ft=scheme


