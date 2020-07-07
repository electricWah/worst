
local base = require("base")
local List = require("list")
local Type = base.Type

-- TODO don't metatable anything here - ["get" map-get] returns a function!

local Map = Type.new("Map")
function Map:__tostring()
    return "Map(" .. self:count() .. ")"
end

function Map.empty()
    return setmetatable({ data = {} }, Map)
end

function Map:has_key(k)
    return self.data[k] ~= nil
end

function Map:set(k, v)
    self.data[k] = v
end

function Map:get(k)
    return self.data[k]
end

function Map:remove(k)
    self.data[k] = nil
end

function Map:count()
    local c = 0
    for _ in pairs(self.data) do c = c + 1 end
    return c
end

function Map:keys()
    local l = {}
    for k, _ in pairs(self.data) do table.insert(l, k) end
    return List.create(l)
end

function Map.clone(m)
    local n = Map.empty()
    for k, v in pairs(m.data) do
        n:set(k, v)
    end
    return n
end

return Map

