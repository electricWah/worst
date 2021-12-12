
local mod = {}

local scope = {}
local successes = {}
local failures = {}

function fmt_str(s) return string.format("%q", s) end

function mod.check(v, msg)
    mod.func(function() if not v then error(msg or "check failed", 6) end end)
end

function mod.check_equal_with(a, b, f, msg)
    mod.check(f(a, b), tostring(msg or "not equal") .. ": "
        .. fmt_str(a) ..  " ~= " .. fmt_str(b))
end

function mod.check_equal(a, b, msg)
    mod.check(a == b, tostring(msg or "not equal") .. ": "
        .. fmt_str(a) ..  " ~= " .. fmt_str(b))
end

function mod.func(f)
    local r, msg = pcall(f)
    local t = successes
    if not r then t = failures end
    local log = {unpack(scope)}
    table.insert(log, msg)
    table.insert(t, log)
end

function mod.table(t)
    local pt = {}
    for k, v in pairs(t) do
        table.insert(pt, {k=k, v=v})
    end
    table.sort(pt, (function(a, b) return a.k < b.k end))
    for i, v in ipairs(pt) do
        mod.run(tostring(v.k), v.v)
    end

    for i, v in ipairs(t) do
        mod.run(tostring(i), v)
    end
end

function mod.run(name, v)
    if not v then
        v = name
        name = nil
    end
    if name then table.insert(scope, name) end
    local t = {
        ["function"] = mod.func,
        ["table"] = mod.table,
        ["boolean"] = (function() end),
    }
    t[type(v)](v)
    if name then table.remove(scope) end
end

function mod.modules(t)
    local tt = {}
    for _, v in ipairs(t) do
        tt[v] = require(v)
    end
    mod.run(tt)

    print(#successes + #failures, "checks", #failures, "failures")
    for _, v in ipairs(failures) do
        local err = table.remove(v)
        print(err)
        print(unpack(v))
        print()
    end
end

return mod

