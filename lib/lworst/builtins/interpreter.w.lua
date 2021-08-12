
local base = require "lworst/base"
local Interpreter = require "lworst/interpreter"
local Port = require "lworst/port"
local List = require "lworst/list"
local Map = require "lworst/map"
local Symbol = base.Symbol

return function(i)

-- this is a strange idea
-- i:define("current-interpreter", function(i) i:stack_push(i) end)

i:define("interpreter-empty", function(i)
    i:stack_push(Interpreter.empty())
end)

i:define("interpreter-eval", function(i)
    local v = i:stack_pop()
    local interp = i:stack_ref(1, Interpreter)
    interp:eval(v)
end)

i:define("interpreter-definition-add", function(i)
    local name = i:stack_pop(Symbol)
    local body = i:stack_pop({List, "function"})
    local interp = i:stack_ref(1, Interpreter)
    interp:define(name, body)
end)

i:define("interpreter-run", function(i)
    local interp = i:stack_ref(1, Interpreter)
    local err = interp:run()
    i:stack_push(err or false)
end)

i:define("interpreter-reset", function(i)
    local interp = i:stack_ref(1, Interpreter)
    interp:reset()
end)

i:define("interpreter-stack-get", function(i)
    local interp = i:stack_ref(1, Interpreter)
    i:stack_push(interp:stack_get())
end)

i:define("interpreter-stack-set", function(i)
    local s = i:stack_pop(list)
    local interp = i:stack_ref(1, Interpreter)
    interp:stack_set(s)
end)

i:define("interpreter-body-get", function(i)
    local interp = i:stack_ref(1, Interpreter)
    i:stack_push(interp:get_body())
end)

i:define("interpreter-body-set", function(i)
    local s = i:stack_pop(list)
    local interp = i:stack_ref(1, Interpreter)
    interp:set_body(s)
end)

i:define("interpreter-toplevel", function(i)
    local interp = i:stack_ref(1, Interpreter)
    i:stack_push(interp:is_toplevel())
end)

i:define("set-trace-port", function(i)
    local p = i:stack_pop({ Port.OutputPort, false })
    i:set_trace_port(p)
end)

i:define("all-definitions", function(i)
    i:stack_push(Map.new(i:all_definitions()))
end)

i:define("current-context-set-code", function(i)
    local body = i:stack_pop(List)
    i:set_body(body)
end)

i:define("current-context-clear", function(i)
    i:set_body(List.empty())
end)

i:define("current-context-definitions", function(i)
    local m = Map.empty()
    for k, v in pairs(i:definitions()) do
        m = Map.set(m, k, base.clone(v))
    end
    i:stack_push(m)
end)

-- Should be a map of symbols to definitions
i:define("current-context-define-all", function(i)
    local m = i:stack_pop(Map)
    for k, v in m:iter() do
        local ks = i:assert_type(k, Symbol)
        local vs = i:assert_type(v, {List, "function"})
        i:define(ks, vs)
    end
end)

end

