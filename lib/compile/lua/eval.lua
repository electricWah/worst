
local base = require "lworst/base"
local Type = base.Type
local List = require "lworst/list"
local Interpreter = require "lworst/interpreter"

local evaluate = require "compile/evaluate"
local S = base.Symbol.new

local luabase = require "compile/lua/base"
local Expr = luabase.Expr

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
    }, getmetatable(self))
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

function Context:emit_raw(line)
    table.insert(self.lines, line)
end

function Context:emit(...)
    if #{...} == 0 then return end
    local indent = string.rep("    ", self.global.indentation)
    table.insert(self.lines, table.concat({ indent, ... }))
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

function Context:try_resolve(parent, name)
    return nil
end

function mod.evaluate(parent, body, interp, inputs, context)
    interp = interp or Interpreter.empty()
    inputs = inputs or List.new{}
    context = context or Context.new()

    -- print("evaluate", body)
    for ev in evaluate.evaluator(context, interp, body) do
        -- print("evaled", ev)
        if base.Error.is(ev) and ev.message == "stack-empty" then
            interp:stack_push(context:stack_pop(interp))
        elseif base.Error.is(ev) and ev.message == "undefined" then
            local name = ev.irritants:head()
            local def
            if context.parent == nil then
                def = context:try_resolve(parent, name)
            else
                def = parent:try_resolve(name)
            end
            if def == nil then
                parent:pause(ev)
            else
                interp:stack_push(def)
            end
        else
            parent:pause(ev)
        end
    end
    return List.new(context.lines), List.new(context.args), interp:stack_get()
end

function Context:evaluate(parent, body, inputs)
    return mod.evaluate(parent, body, nil, inputs, self:inner())
end

return mod

