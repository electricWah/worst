
; fn say-my-name [string name] void [
;     "Hello, " er + "\n" + print
; ]

target lua
require pico8

; "hello pico8" printh

; false if [
;     "hello" print
; ] [
;     "goodbye" print
; ]

while [false] [
    "test" print
]

define testret [
    ;
    print
    "ret a string" 3
]
"testret" testret
drop print

define testif [
    if ["local ifelse" print] ["that's right" print]
]

; ; declare-function-type test [string] []
define test [
    "inside test" print
    while [false] [ "hello" print ]
    print
    print
    print
    "gone o test" print
    6 "egg"
]

"a" "b" "c" test

; op

; "hello" say-my-name

;;; vi: ft=scheme

