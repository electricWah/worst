
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

function List.empty()
    return setmetatable({ data = {}, top = 0 }, List)
end

function List:length() return self.top end
function List:head() return self.data[self.top] end
function List:index(n) return self.data[self.top - n] end

function List.len(t) if List.is(t) then return t:length() else return #t end end

function List:clone()
    return setmetatable({
        data = self.data,
        top = self.top,
    }, List)
end

function List:full_clone()
    local l = List.empty()
    for i = 1, self.top do
        l.data[i] = self.data[i]
    end
    l.top = self.top
    return l
end

function List:pop()
    if self.top == 0 then return nil, nil end
    local l = self:clone()
    local v = l.data[l.top]
    l.top = l.top - 1
    return l, v
end

function List:push(v)
    local l
    if self.data[self.top + 1] ~= nil then
        l = self:full_clone()
    else
        l = self:clone()
    end
    l.top = l.top + 1
    l.data[l.top] = v
    return l
end

function List.create(data)
    if List.is(data) then
        return data:clone()
    elseif getmetatable(data) then
        error("List.create: not a plain table: " .. base.to_string_debug(data))
    else
        local l = List.empty()
        for i, v in ipairs(data) do
            l = l:push(v)
        end
        l = l:reverse()
        return l
    end
end
function List.new(data) return List.create(data) end

function List.to_table(t)
    if not List.is(t) then return t end
    local r = {}
    for i = t.top, 1, -1 do
        table.insert(r, t.data[i])
    end
    return r
end

function List:reverse()
    local r = List.empty()
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

function List.to_string_terse(l)
    local acc = {}
    for v in l:iter() do
        table.insert(acc, base.to_string_terse(v))
    end
    return "(" .. table.concat(acc, " ") .. ")"
end
List.__tostring = function(l) return List.to_string_terse(l) end

function List.equal(a, b)
    local alen = a:length()
    if not List.is(b) then return false
    elseif alen ~= b:length() then return false
    elseif alen == 0 then return true
    else
        for i = 0, alen do
            if not base.equal(a:index(i), b:index(i)) then return false end
        end
        return true
    end
end

function List:iter()
    function f(st, v)
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

return List


