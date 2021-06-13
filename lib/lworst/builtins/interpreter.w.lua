
local base = require "base"
local Interpreter = require "interpreter"
local Port = require "port"
local List = require "list"
local Map = require "map"
local Symbol = base.Symbol

return function(i)

i:define("interpreter-dump-stack", function(i)
    print(unpack(i.stack))
end)

i:define("interpreter-call-stack", function(i)
    i:stack_push(i:call_stack())
end)

i:define("interpreter-stack", function(i)
    i:stack_push(i:stack_get())
end)

i:define("interpreter-stack-set", function(i)
    local new = i:stack_pop(List)
    i:stack_set(new)
end)

i:define("interpreter-stack-length", function(i)
    i:stack_push(i:stack_length())
end)

i:define("interpreter-cpu-time", function(i)
    i:stack_push(require("os").clock())
end)

i:define("interpreter-set-trace-port", function(i)
    local p = i:stack_pop({ Port.OutputPort, false })
    i:set_trace_port(p)
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

i:define(require("interpreter").ERROR_HANDLER, function(i)
    local v = i:stack_pop(Symbol)
    local irritants = i:stack_pop(List)
    print("error:", v, unpack(irritants:to_table()))
    i:reset()
end)

end

