
local base = require "lworst/base"
local Type = base.Type
local List = require "lworst/list"
local S = base.Symbol.new

local mod = require "cil/base"

return function(i)

i:define(S"cil/expect-value", function(i)
    i:stack_push(mod.expect_value(i))
end)

i:define(S"cil/expect-values/list", function(i)
    local n = i:stack_pop("number")
    mod.expect_values_list(i, n)
end)

end

