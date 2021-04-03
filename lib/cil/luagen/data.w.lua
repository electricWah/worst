
-- Hand-written replacement for data.w
local interp = ...

local base = require("base")
local Type = base.Type
local List = require("list")

local S = base.Symbol.new

local Expr = Type.new("cil/expr")
function Expr:__tostring() return "<expr>" end

function Expr.new(value, precedence)
    return setmetatable({
        value = value,
        precedence = precedence or 10,
    }, Expr)
end

function Expr:is_compound() return self.precedence ~= true end

function Expr:set_callable(args, retc)
    self.arguments = args
    self.returns = retc
end

local function value_tostring(v)
    if Type.is(Expr, v) then
        return value_tostring(v.value)
    elseif Type.is("string", v) then
        return base.to_string_debug(v)
    elseif Type.is("boolean", v) then
        if v then return "true" else return "false" end
    elseif Type.is(List, v) then
        local t = {}
        for k in v:iter() do
            table.insert(t, value_tostring_prec(v))
        end
        return "{" .. table.concat(t, ", ") .. "}"
    else
        return base.to_string_terse(v)
    end
end

local function value_tostring_prec(v, prec)
    prec = prec or 10
    if Type.is(Expr, v) and v:is_compound() then
        local t = {}
        for vv in v.value:iter() do
            if not Type.is("string", vv) then
                vv = value_tostring_prec(vv, v.precedence)
            end
            table.insert(t, vv)
        end
        local ts = table.concat(t)
        if prec < v.precedence then ts = "(" .. ts .. ")" end
        return ts
    else
        return value_tostring(v)
    end
end

interp:define(S"cil/make-expr", function(i)
    local p = i:stack_pop({"number", "boolean"}) or 10
    local v = i:stack_pop(Type.is("number", p) and List or nil)
    i:stack_push(Expr.new(v, p))
end)

interp:define(S"cil/expr?", function(i)
    i:stack_push(Type.is(Expr, i:stack_ref(1)))
end)

interp:define(S"cil/expr->string", function (i)
    i:stack_push(value_tostring_prec(i:stack_pop()))
end)

interp:define(S"cil/expr-callable-inputs", function (i)
    local e = i:stack_ref(1, Expr)
    i:stack_push(e.arguments or false)
end)

interp:define(S"cil/expr-callable-outputs", function (i)
    local e = i:stack_ref(1, Expr)
    i:stack_push(e.returns or false)
end)

interp:define(S"cil/set-expr-callable", function (i)
    local outs = i:stack_pop("number")
    local args = i:stack_pop(List)
    local e = i:stack_ref(1, Expr)
    e:set_callable(args, outs)
end)

