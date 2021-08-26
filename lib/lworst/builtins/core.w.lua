
local base = require "lworst/base"
local List = require "lworst/list"
local Type = base.Type
local Symbol = base.Symbol

return function(i)

i:define("quote", function(i)
    i:stack_push(i:quote())
end)

i:define("uplevel", function(i)
    if not i:into_parent() then
        i:error("root-uplevel")
    else
        local v = i:stack_pop(Symbol)
        i:call(v)
    end
end)

i:define("eval", function(i)
    local v = i:stack_pop()
    i:eval(v)
end)

i:define("call", function(i)
    local v = i:stack_pop()
    i:call(v)
end)

i:define("drop", function(i)
    local v = i:stack_pop()
    base.destroy(v)
end)

i:define("equal?", function(i)
    local b = i:stack_ref(1)
    local a = i:stack_ref(2)
    i:stack_push(base.equal(a, b))
end)

i:define("clone", function(i)
    local v = i:stack_ref(1)
    i:stack_push(base.clone(v))
end)

i:define("swap", function(i)
    local a = i:stack_pop()
    local b = i:stack_pop()
    i:stack_push(a)
    i:stack_push(b)
end)

i:define("dig", function(i)
    local a = i:stack_pop()
    local b = i:stack_pop()
    local c = i:stack_pop()
    i:stack_push(b)
    i:stack_push(a)
    i:stack_push(c)
end)

i:define("bury", function(i)
    local a = i:stack_pop()
    local b = i:stack_pop()
    local c = i:stack_pop()
    i:stack_push(a)
    i:stack_push(c)
    i:stack_push(b)
end)

i:define("when", function(i)
    local name = i:stack_pop(Symbol)
    local whether = i:stack_pop("boolean")
    if whether then
        i:call(name)
    end
end)

i:define("and", function(i)
    local a = i:stack_ref(1)
    local b = i:stack_ref(2)
    i:stack_push(b and a)
end)

i:define("or", function(i)
    local a = i:stack_ref(1)
    local b = i:stack_ref(2)
    i:stack_push(b or a)
end)

i:define("number?", function(i)
    i:stack_push(Type.is("number", i:stack_ref(1)))
end)

i:define("string?", function(i)
    i:stack_push(Type.is("string", i:stack_ref(1)))
end)

i:define("bool?", function(i)
    i:stack_push(Type.is("boolean", i:stack_ref(1)))
end)

i:define("symbol?", function(i)
    i:stack_push(Type.is(Symbol, i:stack_ref(1)))
end)

i:define("false?", function(i)
    i:stack_push(not i:stack_ref(1))
end)

i:define("not", function(i)
    i:stack_push(not i:stack_pop())
end)

i:define("pause", function(i)
    i:pause(i:stack_pop())
end)

i:define("error?", function(i)
    i:stack_push(Type.is(base.Error, i:stack_ref(1)))
end)

i:define("error", function(i)
    local msg = i:stack_pop({Symbol, "string"})
    local irritants = i:stack_pop(List)
    i:error(msg, unpack(List.to_table(irritants)))
end)

i:define("error->list", function(i)
    local e = i:stack_pop(base.Error)
    i:stack_push(e:to_list())
end)

i:define("error-message", function(i)
    local e = i:stack_ref(1, base.Error)
    i:stack_push(e.message)
end)

-- string lua-load-string -> function
--                        -> error    #f
i:define("lua-load-string", function(i)
    local src = i:stack_pop("string")
    local r, err = load(src)
    if r then
        i:stack_push(r)
    else
        i:stack_push(err)
        i:stack_push(false)
    end
end)

end

