
local base = require "lworst/base"
local Place = base.Place

return function(i)

i:define("place?", function(i)
    local v = i:stack_ref(1)
    i:stack_push(Place.is(v))
end)

i:define("make-place", function(i)
    local v = i:stack_pop()
    local p = Place.new(v)
    i:stack_push(p)
end)

i:define("place-get", function(i)
    local p = i:stack_ref(1, Place)
    local v = p:get()
    i:stack_push(v)
end)

i:define("place-set", function(i)
    local v = i:stack_pop()
    local p = i:stack_ref(1, Place)
    p:set(v)
end)

end

