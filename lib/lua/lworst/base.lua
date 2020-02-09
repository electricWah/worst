
local Symbol = require("symbol")
local types = require("types")
local Type = types.Type
local ToString = types.ToString
local Drop = types.Drop
local Clone = types.Clone
local Equal = types.Equal

local Char = Type.new("char")
function Char.of_str(s)
    return setmetatable({s = s}, Char)
end
function Char.of_int(v)
    return Char.of_str(string.char(v))
end

Equal.equal_for(Char, function(a, b)
    return Char.is(a) and Char.is(b) and a.s == b.s
end)

ToString.terse_for(Char, function(c) return "#\\" .. c.s end)
ToString.debug_for(Char, function(c) return "Char(" .. c.s .. ")" end)

Char.__tostring = function(s) return ToString.terse(s) end

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
        .. tostring(t) .. "[" .. tostring(k) .. "] = " .. tostring(v))
end

function readonly(t)
    local mt = getmetatable(t) 
    if mt == Readonly then return t end
    if mt ~= nil then
        error("readonly: already has metatable: " .. ToString.debug(t))
    end
    return setmetatable(t, Readonly)
end

return {
    Error = Error,
    Symbol = Symbol,
    Char = Char,
    Type = Type,
    Clone = Clone,
    Drop = Drop,
    Equal = Equal,
    ToString = ToString,
    Stack = Stack,
    Place = Place,
    readonly = readonly,
    Readonly = Readonly,
}

