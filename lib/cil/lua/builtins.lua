
local base = require "lworst/base"
local Type = base.Type
local List = require "lworst/list"

local expr = require "cil/lua/expr"
local control = require "cil/lua/control"

return function(i)

function binop(def, lua)
    i:define(def, function(i)
        local a = i:stack_pop()
        local b = i:stack_pop()
        i:stack_push(expr.lua[lua](a, b))
    end)
end

binop("add", "+")
binop("mul", "*")
binop("sub", "-")
binop("div", "/")

function extern(name, argn, retn)
    i:define(name, function(i)
        local args = {}
        for a = 1, argn do
            table.insert(args, i:stack_pop())
        end
        local rets = { expr.function_call(i, name, retn, args, name) }
        for _, r in ipairs(rets) do
            i:stack_push(r)
        end
    end)
end

extern("print", 1, 0)

i:define("loop", function(i)
    local body = i:quote()
    control.emit_loop(i, body)
end)

i:define("if", function(i)
    local iftbody = i:quote()
    local iffbody = i:quote()
    local ifcond = i:stack_pop()
    control.emit_if_then_else(i, ifcond, iftbody, iffbody)
end)

i:define("define", function(i)
    local name = i:quote()
    local body = i:quote()
    local defname, argn, retn = control.emit_function(i, name, body)
    i:define(name, function(i)
        local args = {}
        for a = 1, argn do
            table.insert(args, i:stack_pop())
        end
        local rets = { expr.function_call(i, defname, retn, args, name) }
        for _, r in ipairs(rets) do
            i:stack_push(r)
        end
    end)
end)

end

