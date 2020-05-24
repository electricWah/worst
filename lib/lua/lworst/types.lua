
local Type = {}
Type.__index = Type
Type.__tostring = function(t)
    return "Type(" .. t.name .. ")"
end

Type.is = function(v) return getmetatable(v) == Type end

function Type.new(name)
    local t = setmetatable({ name = name }, Type)
    t.__index = t
    t.is = function(v) return getmetatable(v) == t end
    return t
end

function Type.any(...)
    local types = {...}
    local names = {}
    for _, t in ipairs(types) do
        table.insert(names, t.name)
    end
    return setmetatable({
        name = table.concat(names, "|"),
        is = function(v)
            for _, t in ipairs(types) do
                if Type.is(t) then
                    if t.is(v) then
                        return true
                    end
                elseif type(t) == "string" then
                    if type(v) == t then
                        return true
                    end
                end
            end
            return false
        end
    }, Type)
end

-- Type.any(t1, t2, ...) -> T with T.is(x) => x has type t1 or t2 or ...

local Trait = {}
Trait.__index = Trait
function Trait.new()
    return setmetatable({}, Trait)
end

function Trait:install_method(ty, name, m)
    if ty[self] then
        ty[self][name] = m
    else
        ty[self] = setmetatable({ [name] = m }, self)
    end
end

function Trait:can(v, method)
    local ty = getmetatable(v)
    if ty then
        local methods = getmetatable(ty[self])
        if methods == self then
            if method ~= nil then
                return methods[method] ~= nil
            else
                return true
            end
        end
    end
    return false
end

function Trait:call(m, v, ...)
    local mt = getmetatable(v)
    if not mt then error("Trait:call " .. m) end
    local f = mt[self][m]
    if not f then error("Trait:call " .. m) end
    return f(v, ...)
end

local Clone = Trait.new()
function Clone.clone_for(t, clone)
    Clone:install_method(t, "clone", clone)
end
function Clone.clone(v) return Clone:call("clone", v) end


local Drop = Trait.new()
function Drop.drop_for(t, drop)
    Drop:install_method(t, "drop", drop)
end
function Drop.drop(v) Drop:call("drop", v) end

local ToString = Trait.new()
function ToString.terse_for(t, f)
    ToString:install_method(t, "terse", f)
end
function ToString.debug_for(t, f)
    ToString:install_method(t, "debug", f)
end

function tostring_default(name, v, ...)
    if ToString:can(v, name) then
        return ToString:call(name, v, ...)
    elseif type(v) == "string" then
        return string.format("%q", v)
    else
        return tostring(v)
    end
end

function ToString.terse(v, ...)
    return tostring_default("terse", v, ...)
end

function ToString.debug(v, ...)
    return tostring_default("debug", v, ...)
end

local Equal = Trait.new()
function Equal.equal_for(t, equal)
    Equal:install_method(t, "equal", equal)
end

function Equal.equal(a, b)
    if Equal:can(a) then
        return Equal:call("equal", a, b)
    elseif Equal:can(b) then
        return Equal:call("equal", b, a)
    else
        return a == b
    end
end

return {
    Type = Type,
    Clone = Clone,
    Drop = Drop,
    Equal = Equal,
    ToString = ToString,
}


