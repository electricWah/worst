
local base = require("base")
local Type = base.Type

-- Immutable lists with a shareable, immutable region and a mutable stack

local List = Type.new("list")

function List.to_string_terse(l)
    local acc = {}
    for v in l:iter() do
        table.insert(acc, base.to_string_terse(l))
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
            if not base.equal(a[i], b[i]) then return false end
        end
        return true
    end
end

-- List.__index = function(l, k)
--     if type(k) == "number" then
--         return l:index(k - 1)
--     else
--         return getmetatable(l)[k]
--     end
-- end

List.__len = function(l) return l:length() end

function List.create(src)
    if List.is(src) then
        return src:clone()
    elseif getmetatable(src) ~= nil then
        error("List.create: not a plain table: " .. base.to_string_debug(src))
    else
        return setmetatable({
            shared = base.readonly(src),
            sharedmax = #src + 1,
            sharedi = 1,
            stack = {},
            stacklen = 0,
        }, List)
    end
end

function List:clone()
    return setmetatable({
        shared = self.shared,
        sharedmax = self.sharedmax,
        sharedi = self.sharedi,
        stack = { unpack(self.stack) },
        stacklen = self.stacklen,
    }, List)
end

function List.empty() return List.create({}) end

function List:length()
    local sharedlen = self.sharedmax - self.sharedi
    return sharedlen + self.stacklen
end

function List:push(v)
    local new = self:clone()
    table.insert(new.stack, v)
    new.stacklen = new.stacklen + 1
    return new
end

function List:head()
    if self.stacklen > 0 then
        return self.stack[self.stacklen]
    else
        return self.shared[self.sharedi]
    end
end

function List:pop()
    local new = self:clone()
    if new.stacklen > 0 then
        new.stacklen = new.stacklen - 1
        return new, table.remove(new.stack)
    elseif new.sharedi < new.sharedmax then
        local r = new.shared[new.sharedi]
        new.sharedi = new.sharedi + 1
        return new, r
    else
        return nil, nil
    end
end

function List:index(n)
    if n < self.stacklen then
        return self.stack[self.stacklen - n]
    else
        return self.shared[self.sharedi + n - self.stacklen]
    end
end

function List:to_table()
    if self.stacklen == 0 then
        -- print("to_table 1")
        return { unpack(self.shared, self.sharedi) }
    else
        -- print("to_table 2")
        local r = {}
        for i = self.stacklen, 1, -1 do
            table.insert(r, self.stack[i])
        end
        for i = self.sharedi, self.sharedmax do
            table.insert(r, self.shared[i])
        end
        return r
    end
end

function List.append(a, b)
    local ta = a:to_table()
    local tb = b:to_table()
    for i, v in ipairs(tb) do
        -- print("insert", i, v)
        table.insert(ta, v)
    end
    return List.create(ta)
end

function List:reverse()
    local r = {}

    for i = self.sharedmax - 1, self.sharedi, -1 do
        table.insert(r, self.shared[i])
    end
    for i = 1, self.stacklen do
        table.insert(r, self.stack[i])
    end

    return List.create(r)
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

return List

