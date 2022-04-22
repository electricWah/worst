
import worst/misc
import assert_t

"5 and 5" equal! (5 5)

"egg 7" equal! (define egg (7) 7 egg)
"not egg" test! (quote egg definition-resolve swap drop not)

"stack empty" equal! (stack-get ())

import data/list

"list-member 1" test! (7 () list-member not)
"list-member 2" test! (7 (7) list-member)
"list-member 3" test! ("a" ("a" "b" "c") list-member)
"list-member 4" test! ("b" ("a" "b" "c") list-member)
"list-member 5" test! ("c" ("a" "b" "c") list-member)
"list-member 6" test! ("d" ("a" "b" "c") list-member not)

