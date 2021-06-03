
local i = ...

local base = require("base")
local Type = base.Type
local List = require("list")
local S = base.Symbol.new

local mod = {}
package.loaded["cil/base"] = mod

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

mod.Expr = Expr

local EvalContext = Type.new("cil/eval-context")
local EVAL_CONTEXT = S"%cil/eval-context"
function EvalContext.new(inputs)
    return setmetatable({
        global = {
            gensym = 0,
            indent = 0,
        },
        statements = {},
        args = {},
        inputs = inputs,
    }, EvalContext)
end

function subcontext(ctx, inputs)
    return setmetatable({
        global = ctx.global,
        statements = {},
        args = {},
        inputs = inputs,
    }, EvalContext)
end

function EvalContext.expect(i, body)
    if body == nil then
        local ectx = i:resolve(EVAL_CONTEXT)
        if not ectx then error("Not in eval context") end
        local ctx = ectx(i)
        return i:stack_pop(EvalContext)
    else
        i:call_then(EVAL_CONTEXT, function(i)
            local ctx = i:stack_pop(EvalContext)
            return body(i, ctx)
        end)
    end
end

-- Ensure there is an eval-context for function body(i, new, ectx) to use
function EvalContext.open(i, inputs, body)
    function define_ctx(i, ctx)
        i:define(EVAL_CONTEXT, function(i) i:stack_push(ctx) end)
    end
    if i:resolve(EVAL_CONTEXT) then
        EvalContext.expect(i, function(i, parent)
            ctx = subcontext(parent, inputs)
            define_ctx(i, ctx)
            i:eval_then(function(i) body(i, ctx) end, function(i)
                define_ctx(i, parent)
            end)
        end)
    else
        local ctx = EvalContext.new(inputs)
        define_ctx(i, ctx)
        return body(i, ctx)
    end
end

function EvalContext.eval(i, body, inputs, k)
    EvalContext.open(i, inputs, function(i, ectx)
        local oldstack = i:stack_get()
        i:stack_set(List.empty())

        i:eval_then(body, function(i)
            local newstack = i:stack_get()
            i:stack_set(oldstack)
            k(i, newstack, List.new(ectx.args), ectx.statements)
        end)
    end)

end

function EvalContext:new_id(name)
    self.global.gensym = self.global.gensym + 1
    return base.Symbol.new((name or "v") .. tostring(self.global.gensym))
end

function EvalContext:new_var(name)
    return Expr.new(self:new_id(name), true)
end

function EvalContext:expect_value(i, name, orelse)
    if i:stack_length() > 0 then return i:stack_pop() end
    local vr
    if self.inputs:length() > 0 then
        self.inputs, vr = self.inputs:pop()
    elseif orelse ~= nil then
        vr = orelse()
    else
        vr = self:new_id(name)
    end
    table.insert(self.args, vr)
    return vr
end

function EvalContext:expect_all(i, name, args)
    local vars = {}
    for _, iv in List.ipairs(args) do
        table.insert(vars, self:expect_value(i, name, function()
            return iv
        end))
    end
    return vars
end

function EvalContext:emit_statement(stmt)
    stmt = List.to_table(stmt)
    stmt = { string.rep("    ", self.global.indent), unpack(stmt) }
    table.insert(self.statements, stmt)
end

function EvalContext:indent() self.global.indent = self.global.indent + 1 end
function EvalContext:unindent() self.global.indent = self.global.indent - 1 end

mod.EvalContext = EvalContext

function mod.expect_value(i)
    local ctx = EvalContext.expect(i)
    return ctx:expect_value(i)
end
i:define(S"cil/expect-value", function(i)
    i:stack_push(mod.expect_value(i))
end)

--     i:define(S"cil/indent>", function() ectx:indent() end)
--     i:define(S"cil/indent<", function() ectx:unindent() end)
--     i:define(S"cil/new-id", function(i)
--         i:stack_push(ectx:new_id())
--     end)
-- end
-- i:define(S"cil/emit-statement", function(i)
--     ectx:emit_statement(stmt)
-- end)

function mod.expect_values_list(i, n)
    local ctx = EvalContext.expect(i)
    local l = {}
    for _ = 1, n do
        table.insert(l, ctx:expect_value(i))
    end
    i:stack_push(List.new(l):reverse())
    -- while #l > 0 do
    --     i:stack_push(table.remove(l))
    -- end
end
i:define(S"cil/expect-values/list", function(i)
    local n = i:stack_pop("number")
    mod.expect_values_list(i, n)
end)


