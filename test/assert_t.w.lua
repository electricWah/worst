
local base = require "lworst/base"
local List = require "lworst/list"

return function(i)

i:define("equal!", function(i)
    local e = i:quote()
    local msg = i:quote()
    i:eval(e)
    local b = i:stack_pop()
    local a = i:stack_pop()
    if a ~= b then
        i:error("not equal! " .. tostring(msg), a, b, e)
    end
end)

i:define("test!", function(i)
    local e = i:quote()
    local msg = i:quote()
    i:eval(e)
    local v = i:stack_pop()
    if not base.unwrap_lua(v) then
        i:error("not true! " .. tostring(msg), v, e)
    end
end)

end

