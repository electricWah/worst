
local Type = {}

local meta = setmetatable({}, {
    __tostring = function() return "meta" end
})

function Type.new(name)
    local ty = {name=name}
    ty.__index = ty
    function ty.is(v) return getmetatable(v) == ty end
    return setmetatable(ty, Type)
end
function Type:__tostring() return "<type " .. self.name .. ">" end

local lua_types = {}
function new_lua_type(ty)
    local t = Type.new(ty)
    t.lua_type = ty
    function t.new(v)
        if getmetatable(v) == t then
            return v
        elseif type(v) ~= ty then
            return error("not a " .. ty .. ": " .. tostring(v) .. ", " .. tostring(getmetatable(v)))
        else
            return setmetatable({
                value = v
            }, t)
        end
    end
    function t.__eq(a, b) return a.value == b.value end
    function t:clone() return t.new(self.value) end
    function t:unwrap() return self.value end
    lua_types[ty] = t
    return t
end

local String = new_lua_type("string")
function String.__tostring(v) return string.format("%q", v.value) end
local Boolean = new_lua_type("boolean")
function Boolean.__tostring(v)
    if v.value then return "#t" else return "#f" end
end
local Number = new_lua_type("number")
function Number.__tostring(v) return tostring(v.value) end
local Function = new_lua_type("function")
function Function.__tostring(v)
    return "<" .. tostring(meta.get(v, "name") or v.value) .. ">"
end
function Function:__call(...)
    return self.value(...)
end

function value(v)
    if v == nil then return nil end
    if getmetatable(getmetatable(v)) == Type then
        return v
    end
    local wrapper = lua_types[type(v)]
    if wrapper then
        return wrapper.new(v)
    else
        error("cannot use value " .. tostring(v) .. debug.traceback(""))
    end
end

function clone(v)
    if getmetatable(getmetatable(v)) == Type and type(v.clone) == "function" then
        local c = v:clone()
        c[meta] = v[meta]
        return c
    elseif type(v) ~= "table" then
        return v
    else
        error("cannot clone " .. type(v) .. " " .. tostring(v))
    end
end

function unwrap_lua(v)
    local mt = getmetatable(v)
    if mt and lua_types[mt.lua_type] then
        return v.value
    else
        return v
    end
end

function is_a(v, ...)
    if v == nil then return false end
    local types = {...}
    local luaty = type(v)
    local mt = getmetatable(v)
    for _, ty in ipairs(types) do
        if mt == ty
            or luaty == ty
            or (mt and mt.lua_type and mt.lua_type == ty)
        then
            return true
        elseif type(ty) == "table" and #ty > 0 and is_a(v, unpack(ty)) then
            return true
        end
    end
    return false
end

local Meta = Type.new("meta")
function Meta:__tostring()
    local t = {}
    for k, v in pairs(self) do
        table.insert(t, tostring(k) .. ": " .. tostring(v))
    end
    return "{" .. table.concat(t, ", ") .. "}"
end
function Meta_clone(orig)
    local c = setmetatable({}, Meta)
    for k, v in pairs(orig or {}) do
        c[k] = v
    end
    return c
end

function meta.set_all(v, p)
    local u = value(clone(v))
    u[meta] = Meta_clone(u[meta])
    for mk, mv in pairs(p) do
        mk = unwrap_lua(mk)
        u[meta][mk] = mv
    end
    return u
end

function meta.set(v, mk, mv)
    return meta.set_all(v, {[mk]=mv})
end

function meta.get(v, mk)
    mk = unwrap_lua(mk)
    local m = value(v)[meta]
    if m and mk then
        return m[mk]
    else
        return m
    end
end

local Symbol = Type.new("symbol")
local SymbolCache = setmetatable({}, { __mode = "kv" })
function Symbol.new(v)
    if SymbolCache[v] then return SymbolCache[v] end
    if Symbol.is(v) then return v end
    if type(v) ~= "string" then error("Symbol.new: not a string: " .. tostring(v)) end
    local s = setmetatable({v = v}, Symbol)
    SymbolCache[v] = s
    return s
end
Symbol.__tostring = function(s) return s.v end
function Symbol.unwrap(s) return s.v end

local Error = Type.new("error")
function Error.new(message, irritants)
    return setmetatable({
        message = message,
        irritants = irritants,
        lua_stack = debug.traceback("", 2)
    }, Error)
end
Error.__tostring = function(e)
    return "error: " ..
        tostring(e.message) .. " " .. tostring(e.irritants) .. ": " .. e.lua_stack
end
function Error:to_list()
    return self.irritants:push(self.message)
end

local Place = Type.new("place")
Place.__tostring = function(p) return "Place(" .. tostring(p.v) .. ")" end
function Place.new(v) return setmetatable({ v = v }, Place) end
function Place:get() return self.v end
function Place:set(v) self.v = v end

return {
    Type = Type,
    value = value,
    clone = clone,
    unwrap_lua = unwrap_lua,
    is_a = is_a,
    meta = meta,
    String = String,
    Boolean = Boolean,
    Number = Number,
    Function = Function,
    Error = Error,
    Symbol = Symbol,
    Place = Place,
}

