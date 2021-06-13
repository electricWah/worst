
local base = require "base"
local List = require "list"
local Symbol = base.Symbol
local Type = base.Type

return function(i)

i:define("string->symbol", function(i)
    local v = i:stack_pop("string")
    i:stack_push(Symbol.new(v))
end)

i:define("symbol->string", function(i)
    local v = i:stack_pop(Symbol)
    i:stack_push(Symbol.unwrap(v))
end)

i:define("to-string/terse", function(i)
    local v = i:stack_ref(1)
    i:stack_push(base.to_string_terse(v))
end)

i:define("to-string/debug", function(i)
    local v = i:stack_ref(1)
    i:stack_push(base.to_string_debug(v))
end)

i:define("string-append", function(i)
    local b = i:stack_pop("string")
    local a = i:stack_pop("string")
    i:stack_push(a .. b)
end)

i:define("string-join", function(i)
    local sep = i:stack_pop("string")
    local strs = i:stack_pop(List)
    local t = strs:to_table()
    for _, v in ipairs(t) do
        if not Type.is("string", v) then
            i:error("wrong-type", "list of strings", strs)
        end
    end
    i:stack_push(table.concat(t, sep))
end)

i:define("string-global-matches", function(i)
    local pat = i:stack_pop("string")
    local str = i:stack_pop("string")
    local t = {}
    for c in string.gmatch(str, pat) do
        table.insert(t, c)
    end
    i:stack_push(List.create(t))
end)

i:define("string-split", function(i)
    local n = i:stack_pop("number")
    local v = i:stack_pop("string")
    if n >= 0 and n == math.floor(n) and n <= string.len(v) then
        local pre = string.sub(v, 1, n)
        local post = string.sub(v, n + 1)
        i:stack_push(pre)
        i:stack_push(post)
    else
        i:stack_push(false)
    end
end)

i:define("string-ref", function(i)
    local n = i:stack_ref(1, "number")
    local v = i:stack_ref(2, "string")
    if n >= 0 and n == math.floor(n) and n < string.len(v) then
        i:stack_push(string.sub(v, n + 1, n + 1))
    else
        i:stack_push(false)
    end
end)

i:define("string-length", function(i)
    local s = i:stack_ref(1, "string")
    i:stack_push(string.len(s))
end)

end

