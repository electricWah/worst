
local base = require("base")
local List = require("list")
local Type = base.Type
local types = require("types")
local Clone = types.Clone

local Map = Type.new("Map")
function Map:__tostring()
    return "Map(" .. self:count() .. ")"
end

function Map.empty()
    return setmetatable({}, Map)
end

function Map:has_key(k)
    return self[k] ~= nil
end

function Map:set(k, v)
    self[k] = v
end

function Map:get(k)
    return self[k]
end

function Map:remove(k)
    self[k] = nil
end

function Map:count()
    local c = 0
    for _ in pairs(self) do c = c + 1 end
    return c
end

function Map:keys()
    local l = {}
    for k, _ in pairs(self) do table.insert(l, k) end
    return List.create(l)
end

Clone.clone_for(Map, function(m)
    local n = Map.empty()
    for k, v in pairs(m) do
        n:set(k, v)
    end
    return n
end)

return Map

