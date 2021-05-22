
-- Expression makers and sublanguage

local i = ...

local base = require("base")
local Type = base.Type
local List = require("list")

local cil = require("cil/base")
local Expr = cil.Expr

local S = base.Symbol.new

local mod = {}
package.loaded["cil/lua/expr"] = mod

-- reassign?
-- not = !
-- unary - = ~
-- and = && 
-- or = ||
-- a[b] = a -> [b]
-- a.b = a -> b
-- f() = f . ()

-- also f .* n () and obj :* method n ()
-- -> n is the number of return values

function exprify(v)
    if Expr.is(v) then return v else return Expr.new(v, true) end
end

-- a[b] (or a.b if b is a symbol)
function mod.index(a, b)
    -- this may not string properly without exprify?
    if base.Symbol.is(b) then
        return Expr.new(List.new{ a, ".", b }, 0)
    else
        return Expr.new(List.new{ a, "[", b, "]" }, 0)
    end
end

function mulrets(ectx, n, v, name)
    if n == true then
        -- true = pure, no assignment needed
        return v
    elseif n == 0 then
        -- no return values at all
        ectx:emit_statement(v)
    else
        -- local r1, r2, ...rN = v
        -- return r1, r2, ...rN
        local rets = {}
        for i = 1, n do
            table.insert(rets, ectx:new_var(name))
        end
        luabase.emit_assignment(ectx, rets, {v}, true)
        return unpack(rets)
    end
end

function mod.function_call(ectx, f, rcount, args, name)
    local a = { f, "(" }
    luabase.csv_into(a, args)
    table.insert(a, ")")
    return mulrets(ectx, rcount, Expr.new(List.new(a)), name)
end

function mod.method_call(ectx, obj, m, rcount, args, name)
    local a = { obj, ":", m, "(" }
    luabase.csv_into(a, args)
    table.insert(a, ")")
    return mulrets(ectx, rcount, Expr.new(List.new(a)), name)
end

-- Lua's names for everything
mod.lua = {}
-- Number of arguments everything takes
mod.arity = {}
-- Worst definitions for Lua stuff
mod.defines = {}

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
    mod.lua[name] = Expr.new(S(name), true)
    mod.arity[name] = 0
end
function addprefix1op(prec, name)
    mod.lua[name] = prefix1op(name, prec)
    mod.arity[name] = 1
end
function addinfix2op(prec, name)
    mod.lua[name] = infix2op(name, prec)
    mod.arity[name] = 2
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

i:define(S"cil/lua-builtin", function(i)
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

