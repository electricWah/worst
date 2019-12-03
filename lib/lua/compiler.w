
;  [[test-definition egg [test-if [7 test-add] [6 test-add]]] compile-block] lua-compile

; expressions are as usual
; statements are emitted:
; expr expr->var name -> emit "var = expr;"; name
; ((expr body) ... (body)) conds
; body expr while-loop
; body list for-iter

define lua-compile [
    import syntax/variable
    import worst/interpreter
    import syntax/object
    import syntax/assign
    import list

    ; the interpreter here keeps a shadow stack for optimisations

    quote var% gensym const %var
    define var? [
        list? if [
            list-length equals? 2 if [
                drop list-head %var equal?
                bury drop drop
            ] [ drop #f ]
        ] [ #f ]
    ]
    define var-new [ [] quote v gensym list-push %var list-push ]

    quote expr% gensym const %expr
    define expr? [
        list? if [
            list-length equals? 2 if [
                drop list-head %expr equal?
                bury drop drop
            ] [ drop #f ]
        ] [ #f ]
    ]
    define make-expr [ [] swap list-push %expr list-push ]

    define literal? [
        var? if [ #f ] [ expr? if [ #f ] [ #t ] ]
    ]


    make-interpreter interp

    define interp-builtin [
        upquote const name
        upquote const body
        body name interp def
    ]
    define interp-builtin-def [
        upquote const name
        upquote const body

        [interp enter] body list-push
        name interp def
    ]


    define-object-constructor interp-place [
        init [ "%%" %name ->string string-append string->symbol ]
        method new [ make-place %get interp def ]
        method exists [ %get interp resolve if [ drop #t ] [ #f ] ]
        method get [ %get interp resolve drop place-get swap drop ]
        method set [ %get interp resolve drop swap place-set drop ]
    ]

    ; Interpreter stack
    interp-place current-stack

    ; Compiler bits for when compiling a code block
    ; if stack-empty then generate an input variable and put it here
    interp-place current-inputs
    ; emit-statement puts code in here
    interp-place current-statements

    define new-compiler-frame [
        [] current-stack new
        [] current-inputs new
        [] current-statements new
    ]

    define stack-input-generate [
        current-inputs get
        swap list-push
        current-inputs set
    ]

    define inputs-get [ current-inputs get ]
    define statements-get [ current-statements get ]

    [] variable %statements-emit

    define emit-statement [
        current-statements exists if [
            current-statements get
            swap list-iterate [list-push]
            current-statements set
        ] [
            %statements-emit get
            swap list-iterate [list-push]
            %statements-emit set
        ]
    ]

    [] current-stack new

    define stack-get [ current-stack get ]
    define stack-set [ current-stack set ]
    define stack-push [ stack-get swap list-push stack-set ]
    define stack-take [
        stack-get
        list-empty? if [
            drop
            current-inputs exists if [
                ; compiling, request an input
                var-new clone stack-input-generate
            ] [
                ; not compiling, stack is empty
                quote stack-empty abort
            ]
        ] [
            list-pop swap stack-set
        ]
        ; non-input-taking version
        ; stack-get
        ; list-empty? if [ drop quote stack-empty abort ] []
        ; list-pop
        ; swap
        ; stack-set
    ]
    define literal! [
        upquote const pred
        pred call if [ ] [
            pred quote wrong-type abort
        ]
    ]

    define interp-push-literal [
        symbol? const sym
        clone stack-push
        [] swap list-push
        sym if [ quote quote list-push ] [ ]
        emit-statement
    ]

    ; TODO extern
    interp-builtin print [
        stack-take drop
        [[] swap list-push quote print funcall ignore]
        emit-statement
    ]

    interp-builtin add [
        ; TODO literal adds
        []
        stack-take list-push
        stack-take list-push
        make-expr
        stack-push

        [binop add] emit-statement
    ]

    interp-builtin quote [
        interp read
        if [] [ quote quote-nothing abort ]
        interp-push-literal
    ]

    quote uplevel-barrier% gensym const uplevel-barrier
    interp-builtin uplevel [
        interp parent if [ ] [ quote root-uplevel abort ]
        uplevel-barrier interp defined?
        if [ drop quote uplevel-forbidden abort ] []
        stack-take literal! symbol?
        [drop] emit-statement
        interp resolve if [ ] [ quote undefined abort ]
        eval
    ]

    interp-builtin swap [
        stack-take
        stack-take
        swap
        stack-push
        stack-push
        [swap] emit-statement
    ]

    interp-builtin drop [
        stack-take drop
        [drop "drop" drop] emit-statement
    ]

    ; TODO attributes for e.g. purity, extern-ness,
    ; renaming (cool-fun -> cool_fun)
    ; or just definition name [args [args...] body [body...] sig [...] ...]
    interp-builtin compile-block [
        [compile-block-complete] interp enter
        #t uplevel-barrier interp def
        interp-builtin compile-block-complete [
            ; easier to manage a single list result here
            [] ; list these in reverse order
            statements-get list-reverse list-push
            stack-get list-push
            inputs-get list-push
            ; usage: this list-iterate [] => [inputs outputs body]

            ; get values from the current context
            ; discard the current context
            ; push the values to the (parent) context stack
            interp parent drop

            stack-push
        ]
        stack-take literal! list? const body
        [drop] emit-statement

        new-compiler-frame
        body interp enter
    ]

    interp-builtin definition-add/compiled [
        stack-take literal! list?
        ; see compile-block compile-block-complete
        list-iterate [] => [inputs outputs defbody]

        stack-take literal! symbol? const name
        [drop] emit-statement

        list-quasiquote [
            ~[list-quasiquote [
                *[defbody]
                ~[outputs list-length swap drop]
                ^[return/n]
            ]]
            ~[inputs list-length swap drop]
            ^[quote] ~[name]
            ^[definition]
        ]
        emit-statement
        ; and then interp def with a call-this-definition ...
    ]
    interp-builtin-def definition-add [
        swap
        compile-block
        definition-add/compiled
    ]

    ; TODO add if/else here

    interp-builtin compile-if-else [
        stack-take literal! list?
        list-iterate [] => [finputs foutputs fbody]
        stack-take literal! list?
        list-iterate [] => [tinputs toutputs tbody]

        ; TODO if this is an actual bool, just emit the corresponding arm
        stack-take const c
        [const %ifcond] emit-statement

        tinputs list-length const tinlen drop
        toutputs list-length const toutlen drop
        finputs list-length const finlen drop
        foutputs list-length const foutlen drop

        ; ensure tin - tout = fin - fout so the stack is always the same
        tinlen toutlen neg add
        finlen foutlen neg add
        equal? if [drop drop] [ "if: arms have different arity" abort ]
        
        tinlen finlen max const inmax
        toutlen foutlen max const outmax

        ; Prepare output variables
        [[]] emit-statement
        outmax do-times [
            [variable/uninit list-push] emit-statement
        ]
        [const %ifouts] emit-statement

        ; Duplicate input variables for true arm
        [[]] emit-statement
        tinlen do-times [
            [swap list-push] emit-statement
        ]
        [const %ifins] emit-statement

        ; Prepare true arm

        ; True arm outputs
        [%ifins list-iterate []]
        tbody
        list-append
        [%ifouts list-iterate [assign drop]]
        list-append
        const tbody
        
        ; False arm outputs
        [%ifins list-iterate []]
        fbody
        list-append
        [%ifouts list-iterate [assign drop]]
        list-append
        const fbody


        ; Now the if-else itself
        [%ifcond] emit-statement
        []
        fbody list-push
        tbody list-push
        emit-statement
        [if-else] emit-statement

        ; Finally, put output variables on the stack
        [%ifouts list-iterate []] emit-statement

        ; Set the interpreter stack to what it should be:
        ; drop as many as the if would use,
        ; put on as many new variables as the if would
        inmax do-times [stack-take drop]
        outmax do-times [var-new stack-push]
    ]

    interp-builtin-def if [
        quote quote uplevel compile-block
        quote quote uplevel compile-block
        compile-if-else
    ]

    interp-builtin compile-while [
        stack-take literal! list?
        list-iterate [] => [bodyins bodyouts bodybody]
        stack-take literal! list?
        list-iterate [] => [condins condouts condbody]

        

        condins list-length const condinlen drop
        bodyins list-length const bodyinlen drop

        condinlen 1 add
        condouts list-length swap drop
        equal? if [drop drop] [
            "while: cond arity must be (in + 1 = out)" abort
        ]

        bodyinlen
        bodyouts list-length swap drop
        equal? if [drop drop] [
            "while: body arity must be (in = out)" abort
        ]

        bodyinlen condinlen max const varlen

        [[]] emit-statement
        varlen do-times [[swap variable swap list-append] emit-statement]
        [const %whilevars] emit-statement
        [%whilevars list-iterate []] emit-statement

        [#t] emit-statement

        condbody
        [[] swap list-push quote not funcall] list-append
        [[break] #f if-else] list-append
        bodybody list-append
        [%whilevars list-iterate [assign drop]] list-append
        [] swap list-push emit-statement

        [while-do] emit-statement

        [%whilevars list-iterate []] emit-statement

        varlen do-times [make-variable]

        ; while (true) do
        ;  condbody
        ;  if (condtop) then break end
        ;  bodybody
        ; end

    ]

    interp-builtin-def while [
        quote quote uplevel compile-block
        quote quote uplevel compile-block
        compile-while
    ]

    upquote interp enter

    while [
        ; TODO somehow this doesn't walk up all parents at the end
        interp next
        ; interpreter-dump-stack
        if [
            symbol? if [
                ; clone ->string print "\n" print
                interp resolve if [
                    eval
                ] [
                    quote undefined abort
                ]
            ] [
                interp-push-literal
            ]
            #t
        ] [ #f ]
    ] []

    %statements-emit get list-reverse
]
export-name lua-compile

; vi: ft=scheme


