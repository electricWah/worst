
local base = require "lworst/base"
local Type = base.Type
local List = require "lworst/list"

local S = base.Symbol.new

local mod = {}

local Expr = Type.new("lua-expr")
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

function exprify(v)
    if Expr.is(v) then return v else return Expr.new(v, true) end
end

-- a[b] (or a.b if b is a symbol)
function mod.index(a, b)
    -- this may not string properly without exprify?
    if base.Symbol.is(b) then
        return Expr.new(List.new{ a, ".", b }, 10)
    else
        return Expr.new(List.new{ a, "[", b, "]" }, 10)
    end
end

local value_tostring_prec
local function value_tostring(v)
    if Type.is(Expr, v) then
        return value_tostring(v.value)
    elseif Type.is("number", v) then
        return base.to_string_debug(v)
    elseif Type.is("string", v) then
        return base.to_string_debug(v)
    elseif Type.is("boolean", v) then
        if v then return "true" else return "false" end
    elseif Type.is(List, v) then
        local t = {}
        for vv in v:iter() do
            table.insert(t, value_tostring_prec(vv))
        end
        return "{" .. table.concat(t, ", ") .. "}"
    else
        return base.to_string_terse(v)
    end
end

value_tostring_prec = function(v, prec)
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
mod.value_tostring_prec = value_tostring_prec

function unique_pairs(a, b)
    a, b = List.to_table(a), List.to_table(b)
    -- do return a, b end
    local aa, bb = {}, {}
    for i = 1, math.min(#a, #b) do
        if a[i] ~= b[i] then
            table.insert(aa, a[i])
            table.insert(bb, b[i])
        end
    end
    -- for i = math.min(#a, #b) + 1, math.max(#a, #b) do
    --     table.insert(aa, a[i])
    --     table.insert(bb, b[i])
    -- end
    return aa, bb
end
mod.unique_pairs = unique_pairs

function csv_into(acc, t)
    if List.len(t) == 0 then return end
    for _, n in List.ipairs(t) do
        table.insert(acc, value_tostring_prec(n))
        table.insert(acc, ", ")
    end
    table.remove(acc)
end
mod.csv_into = csv_into

function csv(t)
    local r = {}
    csv_into(r, t)
    return unpack(r)
end
mod.csv = csv

function assignment(names, vals, new)
    local namelen = List.len(names)
    if namelen == 0 or (not new and List.len(vals) == 0) then return nil end
    local a = {}
    if new then table.insert(a, "local ") end
    csv_into(a, names)
    if List.len(vals) > 0 then
        table.insert(a, " = ")
        csv_into(a, vals)
    end
    return unpack(a)
end
mod.assignment = assignment

function mulrets(ctx, n, v, name)
    if n == true then
        -- true = pure, no assignment needed
        return v
    elseif n == 0 then
        -- no return values at all
        ctx:emit(value_tostring_prec(v))
    else
        -- local r1, r2, ...rN = v
        -- return r1, r2, ...rN
        local rets = {}
        for _ = 1, n do
            table.insert(rets, ctx:gensym(name))
        end
        ctx:emit(assignment(rets, {v}, true))
        return unpack(rets)
    end
end

function mod.function_call(ctx, f, rcount, args, name)
    local a = { f, "(" }
    csv_into(a, args)
    table.insert(a, ")")
    return mulrets(ctx, rcount, Expr.new(List.new(a)), name)
end

function mod.method_call(ctx, obj, m, rcount, args, name)
    local a = { obj, ":", m, "(" }
    csv_into(a, args)
    table.insert(a, ")")
    return mulrets(ctx, rcount, Expr.new(List.new(a)), name)
end

mod.syntax = {}

function prefix1op(op, prec)
    return function(v)
        return Expr.new(List.new{ S(op), " ", exprify(v) }, prec)
    end
end

function infix2op(op, prec)
    return function(a, b)
        return Expr.new(List.new{
            exprify(a), " ", S(op), " ", exprify(b)
        }, prec)
    end
end

function addconst(name)
    mod.syntax[name] = Expr.new(S(name), true)
end
function addprefix1op(prec, name)
    mod.syntax[name] = prefix1op(name, prec)
end
function addinfix2op(prec, name)
    mod.syntax[name] = infix2op(name, prec)
end

addconst("nil")
addconst("...")
addinfix2op(1, "^")
addprefix1op(2, "not")
addprefix1op(2, "#")
addprefix1op(2, "-")
addinfix2op(3, "*")
addinfix2op(3, "/")
addinfix2op(3, "%")
addinfix2op(4, "+")
addinfix2op(4, "-")
addinfix2op(5, "..")
addinfix2op(6, "<")
addinfix2op(6, ">")
addinfix2op(6, "<=")
addinfix2op(6, ">=")
addinfix2op(6, "~=")
addinfix2op(6, "==")
addinfix2op(7, "and")
addinfix2op(8, "or")

return mod


