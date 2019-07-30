
#lang hyper-literate typed/racket

@title[#:style '(toc)]{A Worst Interpreter} @;#| {{{ |#

You are reading an interpreter for
@hyperlink["http://worst.mitten.party"]{The Worst Programming Language}.

It's written in
@hyperlink["https://docs.racket-lang.org/ts-reference"]{Typed Racket},
but any language will work if you'd like to follow along.
As you read on, you will encounter documentation and source code for the
core procedures, built-in library functions, and command-line interface
of a working Worst interpreter, in roughly that order.
@hyperlink["https://gitlab.com/worst-lang/worst/blob/master/worst.rkt"]{A
single source file}
holds this text, the interpreter itself, and a handful of tests.

Reading this interpreter should hopefully give you a good understanding of how
it works, and currently serves as the best source of documentation for Worst.
If you'd rather just run it and have a poke around,
@hyperlink["https://gitlab.com/worst-lang/worst"]{check it out}.

This is a work in progress; in particular, it needs
more built-in functions and more tests. If there are any
necessary builtins missing, or you see any other problems, please feel free to
@hyperlink["https://gitlab.com/worst-lang/worst/issues"]{file an issue}!

@(table-of-contents)

@;#| }}} |#

@section[#:tag "introduction"]{Introduction} @;#| {{{ |#

I wanted Worst to be a language that was easy to implement,
yet flexible enough to grow beyond just being an experiment.
Following a rigorous process of repeatedly deleting everything
and starting over until new features stopped requiring complete redesigns,
I discovered the following combination of properties that worked well together:

@itemlist[
    @item{@bold{Stack-oriented}.
        With a stack instead of environments of local and global variables,
        keeping track of data becomes simple list manipulation.}
    @item{@bold{Concatenative}.
        Programs and functions compose just by being next to each other
        (yes, this is just Forth so far, but bear with me).
        This reduces the core to a basic loop: read a token, evaluate it, repeat.}
    @item{@bold{Homogeneous}.
        It's a lot simpler to manipulate functions when
        it's the same as manipulating lists.
        It also means that reading code is identical to reading data.}
    @item{@bold{@racket[quote] and @racket[uplevel]}.
        Together, these two are core to Worst's identity.
        @racket[quote] is the ability for any function to read
        the next token in the program (useful for making macros),
        and @racket[uplevel] (borrowed from Tcl) is
        the ability to execute code as if it were in the calling stack frame
        (implemented with ``reverse'' call stack).}
    @item{@bold{Lazy parsing}.
        It should be possible to modify the parser mid-program,
        so you can do things like importing syntax forms from a library.
        To this end, source code is parsed just before execution.}
]

Each of these properties work together to support the others.
Combined with a minimum of internal data structures
(just two: the call stack and the data stack), they lead to other properties
like dynamic scope, extensible error handling, and a REPL for free.

So, in brief, here's the main interpreter loop:
@itemlist[
    @item{@seclink["code-next"]{Get the next thing} from the program.
        If it's not a symbol, put it on top of the stack and repeat.}
    @item{If it is a symbol, @seclink["resolving-functions"]{look it up}
        in the definition set.
        (If it's not there, check the calling stack frames too.)}
    @item{If the definition is a normal function, call it
        and start again from the top.}
    @item{Otherwise, it's a list, so treat it as a sub-program.
        @seclink["calling-functions"]{Step into a new stack frame}
        and interpret it.}
    @item{@seclink["the-end-ha-ha"]{Repeat} until the program is empty.
        If there's a calling stack frame, carry on with that.}
]

That's it, ignoring @racket[uplevel],
and the @seclink["reading-code"]{read-eval loop}
that does the actual syntax parsing.
Most of the rest of the code is dedicated to defining built-in functions.

@;#| }}} |#

@section[#:tag "data-structures"]{Data structures} @;#| {{{ |#

The main elements of any program are code and data,
so that means two data structures should be enough.
Unsurprisingly, the data stack will be a regular @racket[list].
Everything related to the code
(function definitions, the call stack, the code itself)
can all go in one structure -- the @racket[context].

Together, the stack and the context contain
everything necessary to run the program.

@chunk[<context>
(struct context
  ; Program code
  ([body        : Code]
   ; Symbols looked up in here
   [definitions : (Immutable-HashTable Symbol Function)]
   ; Bookkeeping required by uplevel
   [children    : (Listof Context)]
   ; The calling context (if it's currently in the middle of a function call)
   [parent      : (Option Context)])
  #:transparent #:type-name Context)

; I never remember the right order for fields, so here's a keyword constructor.
(define (make-context
          #:body [body : Code '()]
          #:definitions [defs : (Immutable-HashTable Symbol Function)
                              (make-immutable-hash)]
          #:children [children : (Listof Context) '()]
          #:parent [parent : (Option Context) #f])
  (context body defs children parent))
]

I introduced a couple of types in there, hopefully with semi-obvious meanings.
Here they are along with some other supporting type definitions:

@chunk[<types>
; The data stack is just a list
(define-type Stack (Listof Any))
; A program is also just a list
(define-type Code (Listof Any))

; Builtin is a Racket procedure that updates a context and stack.
; It needs to be a procedural structure type in order to have a type predicate.
(define-struct/exec
  Builtin
  ([f : (Context Stack . -> . (Values Context Stack))])
  ((lambda (self ctx stack) ((Builtin-f self) ctx stack))
   : (Builtin Context Stack . -> . (Values Context Stack))))

; A Function is either code or a builtin.
; TODO this should be Definition to be consistent with the document.
(define-type Function (U Code Builtin))
(: function? (-> Any Boolean : Function))
(define (function? v) (or (list? v) (Builtin? v)))

; Can't use (Option A) because its None value is #f,
; which is a value we want to use in Worst.
; Using Void instead avoids any ambiguity.
; (Not a problem if the host language supports proper algebraic data types.)
(define-type (Maybe A) (U Void A))

; This will be useful for builtins that need a length or an index.
(define-predicate Nonnegative-Integer? Nonnegative-Integer)
]

@;#| }}} |#

@section[#:tag "core-operations"]{Core operations} @;#| {{{ |#

Now we've got a context, what can we actually do with it?

@subsection[#:tag "resolving-functions"]{Resolving functions}
Looking up a symbol to find its definition
needs to recursively walk up the parent contexts,
looking in the definitions table until it finds an entry.

@chunk[<context-resolve>
(: context-resolve (Context Symbol . -> . (Option Function)))
(define (context-resolve ctx name)
  (or (hash-ref (context-definitions ctx) name #f)
      (and (context-parent ctx)
        (context-resolve (context-parent ctx) name))))
]

@subsection[#:tag "calling-functions"]{Calling functions}
A function is either
a @racket[Builtin] (a regular function that modifies the context and stack)
or @racket[Code] (a list representing the function body).
Regular functions can just be called, but @racket[Code] requires a new context
(and the current context becomes its parent).
This uses some extra functions to @seclink["exceptions"]{deal with errors}.

@chunk[<interp-eval>
(: interp-eval (Context Stack Function
                        . -> . (Values Context Stack)))
(define (interp-eval ctx stack f)
  (cond
    [(Builtin? f) (f ctx stack)]
    [(list? f) (values (make-context #:body f #:parent ctx) stack)]))

; Resolve the symbol, set up an error handler blaming it,
; and eval its definition
(: interp-call (Context Stack Symbol
                        . -> . (Values Context Stack)))
(define (interp-call ctx stack sym)
  (let ([v (context-resolve ctx sym)])
    (if v
      (interp-try-eval ctx stack sym v)
      (interp-handle-error ctx stack 'undefined (list sym)))))
]

@subsection[#:tag "code-next"]{Figuring out what's next}

There's an easy way to do this, and a less easy way.
The easy way is to simply read code from the program.
This is what @racket[quote] will use,
since it usually maps directly to source code.

@chunk[<context-next-code>
(: context-next-code (Context . -> . (Values Context (Maybe Any))))
(define (context-next-code ctx)
  (if (null? (context-body ctx))
    (values ctx (void))
    (let ([code (car (context-body ctx))]
          [ctx (struct-copy context ctx [body (cdr (context-body ctx))])])
      (values ctx code))))
]

The less easy way is used by the interpreter itself,
which needs to take account of uplevels and parent contexts.
It needs to ``return'' to contexts in reverse order of entry,
which means it deals with uplevel children first (if there are any),
followed by the current context
(if there's any code left, using @racket[context-next-code]),
then finally it tries to return to the parent context.

@chunk[<context-next>
(: context-next (Context . -> . (Values Context (Maybe Any))))
(define (context-next ctx)
  (cond
    ; Find the innermost first child
    [(not (null? (context-children ctx)))
     (let* ([parent (struct-copy context ctx
                                 [children (cdr (context-children ctx))])]
            [child (struct-copy context (car (context-children ctx))
                                [parent parent])])
       (context-next child))]
    ; Current context next code
    [(not (null? (context-body ctx)))
     (context-next-code ctx)]
    ; Use the parent, discarding the current context as it's now useless
    [(context-parent ctx)
     (context-next (context-parent ctx))]
    ; There's nothing left. The program is finished.
    [else (values ctx (void))]))
]

@subsection{Traversing the context stack}

Here's @racket[uplevel] itself.
All this does is move up to the parent context
and push the current one on its list of children, like a reverse function call.

@chunk[<context-uplevel>
(: context-uplevel (Context . -> . (Option Context)))
(define (context-uplevel ctx)
  (let ([parent (context-parent ctx)])
    (and parent
         ; Unset parent because it'll be stale
         (let ([child (struct-copy context ctx [parent #f])])
           (struct-copy
             context parent
             [children (cons child (context-children parent))])))))
]

@; #| }}} |#

@section[#:tag "the-end-ha-ha"]{The End} @;#| {{{ |#

That's it! The core functionality is complete. Nice.
You can step through any program using the functions defined so far,
as long as you define all of the builtins it uses.

Okay, so this isn't @emph{really} the end. There's plenty more to do.
The rest of the interpreter will focus on turning this core into something
that can run a whole program from source code to completion.
For that, we'll need a main entry point that sets everything up,
some sort of loop to step through the program, and builtins. Lots of builtins.

The driving loop @racket[interp-run] can use @racket[context-next]
to figure out what to run next,
and either look it up as a function or push it to the stack as a literal:

@chunk[<interp-run>
(: interp-run (Context Stack . -> . (Values Context Stack)))
(define (interp-run ctx stack)
  (let-values ([(ctx v) (context-next ctx)])
    (cond
      ; Program ended
      [(void? v) (values ctx stack)]
      ; Call a symbol
      [(symbol? v)
       (let-values ([(ctx stack) (interp-call ctx stack v)])
         (interp-run ctx stack))]
      ; Push anything else to the stack
      [else (interp-run ctx (cons v stack))])))
]

@subsection{Tests}

Now we have enough code to run a program, we can try testing it.
Without any builtin functions yet, this might be a bit tough,
so let's take it one step at a time, starting from zero.

@chunk[<test-do-nothing>
(test-case "Empty context does nothing successfully"
  (let-values ([(ctx stack) (interp-run (make-context) '())])
    (check-equal? (context-body ctx) '())
    (check-equal? stack '())))
]

Good. Running an empty context with no code and an empty stack does nothing.

As everything except symbols is treated literally,
a program consisting of a sequence of non-symbols
should just result in a stack full of those things
(and a completed program body).

@chunk[<test-literals>
(test-case "Non-symbol literals go on the stack"
  (let-values ([(ctx stack)
                (interp-run
                  (make-context #:body '(1 2 (list #\A) "string" #t))
                  '())])
    (check-equal? ctx (make-context #:body '()))
    (check-equal? stack '(#t "string" (list #\A) 2 1))))
]

Since the stack is a list,
the top of the stack is visually at the left,
so the last thing in the program is the first thing in the stack.

While we still have no definitions to test,
let's make sure it fails if it encounters a symbol.

@chunk[<test-undefined>
(test-case
  "Throws on error"
  (check-exn
    exn:fail?
    (lambda ()
      (let-values ([(ctx stack)
                    (interp-run
                      (make-context #:body '(undefined reference))
                      '())])
        #t))))
]

TODO: There are always more tests to write.

@; #| }}} |#

@section[#:tag "builtins"]{Builtins} @;#| {{{ |#

All non-trivial programs will depend on these builtins.

@(local-table-of-contents)

@subsection{Quote} @;#| {{{ |#

@racket[context-next-code]
returns the lexically next item of code.
The builtin @racket[quote] can put this on the stack
without evaluating it.

@chunk[<builtin-quote>
(define-builtin
  (quote ctx stack)
  (let-values ([(ctx code) (context-next-code ctx)])
    (if (void? code)
      (interp-error 'quote-nothing)
      (values ctx (cons code stack)))))
]

We can test this:

@chunk[<test-quote>
(test-case "Quote"
  (let* ([defines (choose-global-builtins 'quote)]
         [ctx (make-context
                #:definitions defines
                ; this renders funny, it should be '(quote hello)
                #:body '(quote hello))])
    (let-values ([(ctx stack) (interp-run ctx '())])
      (check-equal? ctx (make-context #:definitions defines #:body '()))
      (check-equal? stack '(hello)))))
]

It should fail if there is nothing to quote:

@chunk[<test-quote-nothing>
(test-case "Quote nothing fails"
  (check-exn
    exn:fail?
    (lambda ()
      (let* ([defines (choose-global-builtins 'quote)]
             [ctx (make-context
                    #:definitions defines
                    #:body '(quote))])
        (let-values ([(ctx stack) (interp-run ctx '())])
          #t)))))
]
@; #| }}} |#
@subsection{Uplevel} @;#| {{{ |#

@racket[context-uplevel] moves into the parent context.
Normal execution would @seclink["code-next"]{undo this move immediately},
but the builtin @racket[uplevel] can
take a @racket[symbol] argument off the top of the stack
and use @racket[interp-eval] to sneak in a function to evaluate next.

@chunk[<builtin-uplevel>
(define-builtin
  (uplevel ctx stack)
  (let* ([ctx (context-uplevel ctx)])
    (if ctx
      (interp-call ctx (cdr stack) (stack-top stack symbol?))
      (interp-error 'root-uplevel))))
]

A test of @racket[quote] and @racket[uplevel] in combination:

@chunk[<test-uplevel-quote>
(test-case
  "Uplevel quote"
  (let* ([defines 
           (hash-set*
             (choose-global-builtins
               'quote 'uplevel)
             'inner-quote '(quote quote uplevel))]
         [ctx (make-context
                #:definitions defines
                #:body '(inner-quote "test"))])
    (let-values ([(ctx stack) (interp-run ctx '())])
      (check-equal? ctx (make-context #:definitions defines #:body '()))
      (check-equal? stack '("test")))))
]
@; #| }}} |#
@subsection{Intermission: Utilities} @;#| {{{ |#

I introduced a few functions there without explaining them.
It would be laborious to try and
keep track of all defined builtins in order to use them,
so let's keep a set of global builtins and use @racket[define-builtin]
to add to it. @racket[define-builtin] can use the context and stack,
or take some values off the top of the stack using @racket[stack-top].

@chunk[<global-builtins>
(: *builtins* (Parameterof (Immutable-HashTable Symbol Function)))
(define *builtins* (make-parameter (make-immutable-hash '())))

(: add-global-builtin (Symbol Function . -> . Void))
(define (add-global-builtin name builtin)
  (*builtins* (hash-set (*builtins*) name builtin)))

(define-syntax define-builtin
  (syntax-rules ()
    [(_ (name stack) body ...)
     (define-builtin (name ctx stack) (values ctx (begin body ...)))]
    ; TODO: I tried removing this repetition with a macro,
    ; but it was pretty tough. 3 arguments seems to be enough anyway.
    [(_ (name stack [v1 t1]) body ...)
     (define-builtin
       (name ctx stack)
       (let ([v1 (stack-top stack t1)])
         (values ctx (begin body ...))))]
    [(_ (name stack [v1 t1] [v2 t2]) body ...)
     (define-builtin
       (name ctx stack)
       (let ([v1 (stack-top stack t1)]
             [v2 (stack-top (cdr stack) t2)])
         (values ctx (begin body ...))))]
    [(_ (name stack [v1 t1] [v2 t2] [v3 t3]) body ...)
     (define-builtin
       (name ctx stack)
       (let ([v1 (stack-top stack t1)]
             [v2 (stack-top (cdr stack) t2)]
             [v3 (stack-top (cddr stack) t3)])
         (values ctx (begin body ...))))]
    [(_ (name ctx stack) body ...)
     (add-global-builtin
       'name
       (Builtin (lambda ([ctx : Context] [stack : Stack]) body ...)))]))

; Pick a subset of builtins, for tests
(: choose-global-builtins (->* () #:rest Symbol
                               (Immutable-HashTable Symbol Function)))
(define (choose-global-builtins . names)
  (make-immutable-hash
    (map (lambda ([n : Symbol]) (cons n (hash-ref (*builtins*) n))) names)))

]

@racket[stack-top] is a simple utility to make sure
the stack has a value on top, optionally with the right type.

@chunk[<stack-top>
(: stack-top (All (T) (case-> (Stack . -> . Any)
                              (Stack #t . -> . Any)
                              (Stack (-> Any Boolean : T) . -> . T))))
(define stack-top
  (case-lambda
    [(stack)
     (if (null? stack)
       (interp-error 'stack-empty)
       (car stack))]
    [(stack pred)
     (cond
       [(null? stack) (interp-error 'stack-empty pred)]
       [(eq? pred #t) (car stack)]
       [(not (pred (car stack)))
        (interp-error 'wrong-type pred)]
       [else (car stack)])]))

]

@; #| }}} |#
@subsection{Execution} @;#| {{{ |#

The builtins @racket[call] and @racket[eval] are just wrappers for
@racket[interp-call] and @racket[interp-eval] respectively.

@chunk[<builtin-call-eval>
(define-builtin
  (call ctx stack)
  (let ([v (stack-top stack symbol?)]
        [stack (cdr stack)])
    (interp-call ctx stack v)))

(define-builtin
  (eval ctx stack)
  (let ([v (stack-top stack function?)]
        [stack (cdr stack)])
    (interp-eval ctx stack v)))
]

@;#| }}} |#
@subsection{Conditional execution} @;#| {{{ |#

Many languages have at least one conditional, usually named @racket[if],
and often a handful more for specific situations.

Worst only needs one.
Every other conditional can be implemented in terms of
@racket[when], which conditionally performs a @racket[call]
depending on the value of a boolean on the stack.

@chunk[<builtin-when>
(define-builtin
  (when ctx stack)
  (let* ([name (stack-top stack symbol?)]
         [c (stack-top (cdr stack) boolean?)]
         [stack (cddr stack)])
    (if c
      ; TODO: this could use eval as well
      (interp-call ctx stack name)
      (values ctx stack))))
]

@;#| }}} |#
@subsection{Looping} @;#| {{{ |#

Constructs that require fairly complex syntax,
such as @racket[for] or @racket[while],
are a little bit chunky compared to everything else defined so far.

Luckily we have recursion, but following in the footsteps of Scheme
and automatically squashing tail calls could result in some confusion:
@racket[uplevel] would see a different parent context
depending on whether the current function call was in tail position or not.

So, here's a solution: a builtin that consolidates the current context
with the parent, keeping definitions intact.
Unlike other ``deliberate'' tail call systems,
this mechanism is used from within the tail call itself.
It can serve as a warning that @racket[uplevel] may require special care,
and can be introduced automatically by other looping constructs.

@chunk[<builtin-tail-call>
(define-builtin
  (tail-call ctx stack)
  (values
    ; Only merge if it's safe
    (if
      (and (context-parent ctx)
           (context-parent (context-parent ctx))
           (null? (context-body (context-parent ctx)))
           (null? (context-children (context-parent ctx))))
      (context-merge ctx (context-parent ctx))
      ctx)
    stack))
]

And @racket[context-merge]:

@chunk[<context-merge>
(: context-merge (Context Context . -> . Context))
(define (context-merge src dest)
  (make-context
    #:body (context-body src)
    #:children (context-children src)
    #:parent (context-parent dest)
    #:definitions
    (let ([src-defs (context-definitions src)]
          [dest-defs (context-definitions dest)])
      (if (hash-empty? dest-defs) src-defs
        (for/fold
          ([acc : (Immutable-HashTable Symbol Function) dest-defs])
          ([(k v) (in-hash src-defs)])
          (hash-set acc k v))))))
]

And a test. This test defines @racket[done] to return from the interpreter
loop early, so it can check the size of the call stack before it has a chance
to clean up after itself at the end.

@chunk[<test-tail-call>
(test-case
  "Tail calling loop"
  (let ([res
          (call/cc
            (lambda ([k : ((Option Context) . -> . Nothing)])
              (let* ([defines 
                       (hash-set*
                         (choose-global-builtins
                           'quote 'when 'not 'clone 'tail-call)
                         'done (Builtin
                                 (lambda ([ctx : Context] [_ : Stack])
                                   (k ctx)))
                         'test-tco '(tail-call
                                      clone not quote done when
                                      quote test-tco when))]
                     [ctx (make-context
                            #:definitions defines
                            #:body '(#f #t #t #t #t #t #t #t #t #t #t
                                     test-tco))])
                (let-values ([(ctx stack) (interp-run ctx '())])
                  (k #f)))))])
    (check-not-false res)
    (check-false (context-parent (assert res)))))
]
@;#| }}} |#

@;#| }}} |#

@section[#:tag "exceptions"]{Exceptions} @;#| {{{ |#

There are as many error handling strategies as programming languages,
ranging from ``drop everything and quit'' to ``don't have errors.''

Here's a simple one: if an error occurs, put it on the stack
and call the function @racket[current-error-handler].
If that's not defined, the interpreter has no choice but to quit altogether.

All the pieces are already in place:
@racket[interp-call], defined in @secref{core-operations},
is already set up to use @racket[interp-try-eval].

@chunk[<error-handling>

; Custom error type that can be put on the stack in its component parts
(struct builtin-failure exn:fail
  ([name : Symbol] [irritants : (Listof Any)])
  #:transparent
  #:type-name Builtin-Failure)

; Utility for builtins to signal an error
(: interp-error (All (A) ((Symbol) #:rest Any . ->* . A)))
(define (interp-error name . irritants)
  (raise (builtin-failure
           "Builtin failure"
           (current-continuation-marks)
           name irritants)))

(: interp-handle-error (Context Stack Symbol (Listof Any)
                                . -> . (Values Context Stack)))
(define (interp-handle-error ctx stack name irritants)
  (if (context-resolve ctx 'current-error-handler)
    ; Put the error on the stack
    (interp-call ctx
                 (list* name irritants stack)
                 'current-error-handler)
    ; Kill the interpreter if current-error-handler isn't defined
    (apply error "Unhandled error" name irritants)))

(: interp-try-eval (Context Stack Symbol Function
                            . -> . (Values Context Stack)))
(define (interp-try-eval ctx stack name f)
  (with-handlers
    ; Builtin failures get the arguments given to interp-error
    ([builtin-failure?
       (lambda ([e : Builtin-Failure])
         (interp-handle-error ctx stack
                              (builtin-failure-name e)
                              (cons name (builtin-failure-irritants e))))]
     [exn:fail? ; should this catch all exceptions, including breaks?
       (lambda ([e : exn])
         ; Try to deal with this error in a reasonable way
         (interp-handle-error ctx stack
                              name
                              (list (exn-message e))))])
    (interp-eval ctx stack f)))

; TODO tests for this.

]

@;#| }}} |#

@section[#:tag "more-builtins"]{More builtins} @;#| {{{ |#

This (incomplete) library of builtins
unashamedly takes inspiration from the equivalent concepts in Racket and Rust.
There are a lot here --
this section takes up a third of the entire source file --
but not all of them need to be in every Worst implementation,
nor must they have the same syntax or semantics.
For instance, at the moment, the builtins listed here are quite inconsistent
with whether they consume their input arguments or not.

So, if you're writing your own Worst, feel free to just
use these builtins for inspiration. You may like to skim this section.

@chunk[<builtins>
<builtin-predicates>
<builtin-datum-ops>
<builtin-interpreter-ops>
<builtin-context-ops>
<builtin-utility-ops>
<builtin-definition-ops>
<builtin-list-ops>
<builtin-string-ops>
<builtin-bool-ops>
<builtin-numeric-ops>
<builtin-env-ops>
<builtin-port-ops>
<builtin-subprocess-ops>
<builtin-place-ops>
<builtin-filesystem-ops>
<builtin-hash-table-ops>
]

@subsection{Type predicates} @;#| {{{ |#

Predicates for some of the basic data types.

@chunk[<builtin-predicates>
(define-builtin (char?          s [a #t]) (cons (char? a) s))
(define-builtin (string?        s [a #t]) (cons (string? a) s))
(define-builtin (symbol?        s [a #t]) (cons (symbol? a) s))
(define-builtin (list?          s [a #t]) (cons (list? a) s))
(define-builtin (bool?          s [a #t]) (cons (boolean? a) s))
(define-builtin (number?        s [a #t]) (cons (number? a) s))
(define-builtin (integer?       s [a #t]) (cons (integer? a) s))
(define-builtin (exact?         s [a #t]) (cons (exact? a) s))
(define-builtin (rational?      s [a #t]) (cons (rational? a) s))
(define-builtin (float?         s [a #t]) (cons (double-flonum? a) s))
(define-builtin (eof-object?    s [a #t]) (cons (eof-object? a) s))
]

@;#| }}} |#
@subsection{General operations} @;#| {{{ |#

@chunk[<builtin-datum-ops>
(define-builtin (clone s [a #t]) (cons a s))
(define-builtin (drop s [a #t]) (cdr s))
(define-builtin (swap s [a #t] [b #t]) (list* b a (cddr s)))
(define-builtin (rot s [a #t] [b #t] [c #t]) (list* b c a (cdddr s)))

(define-builtin (equal? s [a #t] [b #t]) (cons (equal? a b) s))
(define-builtin (identical? s [a #t] [b #t]) (cons (eq? a b) s))
]


@;#| }}} |#
@subsection{Interpreter} @;#| {{{ |#

@chunk[<builtin-interpreter-ops>

(define-builtin (interpreter-stack s) (cons s s))

(define-builtin
  (interpreter-exit ctx stack)
  (exit (stack-top stack byte?)))
]

@;#| }}} |#
@subsection{Context} @;#| {{{ |#

@chunk[<builtin-context-ops>

(define-builtin
  (current-context-root? ctx stack)
  (values ctx (cons (not (context-parent ctx)) stack)))

(define-builtin
  (current-context-clear ctx stack)
  (values (make-context #:parent (context-parent ctx)) stack))

(define-builtin
  (current-context-has-definition? ctx stack)
  (values ctx
          (cons (hash-has-key? (context-definitions ctx)
                               (stack-top stack symbol?))
                stack)))

(define-builtin
  (current-context-definitions ctx stack)
  (values ctx
          (cons (context-definitions ctx)
                stack)))

(define-builtin
  (current-context-set-code ctx stack)
  (let ([v (stack-top stack list?)])
    (values (struct-copy context ctx [body v])
            (cdr stack))))

(define-builtin
  (current-context-get-code ctx stack)
  (let ([v (stack-top stack list?)])
    (values (struct-copy context ctx [body v])
            (cdr stack))))

; getter for current context itself?
]

@;#| }}} |#
@subsection{Debugging utilities} @;#| {{{ |#

@chunk[<builtin-utility-ops>
(define-builtin
  (interpreter-dump-stack ctx stack)
  (eprintf "Stack:\n~S\n" stack)
  (values ctx stack))

(require racket/pretty)

(define-builtin
  (interpreter-dump-context ctx stack)
  (pretty-print ctx)
  (values ctx stack))
]
@;#| }}} |#
@subsection{Definitions and builtins} @;#| {{{ |#

@chunk[<builtin-definition-ops>

(define-builtin
  (builtin-get ctx stack)
  (values ctx
          (cons (hash-ref (*builtins*) (stack-top stack symbol?) #f)
                (cdr stack))))

(define-builtin
  (definition-get ctx stack)
  (values ctx
          (cons (hash-ref 
                  (context-definitions ctx)
                  (stack-top stack symbol?) #f)
                stack)))

(define-builtin
  (definition-resolve ctx stack)
  (let ([name (stack-top stack symbol?)])
    (values ctx (cons (context-resolve ctx name) stack))))

(define-builtin
  (definition-add ctx stack)
  (let* ([name (stack-top stack symbol?)]
         [def (stack-top (cdr stack) function?)]
         [defs (hash-set (context-definitions ctx) name def)])
    (values
      (struct-copy context ctx [definitions defs])
      (cddr stack))))

;    definition-exists?
;    defined-names
;    all-builtins
]

@;#| }}} |#
@subsection{Lists} @;#| {{{ |#

@chunk[<builtin-list-ops>

(define-builtin (list-empty? s [a list?]) (cons (empty? a) s))
(define-builtin (list-length s [a list?]) (cons (length a) s))
(define-builtin (list-reverse s [a list?]) (cons (reverse a) (cdr s)))
(define-builtin (list-append s [b list?] [a list?]) (cons (append a b) (cddr s)))
(define-builtin (list-push s [v #t] [a list?]) (cons (cons v a) (cddr s)))
(define-builtin (list-pop s [a list?]) (list* (car a) (cdr a) (cdr s)))

(define-builtin (list-split s [v Nonnegative-Integer?] [l list?])
                (let-values ([(a b) (split-at l v)])
                  (list* a b s)))

(define-builtin (list-get s [n Nonnegative-Integer?] [a list?])
                (cons (list-ref a n) s))
(define-builtin (list-set s [v #t] [n Nonnegative-Integer?] [a list?])
                (cons (list-set a n v) (cdddr s)))
]

@;#| }}} |#
@subsection{Strings and symbols} @;#| {{{ |#

@chunk[<builtin-string-ops>

(define-builtin
  (string-append s [a string?] [b string?])
  (cons (string-append b a)
        (cddr s)))

(define-builtin (string-length s [a string?]) (cons (string-length a) s))

(define-builtin (->string s [a #t]) (cons (~a a) (cdr s)))

; TODO
; string-char-boundary? string-get string->list string-pop string-push
; string->symbol symbol? symbol->string
]

@;#| }}} |#
@subsection{Booleans} @;#| {{{ |#
@chunk[<builtin-bool-ops>
(define-builtin (and s [a #t] [b #t]) (cons (and a b) s))
(define-builtin (or s [a #t] [b #t]) (cons (or a b) s))
(define-builtin (false? s [a #t]) (cons (false? a) s))
(define-builtin (not s [a #t]) (cons (false? a) (cdr s)))
]

@;#| }}} |#
@subsection{Numbers} @;#| {{{ |#

@chunk[<builtin-numeric-ops>
(define-builtin (add s [b number?] [a number?]) (cons (a . + . b) (cddr s)))
(define-builtin (sub s [b number?] [a number?]) (cons (a . - . b) (cddr s)))
(define-builtin (mul s [b number?] [a number?]) (cons (a . * . b) (cddr s)))
(define-builtin (div s [b number?] [a number?]) (cons (a . / . b) (cddr s)))
(define-builtin (abs s [a real?]) (cons (abs a) (cdr s)))
(define-builtin (negate s [a number?]) (cons (- a) (cdr s)))

; TODO comparison: gt?, lt?, gte?, lte?
]

@;#| }}} |#
@subsection{Environment} @;#| {{{ |#

@chunk[<builtin-env-ops>

(define-builtin
  (command-line ctx stack)
  (values ctx (cons (vector->list (current-command-line-arguments)) stack)))

(define-builtin
  (env-get ctx stack)
  (values ctx
          (cons
            (getenv (stack-top stack string?))
            stack)))

; env-list : -> listof string (environment variable names)

(define-builtin
  (env-set ctx stack)
  (let* ([v (stack-top stack string?)]
         [k (stack-top (cdr stack) string?)]
         [stack (cddr stack)])
    (putenv k v)
    (values ctx stack)))

(define-builtin
  (system-resolve-path s [a string?])
  (cons (find-executable-path a) (cdr s)))

]

@;#| }}} |#
@subsection{Ports} @;#| {{{ |#

@chunk[<builtin-port-ops>

(define-builtin (port? s [a #t]) (cons (port? a) s))
(define-builtin (input-port? s [a #t]) (cons (input-port? a) s))
(define-builtin (output-port? s [a #t]) (cons (output-port? a) s))

(define-builtin (current-input-port s) (cons (current-input-port) s))
(define-builtin (current-output-port s) (cons (current-output-port) s))
(define-builtin (current-error-port s) (cons (current-error-port) s))

(define-builtin (port-read-value s [a input-port?]) (cons (read a) s))
(define-builtin (port-read-char s [a input-port?]) (cons (read-char a) s))
(define-builtin (port-peek-char s [a input-port?]) (cons (peek-char a) s))
(define-builtin (port-has-char? s [a input-port?]) (cons (char-ready? a) s))

(define-builtin (port-read-line s [a input-port?]) (cons (read-line a) s))

(define-builtin (port-write-value s [v #t] [p output-port?])
                (write v p)
                (cdr s))
(define-builtin (port-write-string s [v string?] [p output-port?])
                (display v p)
                (cdr s))

(define-builtin (port-copy-all s [o output-port?] [i input-port?])
                (copy-port i o)
                s)

(define-builtin (make-pipe-ports s)
                (let-values ([(i o) (make-pipe)])
                  (list* i o s)))

; TODO
; port-read
; port-seekable?
; port-seek/end
; port-seek/relative
; port-seek/start
; port-unique?
; port-write
; output-port-flush

]

@;#| }}} |#
@subsection{Places} @;#| {{{ |#

A @emph{place} is a mutable storage location
capable of storing exactly one item.
Multiple copies of a place all reference the same object.

@chunk[<builtin-place-ops>

; the name 'place' is taken
(struct mplace ([v : Any])
  #:mutable
  #:transparent)

(define-builtin (place? s [a #t]) (cons (mplace? a) s))
(define-builtin (make-place s [a #t]) (cons (mplace a) (cdr s)))

(define-builtin (place-get s [p mplace?]) (cons (mplace-v p) s))

(define-builtin (place-swap s [v #t] [p mplace?])
                (let ([o (mplace-v p)])
                  (set-mplace-v! p v)
                  (cons o (cdr s))))

(define-builtin (place-set s [v #t] [p mplace?])
                (set-mplace-v! p v)
                (cdr s))

]
@;#| }}} |#
@subsection{Subprocesses} @;#| {{{ |#

Subprocess invocation happens in two parts:
first, configure the @emph{command} to run,
including the program, its arguments,
environment variables, and input, output, and error pipes.
Then spawn the command as a @emph{process} to run it.

TODO replace command with 
https://docs.racket-lang.org/reference/subprocess.html

@chunk[<builtin-subprocess-ops>

; untested

(struct command
  ([program : Path-String]
   [arguments : (Listof (U Bytes Path-String))]
   ; TODO cd?
   [stdin-port : (Option Input-Port)]
   [stdout-port : (Option Output-Port)]
   [stderr-port : (Option Output-Port)]
   [environment : (HashTable String String)])
  #:type-name Command
  #:transparent)

(define-builtin (command? s [a #t]) (cons (command? a) s))

(define-builtin (make-command s [program (make-predicate (U Path String))])
                (cons (command program '() #f #f #f (hash)) (cdr s)))

(define-builtin
  (command-set-arguments s
                         [args (make-predicate (Listof (U Bytes Path-String)))]
                         [cmd command?])
  (cons (struct-copy command cmd [arguments args]) (cddr s)))

(define-builtin
  (command-set-env s [env hash?] [cmd command?])
  (let ([env (cast env (HashTable String String))])
    (cons (struct-copy command cmd [environment env]) (cddr s))))

(define-builtin
  (command-set-stdin s [p input-port?] [cmd command?])
  (cons (struct-copy command cmd [stdin-port p]) (cddr s)))

(define-builtin
  (command-set-stdout s [p output-port?] [cmd command?])
  (cons (struct-copy command cmd [stdout-port p]) (cddr s)))

(define-builtin
  (command-set-stderr s [p output-port?] [cmd command?])
  (cons (struct-copy command cmd [stderr-port p]) (cddr s)))

(struct
  ; process is taken
  proc
  ([subprocess : Subprocess]
   [stdin : (Option Output-Port)]
   [stdout : (Option Input-Port)]
   [stderr : (Option Input-Port)])
  #:type-name Process)

(define-builtin (process? s [a #t]) (cons (proc? a) s))

(define-builtin (process-stdin-port s [p proc?])
                (cons (proc-stdin p) s))
(define-builtin (process-stdout-port s [p proc?])
                (cons (proc-stdout p) s))
(define-builtin (process-stdout-port s [p proc?])
                (cons (proc-stdout p) s))

(define-builtin (process-pid s [p proc?])
                (cons (subprocess-pid (proc-subprocess p)) s))

(define-builtin (process-running? s [p proc?])
                (cons (eq? (subprocess-status (proc-subprocess p))
                           'running) s))

(define-builtin (process-kill s [p proc?])
                (subprocess-kill (proc-subprocess p) #t)
                (cdr s))

(define-builtin (process-wait s [p proc?])
                (subprocess-wait (proc-subprocess p))
                (cons (subprocess-status (proc-subprocess p)) s))

(define-builtin
  (command-spawn s [cmd command?])
  (let-values ([(subp stdout stdin stderr)
                (apply subprocess
                       (command-stdout-port cmd)
                       (command-stdin-port cmd)
                       (command-stderr-port cmd)
                       (command-program cmd)
                       (command-arguments cmd))])
    (cons
      (proc subp stdin stdout stderr)
      (cdr s))))

]

@;#| }}} |#
@subsection{Filesystem} @;#| {{{ |#

@chunk[<builtin-filesystem-ops>

; name tbc
(define-builtin (open-input-file s [f string?])
                (cons (open-input-file f) (cdr s)))

]

TODO. Doesn't have to be as complicated as the Rust version.
file?
file-info
file-info?
file-info-directory?
file-info-file?
file-info-length
file-info-readonly?
file-info-symlink?
file-port
file-sync
make-open-file-options
open-file
open-file-append
open-file-create
open-file-create-new
open-file-read
open-file-truncate
open-file-write

@;#| }}} |#
@subsection{Hash tables} @;#| {{{ |#

Immutable dictionaries based on Racket's @racket[hashtable].
If you want a mutable hash-table, put it in a @racket[place].

@chunk[<builtin-hash-table-ops>

(define-builtin (hash-table? s [h #t]) (cons (hash? h) s))
(define-builtin (hash-table-empty s) (cons (hash) s))
(define-builtin (hash-table-keys s [h hash?]) (cons (hash-keys h) s))

(define-builtin
  (hash-table-first-key s [h hash?])
  (let* ([it (hash-iterate-first h)]
         [key (and it (hash-iterate-key h it))])
    (cons key s)))

(define-builtin (hash-table-exists s [k #t] [h hash?])
                (cons (hash-has-key? h k) s))

(define-builtin (hash-table-get s [k #t] [h hash?])
                (cons (hash-ref h k #f) s))

(define-builtin (hash-table-set s [v #t] [k #t] [h hash?])
                (let ([h (cast h (HashTable Any Any))])
                  (cons (hash-set h k v) (cdddr s))))

(define-builtin (hash-table-take s [k #t] [h hash?])
                (let ([v (hash-ref h k #f)]
                      [tbl (hash-remove h k)]
                      [s (cddr s)])
                  (list* v k tbl s)))

]

@;#| }}} |#
@subsection{Vectors} @;#| {{{ |#

TODO? Vectors are like lists, but mutable/double-ended/constant-sized?

@;#| }}} |#
@subsection{Byte vectors} @;#| {{{ |#

Bytes is a vector of... bytes.
TODO rename from u8vector.

string->u8vector
make-u8vector ; or bytes-empty
u8vector?
u8vector-append
u8vector-extend
u8vector-get
u8vector-invalid-char-index
u8vector-length
u8vector-push
u8vector-set
u8vector->string
u8vector-truncate

@;#| }}} |#

@;#| }}} |#

@section[#:tag "reading-code"]{Reading code} @;#| {{{ |#

So far, all test programs have been fed pre-parsed code in a list.
To read code (or data, it's all the same) from a file,
we can use @racket[port-read-value] (defined in @secref{Ports}).
However, the interpreter doesn't know how to do that,
so we need a way of using it when the program expects to read more code.

So here it is. This read-eval loop checks @racket[quote-read-syntax?]
to read code from @racket[source-input-port],
and basically reimplements the core interpreter loop.
If you wanted, you could redefine
@racket[source-input-port] to read from a different file
or @racket[syntax-read] to change the character-level syntax.

@chunk[<read-eval-loop>

; Add definitions to support a read-eval-loop from the given input port.
(: read-eval-loop-definitions
   (-> (Immutable-HashTable Symbol Function) Input-Port
       (Immutable-HashTable Symbol Function)))
(define (read-eval-loop-definitions builtins source-input-port)
  (hash-set*
    builtins
    'source-input-port `(,source-input-port)
    'syntax-read '(; read a value from the current source file
                   source-input-port port-read-value swap drop)
    'quote-read-syntax? '(; This could be consolidated
                          builtin-quote current-context-root? uplevel)
    ; Override quote to read from the source input file
    ; at the toplevel
    'builtin-quote (hash-ref (*builtins*) 'quote)
    'quote '(builtin-quote builtin-quote
                           builtin-quote syntax-read
                           builtin-quote quote-read-syntax? uplevel
                           builtin-quote swap when drop uplevel)
    'read-eval-loop '(; Loop
                      tail-call
                      ; Read next value
                      quote quote uplevel
                      ; Leave <eof> here (this is a little wonky)
                      eof-object? quote current-context-clear when
                      ; Eval symbols; leave everything else on stack
                      symbol?
                      ; Stack dance to get everything in place
                      quote call swap
                      quote drop swap
                      quote uplevel swap
                      ; 'eval 'drop 'uplevel #t 'swap when drop
                      ; => 'eval 'uplevel
                      ; 'eval 'drop 'uplevel #f 'swap when drop
                      ; => 'eval 'drop
                      quote swap when drop
                      call
                      ; Loop!
                      read-eval-loop)))
]

@;#| }}} |#

@section[#:tag "the-entry-point"]{The entry point} @;#| {{{ |#

The main program has the task of gluing everything so far together.
It picks a source input (either the first file argument or stdin)
and runs the basic read-eval loop from the previous section.

@chunk[<entry-point>
(module+ main
  (let ([source-input-port
          (if (> (vector-length (current-command-line-arguments)) 0)
            (open-input-file (vector-ref (current-command-line-arguments) 0))
            (current-input-port))])
    (let-values
      ([(ctx stack)
        (interp-run
          (make-context
            #:definitions
            (read-eval-loop-definitions (*builtins*) source-input-port)
            ; Run the loop on its own
            #:body '(read-eval-loop))
          ; Start with an empty stack
          '())])
      (void))))
]

@;#| }}} |#

@section[#:tag "invocation"]{Invocation} @;#| {{{ |#

This file is executable: @verbatim{racket worst.rkt}
You can compile it into a binary if you want:
@verbatim{raco exe -o rworst worst.rkt}

You can also write executable scripts:
@verbatim{#!/path/to/rworst}
This works because the default reader treats a shebang line as a comment.

@hyperlink["https://gitlab.com/worst-lang/worst"]{file an issue}!

@;#| }}} |#

@section[#:tag "program-overview"]{Program overview} @;#| {{{ |#

@chunk[<*>

<types>
<context>

<context-resolve>
<context-uplevel>
<context-next-code>
<context-next>
<context-merge>

<error-handling>

<interp-eval>
<interp-run>

<stack-top>
<global-builtins>

<builtin-quote>
<builtin-uplevel>
<builtin-call-eval>
<builtin-when>
<builtin-tail-call>

<builtins>

(module+ test

  (require typed/rackunit)

  <test-do-nothing>
  <test-literals>
  <test-undefined>
  <test-quote>
  <test-quote-nothing>
  <test-uplevel-quote>
  <test-tail-call>

  (void))

<read-eval-loop>

<entry-point>

]@;#| }}} |#

@section[#:tag "license"]{License} @;#| {{{ |#

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

@;#| }}} |#

@; vim:commentstring=@\;#|\ %s\ |#

