
local base = require "lworst/base"
local Map = require "lworst/map"

return function(i)

i:define("map?", function(i)
    local m = i:stack_ref(1)
    i:stack_push(Map.is(m))
end)

i:define("map-empty", function(i)
    i:stack_push(Map.empty())
end)

i:define("map-exists", function(i)
    local k = i:stack_ref(1)
    local m = i:stack_ref(2, Map)
    local v = m:has_key(k)
    i:stack_push(v)
end)

i:define("map-get", function(i)
    local k = i:stack_ref(1)
    local m = i:stack_ref(2, Map)
    i:stack_push(m:get(k) or false)
end)

i:define("map-set", function(i)
    local v = i:stack_pop()
    local k = i:stack_pop()
    local m = i:stack_pop(Map)
    local r = Map.set(m, k, v)
    i:stack_push(r)
end)

i:define("map-remove", function(i)
    local k = i:stack_pop()
    local m = i:stack_pop(Map)
    local r = Map.remove(m, k)
    i:stack_push(r)
end)

i:define("map-keys", function(i)
    local m = i:stack_ref(1, Map)
    i:stack_push(m:keys())
end)

end

