
local base = require("base")
local Type = base.Type
local List = require("list")

local cil = require("cil/base")
local Expr = cil.Expr

local S = base.Symbol.new

local mod = {}
package.loaded["cil/lua/base"] = mod

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
    return r
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
    return a
end
mod.assignment = assignment

function emit_assignment(ectx, names, vals, new)
    local a = assignment(names, vals, new)
    if a ~= nil then
        ectx:emit_statement(List.new(a))
    end
end
mod.emit_assignment = emit_assignment

