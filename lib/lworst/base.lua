
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
        -- is_lua = (getmetatable(v) == nil and type(type(v)) == "string")
    }, Value)
end

-- remove the Value wrapper around v if there is one
function Value:unwrap(v)
    if getmetatable(self) == Value then
        return self.value
    else
        return self
    end
end

function Value:clone()
    local val = Value.unwrap(self)
    local r = setmetatable({}, Value)
    for k, v in pairs(val) do
        r[k] = v
    end
    return r
end

-- shallow copy self with a new inner value
function Value:update(new)
    local r = Value.clone(self)
    r.value = Value.unwrap(new)
    return r
end

-- function Value:update_with(f, ...)
--     return Value.update(self, f(self.value, ...))
-- end

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
    if Symbol.is(v) then return v end
    if type(v) ~= "string" then error("Symbol.new: not a string: " .. v) end
    local s = setmetatable({v = v}, Symbol)
    SymbolCache[v] = s
    return s
end
function Symbol.to_string_terse(s) return s.v end
function Symbol.to_string_debug(s) return "Symbol(" .. s.v .. ")" end
Symbol.__tostring = function(s) return Symbol.to_string_debug(s) end
function Symbol.unwrap(v) return v.v end


function can_impl(v, f) return (getmetatable(v) or {})[f] ~= nil end
local can = setmetatable({}, {
    __index = function(t, k)
        if not rawget(t, k) then
            t[k] = function(v) return can_impl(k, v) end
        end
        return rawget(t, k)
    end,
})

function clone(a)
    if can.clone(a) then
        return a:clone()
    else
        return a
    end
end

can.call = function(a)
    return can_impl(a, '__call')
        or type(a) == "function"
        or can_impl(a, 'call')
end

function destroy(a) if can.destroy(a) then a:destroy() end end

function to_string_format(a, fmt)
    if can.to_string_format(a) then
        return a:to_string_format(fmt) or false
    end
    return false
end

local tostring_types = {}
tostring_types["string"] = function(a)
    return string.format("%q", a)
end
tostring_types["boolean"] = function(a)
    if a then return "#t" else return "#f" end
end

function to_string_fallback(a)
    local t = tostring_types[type(a)]
    if t then return t(a) end
    return tostring(a)
end

function to_string_terse(a)
    if can.to_string_terse(a) then return a:to_string_terse() end
    return to_string_format(a, 'terse') or to_string_fallback(a)
end

function to_string_debug(a)
    if can.to_string_debug(a) then return a:to_string_debug() end
    return to_string_format(a, 'debug') or to_string_fallback(a)
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
    return "<error " ..
        to_string_terse(e.message) .. to_string_terse(e.irritants)
        .. ">"
end
function Error:to_list()
    return self.irritants:push(self.message)
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

function concat_with(t, f, sep)
    local a = {}
    for _, v in ipairs(t) do
        table.insert(a, f(v))
    end
    return table.concat(a, sep)
end

function contract_expect_types(context, types, values)
    for i, v in ipairs(values) do
        local t = types[i]
        if t ~= true and not Type.is(t, v) then
            error(context .. " type mismatch: expected {"
                .. concat_with(types, Type.name, ", ")
                .. "} but got {"
                .. concat_with(values, to_string_debug, ", ")
                .. "}", 3)
        end
    end
end

function contract(itypes, otypes, body)
    return function(...)
        local inputs = { ... }
        if #inputs ~= #itypes then
            error("input argument mismatch: expected "
                .. tostring(#itypes)
                .. " but got "
                .. tostring(#inputs), 3)
        end
        if itypes ~= true then
            contract_expect_types("input", itypes, inputs)
        end
        if otypes == true then
            return body(...)
        else
            local outputs = { body(unpack(inputs)) }
            contract_expect_types("output", otypes, outputs)
            return unpack(outputs)
        end
    end
end

return {
    Value = Value,
    value = value,
    Error = Error,
    Symbol = Symbol,
    Type = Type,
    clone = clone,
    destroy = destroy,
    can = can,
    to_string_format = to_string_format,
    to_string_terse = to_string_terse,
    to_string_debug = to_string_debug,
    Place = Place,
    contract = contract,
}

