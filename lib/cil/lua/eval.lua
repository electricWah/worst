
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

return mod



