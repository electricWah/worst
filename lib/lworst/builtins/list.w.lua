
local base = require "lworst/base"
local List = require "lworst/list"

return function(i)

i:define("list?", function(i)
    local v = i:stack_ref(1)
    i:stack_push(List.is(v))
end)

i:define("list-empty?", function(i)
    local l = i:stack_ref(1, List)
    local len = l:length()
    i:stack_push(len == 0)
end)

i:define("list-push", function(i)
    local v = i:stack_pop()
    local l = i:stack_pop(List)
    local newl = l:push(v)
    i:stack_push(newl)
end)

i:define("list-pop", function(i)
    local l = i:stack_pop(List)
    local newl, v = l:pop()
    if not newl then return i:error("list-empty") end
    i:stack_push(newl)
    i:stack_push(v)
end)

i:define("list-append", function(i)
    local b = i:stack_pop(List)
    local a = i:stack_pop(List)
    local l = List.append(a, b)
    i:stack_push(l)
end)

i:define("list-reverse", function(i)
    local l = i:stack_pop(List)
    local newl = l:reverse()
    i:stack_push(newl)
end)

i:define("list-length", function(i)
    local l = i:stack_ref(1, List)
    i:stack_push(l:length())
end)

i:define("list-ref", function(i)
    local n = i:stack_ref(1, "number")
    local l = i:stack_ref(2, List)
    if n >= 0 and n == math.floor(n) and n < l:length() then
        i:stack_push(l:index(n))
    else
        i:stack_push(false)
    end
end)

end

