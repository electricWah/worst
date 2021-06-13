
local base = require "base"
local List = require "list"
local Symbol = base.Symbol

return function(i)

i:define("definition-add", function(i)
    local name = i:stack_pop(Symbol)
    local body = i:stack_pop({List, "function"})
    i:define(name, body)
end)

i:define("definition-get", function(i)
    local name = i:stack_ref(1, Symbol)
    local def = i:definition_get(name) or false
    i:stack_push(def)
end)

i:define("definition-remove", function(i)
    local name = i:stack_pop(Symbol)
    i:definition_remove(name)
end)

i:define("definition-resolve", function(i)
    local name = i:stack_ref(1, Symbol)
    local def = i:resolve(name) or false
    i:stack_push(def)
end)

end

