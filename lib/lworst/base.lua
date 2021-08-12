
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

function can_call(a)
    return can(a, '__call') or type(a) == "function"
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
    if can_to_string_terse(a) then return a:to_string_terse() end
    return to_string_format(a, 'terse') or to_string_fallback(a)
end

function to_string_debug(a)
    if can_to_string_debug(a) then return a:to_string_debug() end
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

local Stack = Type.new("stack")
Stack.__tostring = function(s)
    local vals = {}
    for _, v in ipairs(s) do
        table.insert(vals, to_string_debug(v))
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
                .. "}")
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
                .. tostring(#inputs))
        end
        if itypes ~= true then
            contract_expect_types("input", itypes, inputs)
        end
        if otypes == true then
            return body(...)
        else
            local outputs = { body(inputs) }
            contract_expect_types("output", otypes, outputs)
            return unpack(outputs)
        end
    end
end

return {
    Error = Error,
    Symbol = Symbol,
    Type = Type,
    clone = clone,
    destroy = destroy,
    equal = equal,
    can_call = can_call,
    to_string_format = to_string_format,
    to_string_terse = to_string_terse,
    to_string_debug = to_string_debug,
    Stack = Stack,
    Place = Place,
    contract = contract,
}

