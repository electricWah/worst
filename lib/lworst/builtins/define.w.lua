
local base = require "lworst/base"
local List = require "lworst/list"
local Symbol = base.Symbol
local S = Symbol.new

return function(i)

i:define("definition-add", function(i)
    local name = i:stack_pop(Symbol)
    local body = i:stack_pop_value({List, "function"})
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
    local def = i:resolve_value(name) or false
    i:stack_push(def)
end)

-- -- "lexical scope"
-- i:define("definition-wrap-definitions", function(i)
--     local defs = i:stack_pop(Map)
--     local body = i:stack_pop(List)
--     i:stack_push(function(i)
--         for k, v in Map.iter(defs) do
--             i:define(k, v)
--         end
--         i:eval(body)
--     end)
-- end)

local next_dynamic = false

i:define("dynamic", function(i)
    next_dynamic = true
end)

-- attribute: wrap body in current definitions
function wrap_lexical_attr(i)
    -- this means it should probably be the last thing?
    -- also not sure why it's here?
    i:into_parent()

    if next_dynamic then
        next_dynamic = false
        return
    end

    local body_value = i:stack_pop_value() --:shallow_clone()

    local body = base.Value.unwrap(body_value)
    local all_defs = i:all_definitions()
    
    body_value.value = function(i)
        i:step_into_new(body)
        for k, v in pairs(all_defs) do
            i:define(k, v)
        end
    end
    i:stack_push(body_value)
end

i:define("default-attributes", function(i)
    i:eval(wrap_lexical_attr)
end)

-- define (attributes) name [ body ... ]
i:define("define", function(i)
    local attrs, name
    local name_or_attrs = i:quote("define")
    if Symbol.is(name_or_attrs) then
        attrs = List.new()
        name = name_or_attrs
    elseif List.is(name_or_attrs) then
        attrs = name_or_attrs
        name = i:quote("define")
    else
        return i:error("TODO define _ name [...]" )
    end
    local body = i:quote("define")

    i:stack_push(name)
    i:stack_push(body)

    -- ugly nonsense, don't know why it works
    i:step_into_new()
    i:define(S"%eval-attributes", List.new({true}))
    i:eval(attrs)
    i:call(S"default-attributes")
    i:into_parent()

    local def = i:stack_pop_value()
    local name = i:stack_pop()
    def.name = name
    def.attributes = attrs
    def.body = body
    i:define(name, def)
end)

end
