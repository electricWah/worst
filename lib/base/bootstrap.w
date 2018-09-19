
"base/define.w" load-lib
"base/syntax.w" load-lib

[] 1 1 equal? 2 negate dig drop drop list-push-head 'true %define
[] true not swap drop list-push-head 'false %define

export-global true
export-global false

; quote^ => 'quote uplevel
; quote the next thing in the calling context
define quote^ ['quote 'uplevel uplevel]
export-global quote^

define ~dig [negate dig]
export-global ~dig

; ~ n -> n negate
define ~ [quote^ negate]
export-global ~

; uplevel% c => 'c uplevel
define uplevel% [''uplevel uplevel]
export-global uplevel%

"base/eval.w" load-lib
"base/cond.w" load-lib
"base/quasiquote.w" load-lib
"base/list.w" load-lib
"base/records.w" load-lib
"base/env.w" load-lib
"base/port.w" load-lib
"base/process.w" load-lib

;;; vi: ft=scheme

