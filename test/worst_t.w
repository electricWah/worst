
import worst/misc
import assert_t

"5 and 5" equal! (5 5)

"egg 7" equal! (define egg (7) 7 egg)
"not egg" test! (quote egg definition-resolve swap drop not)

"stack empty" equal! (stack-get ())

