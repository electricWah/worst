
local Type = {}
Type.__index = Type
Type.__tostring = function(t)
    return "Type(" .. t.name .. ")"
end

function Type.new(name)
    local t = setmetatable({ name = name }, Type)
    t.__index = t
    t.is = function(v) return getmetatable(v) == t end
    return t
end

function Type.is(t, v)
    if type(v) == t
    or getmetatable(v) == t
    or (type(t) == "table" and type(t.is) == "function" and t.is(v))
    or (type(t) == "function" and t(v))
    then return true end

    if type(t) == "table" then
        for _, t in ipairs(t) do
            if Type.is(t, v) then return true end
        end
    end

    return false
end

function Type.name(t)
    if type(t) == "string" then return t end

    if type(t) == "table" then
        local names = {}
        for _, t in ipairs(t) do
            table.insert(names, Type.name(t))
        end
        if #names > 0 then
            return table.concat(names, " or ")
        end
    end

    return tostring(t)
end

local Symbol = Type.new("symbol")
local SymbolCache = setmetatable({}, { __mode = "kv" })
function Symbol.new(v)
    if SymbolCache[v] then return SymbolCache[v] end
    local s = setmetatable({v = v}, Symbol)
    SymbolCache[v] = s
    return s
end
function Symbol.to_string_terse(s) return s.v end
function Symbol.to_string_debug(s) return "Symbol(" .. s.v .. ")" end
Symbol.__tostring = function(s) return Symbol.to_string_debug(s) end
function Symbol.unwrap(v) return v.v end

function can(v, f) return (getmetatable(v) or {})[f] ~= nil end
function can_equal(a) return can(a, 'equal') end
function equal(a, b)
    if can_equal(a) then
        return a:equal(b)
    elseif can_equal(b) then
        return b:equal(a)
    else
        return a == b
    end
end

function can_clone(a) return can(a, 'clone') end
function clone(a)
    if can_clone(a) then
        return a:clone()
    else
        return a
    end
end

function can_destroy(a) return can(a, 'destroy') end
function destroy(a) if can_destroy(a) then a:destroy() end end

function can_to_string_format(a) return can(a, 'to_string_format') end
function can_to_string_terse(a) return can(a, 'to_string_terse') end
function can_to_string_debug(a) return can(a, 'to_string_debug') end

function to_string_format(a, fmt)
    if can_to_string_format(a) then
        return a:to_string_format(fmt) or false
    end
    return false
end

function to_string_terse(a)
    local r = nil
    if can_to_string_terse(a) then r = a:to_string_terse() end
    if not r then r = to_string_format(a, 'terse') end
    if not r then r = tostring(a) end
    return r or false
end

function to_string_debug(a)
    local r = nil
    if can_to_string_debug(a) then r = a:to_string_debug() end
    if not r then r = to_string_format(a, 'debug') end
    if not r and type(a) == "string" then r = string.format("%q", a) end
    if not r then r = tostring(a) end
    return r or false
end

local Char = Type.new("char")
function Char.of_str(s)
    return setmetatable({s = s}, Char)
end
function Char.of_int(v)
    return Char.of_str(string.char(v))
end

function Char.equal(a, b)
    return Char.is(a) and Char.is(b) and a.s == b.s
end

function Char.to_string_terse(c) return "#\\" .. c.s end
function Char.to_string_debug(c) return "Char(" .. c.s .. ")" end

Char.__tostring = function(s) return Char.to_string_terse(s) end

function Char.unwrap(v) return v.v end

local Error = {}
Error.__index = Error
Error.__tostring = function(e)
    local irritants = {}
    for _, v in ipairs(e) do
        table.insert(irritants, tostring(v))
    end
    return "Error(" .. table.concat(irritants, " ") .. ")"
end

setmetatable(Error, {
    __call = function(c, v)
        return setmetatable(v, c)
    end
})

function Error.raise(name, ...)
    error(Error({name, ...}), 0)
end

local Stack = Type.new("stack")
Stack.__tostring = function(s)
    local vals = {}
    for _, v in ipairs(s) do
        table.insert(vals, tostring(v))
    end
    return "Stack(" .. table.concat(vals, " ") .. ")"
end

function Stack.empty()
    return setmetatable({}, Stack)
end

function Stack:push(v)
    table.insert(self, v)
end

function Stack:pop()
    return table.remove(self)
end

function Stack:length()
    return #self
end

local Place = Type.new("place")
Place.__tostring = function(p)
    return "Place(" .. tostring(p.v) .. ")"
end

function Place.new(v)
    return setmetatable({ v = v }, Place)
end

function Place:get()
    return self.v
end

function Place:set(v)
    self.v = v
end

local Readonly = Type.new("readonly")
function Readonly.__newindex(t, k, v)
    error("attempt to assign to readonly table: "
        .. tostring(t) .. "[" .. tostring(k) .. "] = " .. to_string_debug(v))
end

function readonly(t)
    local mt = getmetatable(t) 
    if mt == Readonly then return t end
    if mt ~= nil then
        error("readonly: already has metatable: " .. to_string_debug(t))
    end
    return setmetatable(t, Readonly)
end

return {
    Error = Error,
    Symbol = Symbol,
    Char = Char,
    Type = Type,
    clone = clone,
    destroy = destroy,
    equal = equal,
    to_string_format = to_string_format,
    to_string_terse = to_string_terse,
    to_string_debug = to_string_debug,
    Stack = Stack,
    Place = Place,
    readonly = readonly,
    Readonly = Readonly,
}

