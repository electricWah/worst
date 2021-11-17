
local base = require "lworst/base"
local List = require "lworst/list"
local Type = base.Type

-- Immutable maps
-- could do something more complex with reference counting
-- but currently just clone-on-modify

local MapMeta = Type.new("MapMeta")

function MapMeta:__tostring()
    return "MapMeta<" .. self.name .. ">"
end

local Map = Type.new("Map")

function Map:__tostring()
    return "Map(" .. self:count() .. ")"
end

function Map.new(src)
    return setmetatable({ data = src }, Map)
end

function Map.empty()
    return Map.new({})
end

function Map:clone()
    local data = {}
    for k, v in pairs(self.data) do
        -- clone?
        data[k] = v
    end
    return setmetatable({ data = data }, Map)
end

function Map:has_key(k)
    return self.data[k] ~= nil
end

function Map:set(k, v)
    if v == self.data[k] then
        return self
    else
        local m = self:clone()
        m.data[k] = v
        return m
    end
end

function Map:get(k)
    return self.data[k]
end

function Map:remove(k)
    if self.data[k] == nil then
        return self
    else
        local m = self:clone()
        m.data[k] = nil
        return m
    end
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

function Map:iter()
    local s = self
    function f(st, v)
        local k, v = next(s.data, st.k)
        if k == nil then return nil end
        st.k = k
        return k, v
    end
    return f, {s=self}, self
end

return Map

