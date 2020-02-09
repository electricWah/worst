
local base = require("base")
local Symbol = base.Symbol
local Char = base.Char
local Place = base.Place
local Clone = base.Clone
local Interpreter = require("interpreter")
local Port = require("port")
local List = require("list")
local Reader = require("reader")
local Map = require("map")
local Type = base.Type
local Drop = base.Drop
local Clone = base.Clone
local Equal = base.Equal
local ToString = base.ToString

local mod = {}

mod["quote"] = function(i, s)
    local v = i:body_read()
    if v == nil then
        i:error("quote-nothing")
    else
        i:stack_push(s, v)
    end
end

mod["uplevel"] = function(i, s)
    if not i:into_parent() then
        i:error("root-uplevel")
    else
        local v = i:stack_pop(s, Symbol)
        i:call(s, v)
    end
end

mod["eval"] = function(i, s)
    local v = i:stack_pop(s)
    i:eval(s, v)
end

mod["call"] = function(i, s)
    local v = i:stack_pop(s)
    i:call(s, v)
end

mod["drop"] = function(i, s)
    local v = i:stack_pop(s)
    if Drop:can(v) then
        Drop.drop(v)
    end
end

mod["equal?"] = function(i, s)
    local b = i:stack_ref(s, 1)
    local a = i:stack_ref(s, 2)
    if Equal:can(a) then
        i:stack_push(s, Equal.equal(a, b))
    else
        i:stack_push(s, a == b)
    end
end


mod["clone"] = function(i, s)
    local v = i:stack_ref(s, 1)
    if Clone:can(v) then
        i:stack_push(s, Clone.clone(v))
    else
        i:stack_push(s, v)
    end
end

mod["swap"] = function(i, s)
    local a = i:stack_pop(s)
    local b = i:stack_pop(s)
    i:stack_push(s, a)
    i:stack_push(s, b)
end

mod["dig"] = function(i, s)
    local a = i:stack_pop(s)
    local b = i:stack_pop(s)
    local c = i:stack_pop(s)
    i:stack_push(s, b)
    i:stack_push(s, a)
    i:stack_push(s, c)
end

mod["bury"] = function(i, s)
    local a = i:stack_pop(s)
    local b = i:stack_pop(s)
    local c = i:stack_pop(s)
    i:stack_push(s, a)
    i:stack_push(s, c)
    i:stack_push(s, b)
end

mod["when"] = function(i, s)
    local name = i:stack_pop(s, Symbol)
    local whether = i:stack_pop(s, "boolean")
    if whether then
        i:call(s, name)
    end
end

mod["or"] = function(i, s)
    local a = i:stack_ref(s, 1)
    local b = i:stack_ref(s, 2)
    i:stack_push(s, b or a)
end

mod["false?"] = function(i, s)
    i:stack_push(s, not i:stack_ref(s, 1))
end

mod["not"] = function(i, s)
    i:stack_push(s, not i:stack_pop(s))
end

mod["add"] = function(i, s)
    local a = i:stack_pop(s, "number")
    local b = i:stack_pop(s, "number")
    i:stack_push(s, a + b)
end

mod["list-empty?"] = function(i, s)
    local l = i:stack_ref(s, 1, List)
    local len = l:length()
    i:stack_push(s, len == 0)
end

mod["list-push"] = function(i, s)
    local v = i:stack_pop(s)
    local l = i:stack_pop(s, List)
    local newl = l:push(v)
    i:stack_push(s, newl)
end

mod["list-pop"] = function(i, s)
    local l = i:stack_pop(s, List)
    local newl, v = l:pop()
    if not newl then return i:error("list-empty") end
    i:stack_push(s, newl)
    i:stack_push(s, v)
end

mod["list-append"] = function(i, s)
    local b = i:stack_pop(s, List)
    local a = i:stack_pop(s, List)
    local l = List.append(a, b)
    i:stack_push(s, l)
end

mod["list-reverse"] = function(i, s)
    local l = i:stack_pop(s, List)
    local newl = l:reverse()
    i:stack_push(s, newl)
end

mod["list-length"] = function(i, s)
    local l = i:stack_ref(s, 1, List)
    i:stack_push(s, l:length())
end

mod["env-get"] = function(i, s)
    local name = i:stack_ref(s, 1, "string")
    local value = os.getenv(name) or false
    i:stack_push(s, value)
end

local tdef = Type.any(List, "function")
mod["definition-add"] = function(i, s)
    local name = i:stack_pop(s, Symbol)
    local body = i:stack_pop(s, tdef)
    i:define(name, body)
end

mod["definition-get"] = function(i, s)
    local name = i:stack_ref(s, 1, Symbol)
    local def = i:definition_get(name) or false
    i:stack_push(s, def)
end

