
local base = require "lworst/base"
local List = require "lworst/list"
local Map = require "lworst/map"
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

-- "lexical scope"
i:define("definition-wrap-definitions", function(i)
    local defs = i:stack_pop(Map)
    local body = i:stack_pop(List)
    i:stack_push(function(i)
        for k, v in pairs(defs) do
            i:define(k, v)
        end
        i:eval(body)
    end)
end)

end

