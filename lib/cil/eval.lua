
local base = require "lworst/base"
local Type = base.Type
local List = require "lworst/list"
local Interpreter = require "lworst/interpreter"

local Expr = require "cil/expr"
local S = base.Symbol.new

local mod = {}

function make_yield(name, ty)
    local t = Type.new(name)
    function t.new(v)
        return setmetatable({ value = v }, t)
    end
    t.yield = base.contract({Interpreter, ty or true}, {}, function(i, v)
        i:pause(t.new(v))
    end)
    return t
end

local Gensym = make_yield("gensym", "string")
local Emit = make_yield("emit")
local Indent = make_yield("indent")

mod.Gensym = Gensym
mod.Emit = Emit
mod.Indent = Indent

function mod.gensym(i, name)
    if base.Symbol.is(name) then name = base.Symbol.unwrap(name) end
    Gensym.yield(i, name)
    return i:stack_pop()
end

function mod.emit(i, line)
    Emit.yield(i, line)
end

function mod.indent(i) Indent.yield(i, 1) end
function mod.unindent(i) Indent.yield(i, -1) end

function evaluate(parent, body, inputs, interpreter, resolve_barrier)
    inputs = inputs or List.new{}
    local interp = interpreter or Interpreter.empty()
    interp:set_body(body)
    resolve_barrier = resolve_barrier or interpreter ~= nil

    local args = {}
    local gensym = 0
    local indentation = 0
    local lines = {}

    while true do
        local r = interp:run()
        if r == nil then
            break
        elseif Gensym.is(r) then
            gensym = gensym + 1
            interp:stack_push(Expr.new(S(r.value .. gensym), true))
        elseif Emit.is(r) then
            st = { string.rep("    ", indentation), unpack(List.to_table(r.value)) }
            table.insert(lines, st)
        elseif Indent.is(r) then
            indentation = indentation + r.value
        elseif base.Error.is(r) and r.message == "stack-empty" then
            local v
            if inputs:length() > 0 then
                inputs, v = inputs:pop()
            else
                gensym = gensym + 1
                v = Expr.new(S("v" .. gensym), true)
            end
            interp:stack_push(v)
            table.insert(args, v)
        elseif not resolve_barrier
                and base.Error.is(r) and r.message == "undefined" then
            local name = r.irritants:head()
            local def = parent:try_resolve(name)
            interp:stack_push(def)
        else
            parent:pause(r)
        end
    end

    return List.new(lines), List.new(args), interp:stack_get()

end

mod.evaluate = evaluate

return mod

