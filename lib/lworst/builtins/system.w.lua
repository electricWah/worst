
local List = require "lworst/list"

return function(i)

i:define("env-get", function(i)
    local name = i:stack_ref(1, "string")
    local value = os.getenv(name) or false
    i:stack_push(value)
end)

i:define("stack-dump", function(i)
    print(i:stack_get())
end)

i:define("current-call-stack", function(i)
    i:stack_push(i:call_stack())
end)

i:define("stack-get", function(i)
    i:stack_push(i:stack_get())
end)

i:define("stack-set", function(i)
    local new = i:stack_pop(List)
    i:stack_set(new)
end)

i:define("stack-length", function(i)
    i:stack_push(i:stack_length())
end)

i:define("cpu-time", function(i)
    i:stack_push(require("os").clock())
end)

end

