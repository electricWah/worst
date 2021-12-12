
local base = require "lworst/base"
local Type = base.Type
local List = require "lworst/list"

local ce = require "compile/evaluate"
local luabase = require "compile/lua/base"
local control = require "compile/lua/control"

return function(i, parent)

function binop(def, lua)
    i:define(def, function(i)
        local ctx = ce.context(i)
        local a = ctx:stack_pop(i, def .. "_a")
        local b = ctx:stack_pop(i, def .. "_b")
        i:stack_push(luabase.syntax[lua](a, b))
    end)
end

binop("add", "+")
binop("mul", "*")
binop("sub", "-")
binop("div", "/")

function extern(name, argn, retn)
    i:define(name, function(i)
        local ctx = ce.context(i)
        local args = {}
        for a = 1, argn do
            table.insert(args, ctx:stack_pop(i, name .. tostring(a) .. "_"))
        end
        local rets = { luabase.function_call(ctx, name, retn, args, name) }
        for _, r in ipairs(rets) do
            i:stack_push(r)
        end
    end)
end

extern("print", 1, 0)

-- some mostly stack op bits that simply work
for _, name in ipairs({ "drop", "swap", "dig", "bury" }) do
    i:define(name, parent:resolve(base.Symbol.new(name)))
end

i:define("loop", function(i)
    local body = i:quote()
    control.emit_loop(i, body)
end)

i:define("if", function(i)
    local ctx = ce.context(i)
    local iftbody = i:quote()
    local iffbody = i:quote()
    local ifcond = ctx:stack_pop(i, "ifc")
    control.emit_if_then_else(ctx, i, ifcond, iftbody, iffbody)
end)

i:define("define", function(i)
    local ctx = ce.context(i)
    local name = i:quote()
    local body = i:quote()
    local defname, argn, retn = control.emit_function(ctx, i, name, body)
    i:define(name, function(i)
        local args = {}
        for a = 1, argn do
            table.insert(args, ctx:stack_pop(i, base.Symbol.unwrap(name) .. tostring(a) .. "_"))
        end
        local rets = { luabase.function_call(ctx, defname, retn, args, name) }
        for _, r in ipairs(rets) do
            i:stack_push(r)
        end
    end)
end)

end