mod["definition-remove"] = function(i, s)
    local name = i:stack_pop(s, Symbol)
    i:definition_remove(name)
end

mod["definition-resolve"] = function(i, s)
    local name = i:stack_ref(s, 1, Symbol)
    local def = i:resolve(name) or false
    i:stack_push(s, def)
end

mod["symbol?"] = function(i, s)
    local v = i:stack_ref(s, 1)
    i:stack_push(s, Symbol.is(v))
end

mod["to-string/terse"] = function(i, s)
    local v = i:stack_ref(s, 1)
    i:stack_push(s, ToString.terse(v))
end

mod["to-string/debug"] = function(i, s)
    local v = i:stack_ref(s, 1)
    i:stack_push(s, ToString.debug(v))
end

mod["interpreter-dump-stack"] = function(i, s)
    print(unpack(s))
end

mod["interpreter-stack"] = function(i, s)
    local l = List.empty()
    for _, v in ipairs(s) do
        l = l:push(v)
    end
    i:stack_push(s, l)
end

mod["interpreter-stack-set"] = function(i, s)
    local new = i:stack_pop(s, List):to_table()
    while s:pop() ~= nil do end
    for _, v in ipairs(new) do
        s:push(v)
    end
end

mod["string-append"] = function(i, s)
    local b = i:stack_pop(s, "string")
    local a = i:stack_pop(s, "string")
    i:stack_push(s, a .. b)
end

mod["current-input-port"] = function(i, s)
    i:stack_push(s, Port.stdin())
end

mod["current-output-port"] = function(i, s)
    i:stack_push(s, Port.stdout())
end

mod["current-error-port"] = function(i, s)
    i:stack_push(s, Port.stderr())
end

mod["open-input-file"] = function(i, s)
    local path = i:stack_pop(s, "string")
    local f = Port.open_input_file(path)
    i:stack_push(s, f)
end

mod["port-read-value"] = function(i, s)
    local port = i:stack_ref(s, 1, Port.InputPort)
    local v = Reader.read_next(port)
    if v == nil then
        i:stack_push(s, Port.EOF)
    else
        i:stack_push(s, v)
    end
end

mod["port-has-data"] = function(i, s)
    local port = i:stack_ref(s, 1, Port.InputPort)
    i:stack_push(s, port:buffer_size() > 0)
end

mod["port-peek-char"] = function(i, s)
    local port = i:stack_ref(s, 1, Port.InputPort)
    i:stack_push(s, Char.of_str(port:peek()))
end

mod["port-read-char"] = function(i, s)
    local port = i:stack_ref(s, 1, Port.InputPort)
    i:stack_push(s, Char.of_str(port:take(1)))
end

mod["port-write-string"] = function(i, s)
    local v = i:stack_pop(s, String)
    local port = i:stack_ref(s, 1, Port.OutputPort)
    port:write_string(v)
end

mod["eof-object?"] = function(i, s)
    local v = i:stack_ref(s, 1)
    i:stack_push(s, Port.Eof.is(v))
end

mod["make-place"] = function(i, s)
    local v = i:stack_pop(s)
    local p = Place.new(v)
    i:stack_push(s, p)
end

mod["place-get"] = function(i, s)
    local p = i:stack_ref(s, 1, Place)
    local v = p:get()
    i:stack_push(s, v)
end

mod["place-set"] = function(i, s)
    local v = i:stack_pop(s)
    local p = i:stack_ref(s, 1, Place)
    p:set(v)
end

mod["map-empty"] = function(i, s)
    i:stack_push(s, Map.empty())
end

mod["map-exists"] = function(i, s)
    local k = i:stack_ref(s, 1)
    local m = i:stack_ref(s, 2, Map)
    local v = m:has_key(k)
    i:stack_push(s, v)
end

mod["map-set"] = function(i, s)
    local v = i:stack_pop(s)
    local k = i:stack_pop(s)
    local m = i:stack_ref(s, 1, Map)
    m:set(k, v)
end

mod["map-get"] = function(i, s)
    local k = i:stack_ref(s, 1)
    local m = i:stack_ref(s, 2, Map)
    i:stack_push(s, m:get(k) or false)
end

mod["map-keys"] = function(i, s)
    local m = i:stack_ref(s, 1, Map)
    i:stack_push(s, m:keys())
end

mod["current-context-set-code"] = function(i, s)
    local body = i:stack_pop(s, List)
    i:set_body(body)
end

mod["current-context-clear"] = function(i, s)
    i:set_body(List.empty())
end

mod[Interpreter.ERROR_HANDLER] = function(i, s)
    local v = i:stack_pop(s, Symbol)
    local irritants = i:stack_pop(s, List)
    print("error:", v, unpack(irritants:to_table()))
    i:reset()
end

return mod

