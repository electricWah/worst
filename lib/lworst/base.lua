
local Value = {}
Value.__index = Value

function Value:__tostring()
    return "<value " .. tostring(self.value) .. ">"
end

-- create a value
function value(v)
    if v == nil or getmetatable(v) == Value then return v end
    return setmetatable({
        value = v,
    }, Value)
end

-- remove the Value wrapper around v if there is one
function Value:unwrap()
    if getmetatable(self) == Value then
        return self.value
    else
        return self
    end
end

local Type = {}
Type.__index = Type
Type.__tostring = function(t) return t.name end

function Type.new(name)
    local t = setmetatable({ name = name }, Type)
    t.__index = t
    t.is = function(v) return getmetatable(v) == t end
    return t
end

function Type.is(t, v)
    if t == nil then error("Type.is(nil, " .. tostring(v) .. ")") end
    if type(v) == t
    or getmetatable(v) == t
    or (type(t) == "table" and type(t.is) == "function" and t.is(v))
    or (type(t) == "function" and t(v))
    or (type(t) ~= "string" and t == v)
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
    if type(v) ~= "string" then error("Symbol.new: not a string: " .. v) end
    local s = setmetatable({v = v}, Symbol)
    SymbolCache[v] = s
    return s
end
function Symbol.write_string(s) return s.v end
Symbol.__tostring = function(s) return s.v end
function Symbol.unwrap(s) return s.v end


function can_impl(v, f) return (getmetatable(v) or {})[f] ~= nil end
local can = setmetatable({}, {
    __index = function(t, k)
        if not rawget(t, k) then
            t[k] = function(v) return can_impl(k, v) end
        end
        return rawget(t, k)
    end,
})

can.call = function(a)
    return can_impl(a, '__call')
        or type(a) == "function"
        or can_impl(a, 'call')
end

function write_string(v)
    local t = type(v)
    if t == "string" then
        return string.format("%q", v)
    elseif t == "boolean" then
        if v then return "#t" else return "#f" end
    elseif can.write_string(v) then
        return v:write_string()
    else
        return tostring(v)
    end
end

local Error = Type.new("error")
function Error.new(message, irritants)
    return setmetatable({
        message = message,
        irritants = irritants,
        lua_stack = debug.traceback("", 2)
    }, Error)
end
Error.__tostring = function(e)
    return "<error " .. write_string(e.message) .. write_string(e.irritants) .. ">"
end
function Error:to_list()
    return self.irritants:push(self.message)
end

local Place = Type.new("place")
Place.__tostring = function(p)
    return "Place(" .. write_string(p.v) .. ")"
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

return {
    Value = Value,
    value = value,
    Error = Error,
    Symbol = Symbol,
    Type = Type,
    can = can,
    write_string = write_string,
    Place = Place,
}

