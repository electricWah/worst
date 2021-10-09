
local base = require "lworst/base"
local Type = base.Type
local List = require "lworst/list"
local Map = require "lworst/map"
local Interpreter = require "lworst/interpreter"

local Expr = require "cil/expr"
local eval = require "cil/eval"
local S = base.Symbol.new

local mod = {}

local Context = Type.new("context")
function Context.new(inputs)
    return setmetatable({
        inputs = inputs or List.new{},
        args = {},
        lines = {},
        global = {
            gensym = 0,
            indentation = 0,
        },
    }, Context)
end

function Context:inner(inputs)
    return setmetatable({
        inputs = inputs or List.new{},
        args = {},
        lines = {},
        global = self.global,
        parent = self,
    }, Context)
end

function Context:gensym(name)
    if base.Symbol.is(name) then name = base.Symbol.unwrap(name) end
    self.global.gensym = self.global.gensym + 1
    return Expr.new(S((name or "v") .. self.global.gensym), true)
end

function Context:indent()
    self.global.indentation = self.global.indentation + 1
end
function Context:unindent()
    self.global.indentation = self.global.indentation - 1
end

function Context:emit(line)
    local indent = string.rep("    ", self.global.indentation)
    table.insert(self.lines, { indent, unpack(List.to_table(line)) })
end

function Context:stack_pop(i, name)
    local v
    if i:stack_length() > 0 then
        return i:stack_pop()
    elseif self.inputs:length() > 0 then
        self.inputs, v = self.inputs:pop()
    else
        v = self:gensym(name)
    end
    table.insert(self.args, v)
    return v
end

function evaluate(context, parent, body, inputs, interp)
    inputs = inputs or List.new{}
    interp = interp or Interpreter.empty()

    -- print("evaluate", body)
    for ev in eval.evaluator(context, interp, body) do
        -- print("evaled", ev)
        if context.parent ~= nil
            and base.Error.is(ev) and ev.message == "undefined" then
            local name = ev.irritants:head()
            local def = parent:try_resolve(name)
            interp:stack_push(def)
        else
            parent:pause(ev)
        end
    end
    return List.new(context.lines), List.new(context.args), interp:stack_get()
end

function Context:evaluate(parent, body, inputs)
    return evaluate(self:inner(), parent, body, inputs, nil)
end

function mod.evaluate(parent, body, interp)
    return evaluate(Context.new(), parent, body, nil, interp)
end



function test_evaluate(parent, body, inputs, interp, toplevel)
    inputs = inputs or List.new{}
    toplevel = toplevel or interp ~= nil

    local gensym = 0
    local args = {}
    local indentation = 0
    local lines = {}

    for ev in eval.evaluator(interp, body) do
        local d = ev.value
        if eval.gensym.is(ev) then
            if toplevel then
                gensym = gensym + 1
                interp:stack_push(Expr.new(S(d .. gensym), true))
            else
                interp:stack_push(eval.gensym(parent, d))
            end
        elseif eval.emit.is(ev) then
            local st = { string.rep("    ", indentation), unpack(List.to_table(d)) }
            table.insert(lines, st)
        elseif eval.indent.is(ev) then
            indentation = indentation + 1
        elseif eval.unindent.is(ev) then
            indentation = indentation - 1
        elseif eval.stack_push.is(ev) then
            -- in emit_builtin mode, um, emit stack_push I guess
            interp:stack_push(d)
        elseif eval.stack_pop.is(ev) then
            -- no else, it would just be stack_push(stack_pop())
            if interp:stack_length() == 0 then
                local v
                if inputs:length() > 0 then
                    inputs, v = inputs:pop()
                elseif toplevel then
                    gensym = gensym + 1
                    v = Expr.new(S("v" .. gensym), true)
                else
                    v = mod.gensym(parent, "v")
                end
                interp:stack_push(v)
                table.insert(args, v)
            end
        elseif not toplevel
                and base.Error.is(r) and r.message == "undefined" then
            local name = r.irritants:head()
            local def = parent:try_resolve(name)
            interp:stack_push(def)
        else
            parent:pause(d)
        end
    end

    return List.new(lines), List.new(args), interp:stack_get()
end

return mod



