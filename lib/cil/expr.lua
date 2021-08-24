
local base = require "lworst/base"
local Type = base.Type
local List = require "lworst/list"

local Expr = Type.new("cil/expr")
function Expr.new(value, precedence)
    return setmetatable({
        value = value,
        precedence = precedence or 10,
    }, Expr)
end
function Expr:__tostring()
    return "<expr " .. tostring(self.value) .. ">"
end
function Expr:is_compound() return self.precedence ~= true end

function Expr:set_callable(args, retc)
    self.arguments = args
    self.returns = retc
end

return Expr

