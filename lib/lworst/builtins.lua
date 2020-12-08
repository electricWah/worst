
local base = require("base")
local Symbol = base.Symbol
local Place = base.Place
local Clone = base.Clone
local Interpreter = require("interpreter")
local Port = require("port")
local List = require("list")
local Reader = require("reader")
local Map = require("map")
local Type = base.Type

local mod = {}

mod["quote"] = function(i)
    local v = i:body_read()
    if v == nil then
        i:error("quote-nothing")
    else
        i:stack_push(v)
    end
end

mod["uplevel"] = function(i)
    if not i:into_parent() then
        i:error("root-uplevel")
    else
        local v = i:stack_pop(Symbol)
        i:call(v)
    end
end

mod["eval"] = function(i)
    local v = i:stack_pop()
    i:eval(v)
end

mod["call"] = function(i)
    local v = i:stack_pop()
    i:call(v)
end

mod["drop"] = function(i)
    local v = i:stack_pop()
    base.destroy(v)
end

mod["equal?"] = function(i)
    local b = i:stack_ref(1)
    local a = i:stack_ref(2)
    i:stack_push(base.equal(a, b))
end

mod["clone"] = function(i)
    local v = i:stack_ref(1)
    i:stack_push(base.clone(v))
end

mod["swap"] = function(i)
    local a = i:stack_pop()
    local b = i:stack_pop()
    i:stack_push(a)
    i:stack_push(b)
end

mod["dig"] = function(i)
    local a = i:stack_pop()
    local b = i:stack_pop()
    local c = i:stack_pop()
    i:stack_push(b)
    i:stack_push(a)
    i:stack_push(c)
end

mod["bury"] = function(i)
    local a = i:stack_pop()
    local b = i:stack_pop()
    local c = i:stack_pop()
    i:stack_push(a)
    i:stack_push(c)
    i:stack_push(b)
end

mod["when"] = function(i)
    local name = i:stack_pop(Symbol)
    local whether = i:stack_pop("boolean")
    if whether then
        i:call(name)
    end
end

mod["and"] = function(i)
    local a = i:stack_ref(1)
    local b = i:stack_ref(2)
    i:stack_push(b and a)
end

mod["or"] = function(i)
    local a = i:stack_ref(1)
    local b = i:stack_ref(2)
    i:stack_push(b or a)
end

mod["number?"] = function(i)
    i:stack_push(type(i:stack_ref(1)) == "number")
end

mod["string?"] = function(i)
    i:stack_push(type(i:stack_ref(1)) == "string")
end

mod["bool?"] = function(i)
    i:stack_push(type(i:stack_ref(1)) == "boolean")
end

mod["false?"] = function(i)
    i:stack_push(not i:stack_ref(1))
end

mod["not"] = function(i)
    i:stack_push(not i:stack_pop())
end

mod["add"] = function(i)
    local a = i:stack_pop("number")
    local b = i:stack_pop("number")
    i:stack_push(a + b)
end

mod["mul"] = function(i)
    local a = i:stack_pop("number")
    local b = i:stack_pop("number")
    i:stack_push(a * b)
end

mod["negate"] = function(i)
    local a = i:stack_pop("number")
    i:stack_push(-a)
end

mod["recip"] = function(i)
    local a = i:stack_pop("number")
    i:stack_push(1 / a)
end

mod["ascending?"] = function(i)
    local a = i:stack_ref(1, "number")
    local b = i:stack_ref(2, "number")
    i:stack_push(a > b)
end

mod["list?"] = function(i)
    local v = i:stack_ref(1)
    i:stack_push(List.is(v))
end

mod["list-empty?"] = function(i)
    local l = i:stack_ref(1, List)
    local len = l:length()
    i:stack_push(len == 0)
end

mod["list-push"] = function(i)
    local v = i:stack_pop()
    local l = i:stack_pop(List)
    local newl = l:push(v)
    i:stack_push(newl)
end

mod["list-pop"] = function(i)
    local l = i:stack_pop(List)
    local newl, v = l:pop()
    if not newl then return i:error("list-empty") end
    i:stack_push(newl)
    i:stack_push(v)
end

mod["list-append"] = function(i)
    local b = i:stack_pop(List)
    local a = i:stack_pop(List)
    local l = List.append(a, b)
    i:stack_push(l)
