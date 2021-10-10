
-- Expression makers and sublanguage

local base = require "lworst/base"
local Type = base.Type
local List = require "lworst/list"

local S = base.Symbol.new

local eval = require "cil/eval"
local luabase = require "cil/lua/base"
local Expr = luabase.Expr

local mod = {}

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
        return Expr.new(List.new{ a, ".", b }, 10)
    else
        return Expr.new(List.new{ a, "[", b, "]" }, 10)
    end
end

function mulrets(ctx, n, v, name)
    if n == true then
        -- true = pure, no assignment needed
        return v
    elseif n == 0 then
        -- no return values at all
        ctx:emit({ luabase.value_tostring_prec(v) })
    else
        -- local r1, r2, ...rN = v
        -- return r1, r2, ...rN
        local rets = {}
        for _ = 1, n do
            table.insert(rets, ctx:gensym(name))
        end
        luabase.emit_assignment(ctx, rets, {v}, true)
        return unpack(rets)
    end
end

function mod.function_call(ctx, f, rcount, args, name)
    local a = { f, "(" }
    luabase.csv_into(a, args)
    table.insert(a, ")")
    return mulrets(ctx, rcount, Expr.new(List.new(a)), name)
end

function mod.method_call(ctx, obj, m, rcount, args, name)
    local a = { obj, ":", m, "(" }
    luabase.csv_into(a, args)
    table.insert(a, ")")
    return mulrets(ctx, rcount, Expr.new(List.new(a)), name)
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

return mod

