
local base = require "lworst/base"
local Type = base.Type
local List = require "lworst/list"

local cil = require "cil/base"
local Expr = cil.Expr
local EvalContext = cil.EvalContext

local luabase = require "cil/lua/base"
local luaexpr = require "cil/lua/expr"

local S = base.Symbol.new

local mod = require "cil/lua/control"

return function(i)

i:define(S"cil/lua-if-then-else", function(i)
    local iftbody = i:stack_pop(List)
    local iffbody = i:stack_pop(List)
    local ifcond = i:stack_pop()
    mod.emit_if_then_else(i, ifcond, iftbody, iffbody)
end)

i:define(S"cil/lua-loop", function(i)
    local body = i:stack_pop(List)
    mod.emit_loop(i, body)
end)

i:define(S"cil/lua-break", mod.emit_break)

i:define(S"cil/lua-function", mod.emit_function)

end

