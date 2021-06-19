
-- Expression makers and sublanguage

local base = require "lworst/base"
local Type = base.Type
local List = require "lworst/list"

local cil = require "cil/base"
local Expr = cil.Expr
local EvalContext = cil.EvalContext

local S = base.Symbol.new

local luabase = require "cil/lua/base"

local mod = require "cil/lua/expr"

return function(i)

i:define(S"cil/lua-expression", function(i)
    local name = i:stack_pop("string")
    local op = mod.lua[name]
    local arity = mod.arity[name]
    if op == nil then
        i:error("undefined", name)
    elseif arity == 0 then
        i:stack_push(op)
    elseif arity == 1 then
        local a = i:stack_pop()
        i:stack_push(op(a))
    elseif arity == 2 then
        local b = i:stack_pop()
        local a = i:stack_pop()
        i:stack_push(op(a, b))
    else
        i:error("bad-arity", name)
    end
end)

i:define(S"cil/lua-function-call", function(i)
    local rcount = i:stack_pop({"number", true})
    local name = i:stack_pop()
    local args = i:stack_pop(List)

    EvalContext.expect(i, function(i, ectx)
        local rets = { mod.function_call(ectx, name, rcount, args) }
        for _, r in ipairs(rets) do
            i:stack_push(r)
        end
    end)
end)

i:define(S"cil/lua-method-call", function(i)
    local rcount = i:stack_pop({"number", true})
    local method = i:stack_pop("string")
    local obj = i:stack_pop()
    local args = i:stack_pop(List)

    EvalContext.expect(i, function(i, ectx)
        local rets = { mod.method_call(ectx, obj, method, rcount, args) }
        for _, r in ipairs(rets) do
            i:stack_push(r)
        end
    end)
end)

i:define(S"cil/lua-index", function(i)
    local index = i:stack_pop()
    local dex = i:stack_pop()
    i:stack_push(mod.index(dex, index))
end)

end