end

mod["list-reverse"] = function(i)
    local l = i:stack_pop(List)
    local newl = l:reverse()
    i:stack_push(newl)
end

mod["list-length"] = function(i)
    local l = i:stack_ref(1, List)
    i:stack_push(l:length())
end

mod["list-ref"] = function(i)
    local n = i:stack_ref(1, "number")
    if n < 0 and n ~= math.floor(n) then i:error("nonnegative-integer", n) end
    local l = i:stack_ref(2, List)
    if n >= l:length() then i:error("out-of-range", n, l:length()) end

    i:stack_push(l[n])
end

mod["env-get"] = function(i)
    local name = i:stack_ref(1, "string")
    local value = os.getenv(name) or false
    i:stack_push(value)
end

mod["definition-add"] = function(i)
    local name = i:stack_pop(Symbol)
    local body = i:stack_pop({List, "function"})
    i:define(name, body)
end

mod["definition-get"] = function(i)
    local name = i:stack_ref(1, Symbol)
    local def = i:definition_get(name) or false
    i:stack_push(def)
end

mod["definition-remove"] = function(i)
    local name = i:stack_pop(Symbol)
    i:definition_remove(name)
end

mod["definition-resolve"] = function(i)
    local name = i:stack_ref(1, Symbol)
    local def = i:resolve(name) or false
    i:stack_push(def)
end

mod["string->symbol"] = function(i)
    local v = i:stack_pop("string")
    i:stack_push(Symbol.new(v))
end

mod["symbol->string"] = function(i)
    local v = i:stack_pop(Symbol)
    i:stack_push(Symbol.unwrap(v))
end

mod["symbol?"] = function(i)
    local v = i:stack_ref(1)
    i:stack_push(Symbol.is(v))
end

mod["to-string/terse"] = function(i)
    local v = i:stack_ref(1)
    i:stack_push(base.to_string_terse(v))
end

mod["to-string/debug"] = function(i)
    local v = i:stack_ref(1)
    i:stack_push(base.to_string_debug(v))
end

mod["interpreter-dump-stack"] = function(i)
    print(unpack(i.stack))
end

mod["interpreter-call-stack"] = function(i)
    i:stack_push(i:call_stack())
end

mod["interpreter-stack"] = function(i)
    i:stack_push(i:stack_get())
end

mod["interpreter-stack-set"] = function(i)
    local new = i:stack_pop(List)
    i:stack_set(new)
end

mod["interpreter-stack-length"] = function(i)
    i:stack_push(i:stack_length())
end

mod["string-append"] = function(i)
    local b = i:stack_pop("string")
    local a = i:stack_pop("string")
    i:stack_push(a .. b)
end

mod["string-join"] = function(i)
    local sep = i:stack_pop("string")
    local strs = i:stack_pop(List)
    local t = strs:to_table()
    for _, v in ipairs(t) do
        if not Type.is("string", v) then
            i:error("wrong-type", "list of strings", strs)
        end
    end
    i:stack_push(table.concat(t, sep))
end

mod["string-global-matches"] = function(i)
    local pat = i:stack_pop("string")
    local str = i:stack_pop("string")
    local t = {}
    for c in string.gmatch(str, pat) do
        table.insert(t, c)
    end
    i:stack_push(List.create(t))
end

mod["string-ref"] = function(i)
    local n = i:stack_ref(1, "number")
    local v = i:stack_ref(2, "string")
    if n >= 0 and n == math.floor(n) and n < string.len(v) then
        i:stack_push(string.sub(v, n + 1, n + 1))
    else
        i:stack_push(false)
    end

end

mod["current-input-port"] = function(i)
    i:stack_push(Port.stdin())
end

mod["current-output-port"] = function(i)
    i:stack_push(Port.stdout())
end

mod["current-error-port"] = function(i)
    i:stack_push(Port.stderr())
end

mod["open-input-file"] = function(i)
    local path = i:stack_pop("string")
    local f, err = Port.open_input_file(path)
    if f then
        i:stack_push(f)
    else
        i:stack_push(err)
        i:stack_push(false)
    end
end

