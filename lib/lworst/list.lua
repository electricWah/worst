
local base = require "lworst/base"
local Type = base.Type

-- Immutable lists

-- List contains
-- - Data table
-- - Top index
-- Data table contains:
-- - The data (duh)

-- TODO if performance degrades due to shared pop/push and too much full_clone
-- add a weak refs table in data pointing to self,
-- and if self is a unique reference to the data, don't full_clone

-- Pop: just decrement top
-- Push:
-- - if data[top + 1] is nil, just table.insert and increment top
-- - otherwise do a complete clone

local List = Type.new("list")

-- create
function List.new(src)
    if List.is(src) then
        return src
    elseif getmetatable(src) then
        return error("List.new: not a plain table: " .. tostring(src))
    end

    local data, min, max = {}, 0, 0
    if src ~= nil then
        for _, v in ipairs(src) do
            data[max] = v
            max = max + 1
        end
    end
    return setmetatable({
        data = data,
        min = min,
        max = max,
    }, List)
end

function List:clone()
    return setmetatable({
        data = self.data,
        min = self.min,
        max = self.max,
    }, List)
end

-- query
function List:length()
    if List.is(self) then
        return self.max - self.min
    else
        return #self
    end
end
function List:index(n)
    if n >= 0 and n < self.max - self.min then
        return self.data[self.min + n]
    else
        return nil
    end
end
function List:head() return self:index(0) end

function List.__eq(a, b)
    local alen = a:length()
    if alen ~= b:length() then return false
    elseif alen == 0 then return true
    else
        for i = 0, alen do
            if a:index(i) ~= b:index(i) then return false end
        end
        return true
    end
end

-- modify
function list_clone(l, full)
    l = base.clone(l)
    if full then
        local data, min, max = {}, 0, 0
        for _, v in List.ipairs(l) do
            data[max] = v
            max = max + 1
        end
        l.data, l.min, l.max = data, min, max
    end
    return l
end

function List:pop()
    if self:length() < 1 then return nil, nil end
    local l = list_clone(self)
    local v = l.data[l.min]
    l.min = l.min + 1
    return l, v
end

function List:push(v)
    local l = list_clone(self, (self.data[self.min - 1] ~= nil))
    l.min = l.min - 1
    l.data[l.min] = base.value(v)
    return l
end

function List:shift()
    if self:length() < 1 then return nil, nil end
    local l = list_clone(self)
    local v = l.data[l.max]
    l.max = l.max - 1
    return l, v
end

function List:unshift(v)
    local l = list_clone(self, (self.data[self.max + 1] ~= nil))
    l.max = l.max + 1
    l.data[l.max] = base.value(v)
    return l
end

-- transform

function List:reverse()
    local r = List.new()
    for v in self:iter() do
        r = r:push(v)
    end
    return r
end

function List:append(thee)
    for v in self:reverse():iter() do
        thee = thee:push(v)
    end
    return thee
end

-- convert

function List.to_table(t)
    if not List.is(t) then return t end
    local r = {}
    for i = t.min, t.max - 1 do
        table.insert(r, t.data[i])
    end
    return r
end

function List:__tostring()
    local acc = {}
    for v in self:iter() do
        table.insert(acc, tostring(v))
    end
    return "(" .. table.concat(acc, " ") .. ")"
end

-- traverse

function List:iter()
    function f(st)
        local s, v = st.s:pop()
        if not s then return nil end
        st.s = s
        return v
    end
    return f, {s=self}, self
end

-- ipairs over a list or table
function List.ipairs(t)
    if List.is(t) then
        function f(t, i)
            if i >= t:length() then return nil end
            return i + 1, t:index(i)
        end
        return f, t, 0
    else
        return ipairs(t)
    end
end

function List:map(f)
    local r = {}
    for _, v in List.ipairs(self) do
        table.insert(r, f(v))
    end
    return List.new(r)
end

-- as pairs

-- create a (k v ...) list from pairs(src)
function List.new_pairs(src, allow_metatable)
    if (getmetatable(src) and not allow_metatable) or type(src) ~= "table" then
        return error("List.new_pairs: not a plain table: " .. tostring(src))
    end

    local data, min, max = {}, 0, 0
    for k, v in pairs(src) do
        data[max] = k
        data[max + 1] = v
        max = max + 2
    end
    return setmetatable({
        data = data,
        min = min,
        max = max,
    }, List)
end

-- pairs over a list of (k1 v1 k2 v2 ...) or table
function List.pairs(t)
    if List.is(t) then
        function f(s)
            local i = s.i
            if i >= s.t:length() then return nil end
            s.i = i + 2
            return t:index(i), t:index(i + 1)
        end
        return f, {t=t, i=0}, 0
    else
        return pairs(t)
    end
end

function List:find_key(k)
    -- how to differentiate between i-tables and kv-tables?
    -- if not List.is(self) then
    --     return self[k]
    -- end
    for key, v in List.pairs(self) do
        if key == k then
            return v
        end
    end
    return nil
end

function List:add_pair(k, v) return self:push(v):push(k) end

return List




