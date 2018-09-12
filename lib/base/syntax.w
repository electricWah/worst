
define string->char [0 string-get swap drop]
export-global string->char

#! define ' quote !#
"quote" parser-new-rule
"'" string->char char-class-just combo-just parser-accept-input
quote nothing combo-just quote atom combo-just combo-either parser-accept-state
parser-finish-token
parser-start-token
quote symbol parser-set-token-type
parser-append-token
parser-finish-token
parser-save-rule

[quote quote uplevel] quote ' %define
export-global '

#! single-line comments !#
"semicolon-comment" parser-new-rule
'nothing combo-just 'atom combo-just combo-either parser-accept-state
";" string->char char-class-just combo-just parser-accept-input
parser-finish-token
'single-line-comment parser-set-state
parser-save-rule

"single-line-comment" parser-new-rule
'single-line-comment combo-just parser-accept-state
"
" string->char char-class-just combo-just combo-negate parser-accept-input
parser-save-rule

"end-single-line-comment" parser-new-rule
'single-line-comment combo-just parser-accept-state
"
" string->char char-class-just combo-just parser-accept-input
'nothing parser-set-state
parser-save-rule

;;; vi: ft=scheme

