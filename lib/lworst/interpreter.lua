
local os = require "os"
local base = require "lworst/base"
local List = require "lworst/list"
local Error = base.Error
local Symbol = base.Symbol
local Type = base.Type

local Interpreter = Type.new("interpreter")
function Interpreter:__tostring() return "<interpreter>" end

local Frame = Type.new("frame")
function new_frame(body)
    if base.is_a(body, "function") then
        local b = body
        body = coroutine.create(function(i) b(i) end)
    end
    return setmetatable({
        body = body or List.new(),
        childs = {},
        defs = {}, -- maybe nil if thread so you definitely can't define
    }, Frame)
end

function Interpreter.new(body)
    local frame
    -- function frames are always evaluated as if in their parent frames
    -- so toplevel frame must be a list (even if empty)
    if base.is_a(body, "function") then
        frame = new_frame()
        table.insert(frame.childs, new_frame(body))
    else
        frame = new_frame(body)
    end
    return setmetatable({
        parents = {},
        frame = frame,
        defstacks = {},
        stack = {},
    }, Interpreter)
end

-- returns old frame
function enter_parent_frame(interp, keep_child)
    if #interp.parents == 0 then return nil end
    local frame = interp.frame
    interp.frame = table.remove(interp.parents)

    for name, def in pairs(interp.frame.defs) do
        table.remove(interp.defstacks[name])
    end

    if keep_child or #frame.childs > 0 then
        table.insert(interp.frame.childs, frame)
    end
    if not base.is_a(interp.frame.body, List) then
        print("into_parent not list?")
    end

    return frame
end

function enter_child_frame(interp, frame)
    for name, def in pairs(interp.frame.defs) do
        local s = interp.defstacks[name] or {}
        table.insert(s, def)
        interp.defstacks[name] = s
    end

    table.insert(interp.parents, interp.frame)
    interp.frame = frame
end

function Interpreter:set_body(body) self.frame.body = List.new(body) end
function Interpreter:get_body(body) return self.frame.body or List.new() end
function Interpreter:is_toplevel() return #self.parents == 0 end

function read_body(interp)
    local v
    interp.frame.body, v = interp.frame.body:pop()
    return v
end

function Interpreter:reset()
    while enter_parent_frame(self) do end
    local defs = self.frame.defs
    self.frame = new_frame()
    self.frame.defs = defs
end

function Interpreter:eval(body)
    -- check current before yield?
    if base.is_a(body, List) then
        coroutine.yield(new_frame(body))
    elseif base.is_a(body, "function") then
        coroutine.yield(new_frame(body))
    else
        error("eval: " .. tostring(body))
    end
end

function Interpreter:enter_new_frame(body)
    enter_child_frame(self, new_frame(body))
end

function Interpreter:do_uplevel(body)
    if self:is_toplevel() then return self:error("root-uplevel") end
    local prev = enter_parent_frame(self)
    self:eval(body)
    enter_child_frame(self, prev)
end

-- returns nil on completion, base.Error on error, any other value on pause
function Interpreter:run()
    while true do
        local frame = table.remove(self.frame.childs)
        if frame and base.is_a(frame.body, "thread") then
            -- thread/function frames are run in their parent context
            if coroutine.status(frame.body) == "dead" then
                -- drop it
            else
                local ok, res = coroutine.resume(frame.body, self)
                if coroutine.status(frame.body) ~= "dead" then
                    table.insert(self.frame.childs, frame)
                end
                if not ok then
                    -- necessary? wrap non-Error in Error? ditch Error? idk
                    self:error(res)
                elseif base.is_a(res, Frame) then
                    table.insert(self.frame.childs, res)
                elseif res ~= nil then
                    return res
                end
            end
        elseif frame then
            enter_child_frame(self, frame)
        elseif base.is_a(self.frame.body, List) then
            local v
            self.frame.body, v = self.frame.body:pop()
            -- print("read", v, self.frame.body)
            if v == nil then
            elseif base.is_a(v, Symbol) then
                -- call in frame coroutine to catch 'undefined'
                table.insert(self.frame.childs, new_frame(function(i) i:call(v) end))
            else
                self:stack_push(v)
            end
        elseif self.frame.body == nil then
            if not enter_parent_frame(self) then
                return nil
            end
        else
            error("bad body " .. tostring(self.frame.body) .. debug.traceback(""))
        end
    end
