
local types = require("types")
local Type = types.Type
local ToString = types.ToString
local Drop = types.Drop
local Clone = types.Clone

local SymbolCache = setmetatable({}, { __mode = "kv" })

local Symbol = Type.new("symbol")
function Symbol.new(v)
    if SymbolCache[v] then return SymbolCache[v] end
    local s = setmetatable({v = v}, Symbol)
    SymbolCache[v] = s
    return s
end

ToString.terse_for(Symbol, function(s) return s.v end)
ToString.debug_for(Symbol, function(s) return "Symbol(" .. s.v .. ")" end)

Symbol.__tostring = function(s) return ToString.terse(s) end

function Symbol.unwrap(v) return v.v end

return Symbol