mod["open-output-file"] = function(i)
    local path = i:stack_pop("string")
    local f, err = Port.open_output_file(path)
    if f then
        i:stack_push(f)
    else
        i:stack_push(err)
        i:stack_push(false)
    end
end

mod["open-input-string"] = function(i)
    local s = i:stack_pop("string")
    i:stack_push(Port.InputPort.string(s))
end

mod["port-read-value"] = function(i)
    local port = i:stack_ref(1, Port.InputPort)
    local v = Reader.read_next(port)
    if v == nil then
        i:stack_push(Port.EOF)
    else
        i:stack_push(v)
    end
end

mod["port-has-data"] = function(i)
    local port = i:stack_ref(1, Port.InputPort)
    i:stack_push(port:buffer_size() > 0)
end

mod["port-peek-char"] = function(i)
    local port = i:stack_ref(1, Port.InputPort)
    i:stack_push(port:peek())
end

mod["port-read-char"] = function(i)
    local port = i:stack_ref(1, Port.InputPort)
    i:stack_push(port:take(1))
end

mod["port-read-line"] = function(i)
    local port = i:stack_ref(1, Port.InputPort)
    local s = port:match("[^\n]+\n?")
    if s == nil then
        i:stack_push(false)
    else
        port:take(string.len(s))
        i:stack_push(s)
    end
end

mod["port-write-string"] = function(i)
    local v = i:stack_pop(String)
    local port = i:stack_ref(1, Port.OutputPort)
    port:write_string(v)
end

mod["port-close"] = function(i)
    local port = i:stack_pop({ Port.InputPort, Port.OutputPort })
    port:close()
end

mod["eof-object?"] = function(i)
    local v = i:stack_ref(1)
    i:stack_push(Port.Eof.is(v))
end

mod["place?"] = function(i)
    local v = i:stack_ref(1)
    i:stack_push(Place.is(v))
end

mod["make-place"] = function(i)
    local v = i:stack_pop()
    local p = Place.new(v)
    i:stack_push(p)
end

mod["place-get"] = function(i)
    local p = i:stack_ref(1, Place)
    local v = p:get()
    i:stack_push(v)
end

mod["place-set"] = function(i)
    local v = i:stack_pop()
    local p = i:stack_ref(1, Place)
    p:set(v)
end

mod["map?"] = function(i)
    local m = i:stack_ref(1)
    i:stack_push(Map.is(m))
end

mod["map-empty"] = function(i)
    i:stack_push(Map.empty())
end

mod["map-exists"] = function(i)
    local k = i:stack_ref(1)
    local m = i:stack_ref(2, Map)
    local v = m:has_key(k)
    i:stack_push(v)
end

mod["map-get"] = function(i)
    local k = i:stack_ref(1)
    local m = i:stack_ref(2, Map)
    i:stack_push(m:get(k) or false)
end

mod["map-set"] = function(i)
    local v = i:stack_pop()
    local k = i:stack_pop()
    local m = i:stack_pop(Map)
    local r = Map.set(m, k, v)
    i:stack_push(r)
end

mod["map-remove"] = function(i)
    local k = i:stack_pop()
    local m = i:stack_pop(Map)
    local r = Map.remove(m, k)
    i:stack_push(r)
end

mod["map-keys"] = function(i)
    local m = i:stack_ref(1, Map)
    i:stack_push(m:keys())
end

mod["current-context-set-code"] = function(i)
    local body = i:stack_pop(List)
    i:set_body(body)
end

mod["current-context-clear"] = function(i)
    i:set_body(List.empty())
end

mod["current-context-definitions"] = function(i)
    local m = Map.empty()
    for k, v in pairs(i:definitions()) do
        m = Map.set(m, k, base.clone(v))
    end
    i:stack_push(m)
end

-- string lua-load-string -> function #t
--                        -> error    #f
mod["lua-load-string"] = function(i)
    local src = i:stack_pop("string")
    local r, err = load(src)
    if r then
        i:stack_push(r)
        i:stack_push(true)
    else
        i:stack_push(err)
        i:stack_push(false)
    end
end

mod[Interpreter.ERROR_HANDLER] = function(i)
    local v = i:stack_pop(Symbol)
    local irritants = i:stack_pop(List)
    print("error:", v, unpack(irritants:to_table()))
    i:reset()
end

return mod