end

function Interpreter:call(name)
    local def = self:try_resolve(name)
    self:eval(def)
end

function Interpreter:quote(purpose)
    local v = read_body(self)
    if v == nil then
        self:error("quote-nothing", purpose)
        v = self:stack_pop(nil, base.value(purpose and ("quote: " .. purpose) or "quote"))
    end
    return v
end

function Interpreter:resolve(name)
    if self.frame.defs[name] ~= nil then
        return self.frame.defs[name]
    else
        local namestack = self.defstacks[name]
        return namestack and namestack[#namestack]
    end
end

function Interpreter:define(name, def)
    if type(name) == "string" then name = Symbol.new(name) end
    if name == nil then
        self:error("define(nil, def)")
    elseif def == nil then
        self:error("define(_, nil)", name)
    else
        self.frame.defs[name] = base.meta.set(base.value(def), "name", name)
    end
end

function Interpreter:definition_get(name)
    return self.frame.defs[name]
end

function Interpreter:definition_remove(name)
    self.frame.defs[name] = nil
end

function Interpreter:definitions()
    local m = {}
    for k, v in pairs(self.frame.defs) do
        m[k] = v
    end
    return m
end

function Interpreter:all_definitions()
    local m = {}
    for k, v in pairs(self.defstacks) do
        m[k] = v[#v]
    end
    for k, v in pairs(self.frame.defs) do
        m[k] = v
    end
    return m
end

function Interpreter:pause(v)
    -- check current? remove this and just yield?
    coroutine.yield(v)
end

function Interpreter:error(name, ...)
    -- move into base.error?
    coroutine.yield(Error.new(name, List.new {...}))
end

function Interpreter:try_resolve(name)
    local def = self:resolve(name)
    if def == nil then
        self:error("undefined", name)
        def = self:stack_pop()
    end
    return def
end

function Interpreter:stack_push(v)
    local mt = getmetatable(v)
    if v == nil then
        self:error("stack_push(nil)")
    elseif type(v) == "table" and mt == nil then
        self:error("stack_push: unknown type", v)
    else
        table.insert(self.stack, base.value(v))
    end
end

function Interpreter:assert_type(v, ty, purpose)
    -- could move to base if error is
    if not base.is_a(v, ty) then
        local type_name = ty
        if type(ty) == "table" and not getmetatable(ty) then
            local tt = {}
            for _, v in ipairs(ty) do table.insert(tt, tostring(v)) end
            type_name = table.concat(tt, ", ")
        else
            type_name = tostring(ty)
        end

        self:error("wrong-type", type_name, v, purpose)
        return nil
    elseif type(ty) == "string" then
        return base.unwrap_lua(v)
    else
        return v
    end
end

function Interpreter:stack_ref(i, ty)
    local v = self.stack[#self.stack - (i - 1)]
    if v == nil then
        self:error("stack-empty", ty)
    elseif ty ~= nil then
        return self:assert_type(v, ty, "stack_ref " .. i)
    else
        return v
    end
end

function Interpreter:stack_pop(ty, purpose)
    while true do
        local v = table.remove(self.stack)
        if v == nil then
            local type_name = ty
            if type(ty) == "table" and not getmetatable(ty) then
                type_name = List.new_pairs(ty)
            end
            self:error("stack-empty", type_name or "<any>", purpose)
        elseif ty ~= nil then
            return self:assert_type(v, ty, purpose)
        else
            return v
        end
    end
end

function Interpreter:stack_length()
    return #self.stack
end

function Interpreter:stack_get()
    local l = List.new()
    for _, v in ipairs(self.stack) do
        l = l:push(v)
    end
    return l
end

function Interpreter:stack_set(l)
    while table.remove(self.stack) ~= nil do end
    for v in l:reverse():iter() do
        self:stack_push(v)
    end
end

return Interpreter

