
import assert_t

equal! (5 5) "5 and 5"

equal! (define egg (7) 7 egg) "egg 7"
test! (quote egg definition-resolve swap drop not) "not egg"

equal! (stack-get ()) "stack empty"

