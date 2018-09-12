
;;; vi: ft=scheme

; Read literal char as @\<char>

parse-rule at [
    state-transition nothing at
    accept [ "@" string->char ]

    start
    type string
]

parse-rule at-backslash [
    state-transition at at-backslash
    accept [ "\\" string->char ]
]

parse-rule at-backslash-char [
    state-transition at-backslash nothing
    accept [ anything ]

    append
    finish
    prepend [ 'string->char ]
]

define parser-parentheses [
    local %end-char
    local %start-char
    "" %start-char string-push %end-char string-push local parens-tag
    parse-rule extra-paren-end [
        accept [ %end-char ]
        finish
        start
        parens-tag %tag
        type end-list
        append
        finish
        set-state nothing
    ]

    parse-rule extra-paren-start [
        accept [ %start-char ]
        finish
        start
        parens-tag %tag
        type start-list
        append
        finish
        set-state nothing
    ]

]

export-global parser-parentheses

; parse-rule start-list-braces [
; ]

;         ReaderArm::new("start-list")
;             .accept_input(Combo::Just('['.into()))
;             .run(ReaderInstruction::finish_token())
;             .run(ReaderInstruction::start_token())
;             .run(ReaderInstruction::set_tag("[]"))
;             .run(ReaderInstruction::set_type(TokenType::StartList))
;             .run(ReaderInstruction::append_token())
;             .run(ReaderInstruction::finish_token())
;             .run(ReaderInstruction::set_state("nothing"))
;         ,
;         ReaderArm::new("end-list")
;             .accept_input(Combo::Just(']'.into()))
;             .run(ReaderInstruction::finish_token())
;             .run(ReaderInstruction::start_token())
;             .run(ReaderInstruction::set_tag("[]"))
;             .run(ReaderInstruction::set_type(TokenType::EndList))
;             .run(ReaderInstruction::append_token())
;             .run(ReaderInstruction::finish_token())
;             .run(ReaderInstruction::set_state("nothing"))
;         ,

