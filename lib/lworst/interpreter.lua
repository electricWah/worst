
local os = require "os"
local base = require "lworst/base"
local List = require "lworst/list"
local Error = base.Error
local Symbol = base.Symbol
local Stack = base.Stack
local Type = base.Type

local Interpreter = Type.new("interpreter")
function Interpreter:__tostring() return "<interpreter>" end

function frame_empty(name)
    return {
        body = List.empty(),
        threads = {},
        childs = {},
        defs = {},
        name = name
    }
end

function Interpreter.empty()
    return setmetatable({
        parents = {},
        frame = frame_empty(),
        defstacks = {},
        stack = Stack.empty(),
    }, Interpreter)
end

function Interpreter.create(body)
    local i = Interpreter.empty()
    i.frame.body = List.create(body)
    return i
end

function Interpreter:is_toplevel() return #self.parents == 0 end

function Interpreter:resolve(name)
    if self.frame.defs[name] ~= nil then
        return self.frame.defs[name]
    else
        local namestack = self.defstacks[name]
        return namestack and namestack[#namestack]
    end
end

-- returns old frame
function enter_parent_frame(interp)
    if #interp.parents == 0 then return nil end
    local frame = interp.frame
    interp.frame = table.remove(interp.parents)
    -- table.insert(interp.frame.childs, frame)

    for name, def in pairs(interp.frame.defs) do
        table.remove(interp.defstacks[name])
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

function Interpreter:step_into_new(body, name)
    local f = frame_empty(name)
    f.body = List.create(body)
    enter_child_frame(self, f)
end

function Interpreter:set_body(body)
    self.frame.body = List.create(body)
end

function Interpreter:get_body(body) return self.frame.body end

function Interpreter:body_read()
    local body, v = self.frame.body:pop()
    self.frame.body = body or List.empty()
    return v
end

function Interpreter:into_parent()
    local old_frame = enter_parent_frame(self)
    if not old_frame then
        return false
    elseif old_frame.body:length() > 0 or #old_frame.childs > 0 then
        table.insert(self.frame.childs, old_frame)
    end
    return true
end

function Interpreter:reset()
    while self:into_parent() do end
    local defs = self.frame.defs
    self.frame = frame_empty()
    self.frame.defs = defs
end

function Interpreter:define(name, def)
    if type(name) == "string" then name = Symbol.new(name) end
    if name == nil then
        self:error("define(nil, def)")
    elseif def == nil then
        self:error("define(_, nil)", name)
    else
        self.frame.defs[name] = def
    end
end

function Interpreter:definition_get(name)
    return self.frame.defs[name]
end

function Interpreter:definition_remove(name)
    self.frame.defs[name] = nil
end

function Interpreter:definitions() return self.frame.defs end

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

function Interpreter:pause(v) coroutine.yield(v) end

function Interpreter:error(name, ...)
    coroutine.yield(Error.new(name, List.new {...}), 2)
end

-- current is at the front
function Interpreter:call_stack()
    local st = List.empty()
    for _,  p in ipairs(self.parents) do
        st = List.push(st, p.name or false)
    end
    st = List.push(st, self.frame.name or false)
    return st
end

function Interpreter:set_trace_port(p) self.trace_port = p end

function start_eval_trace(i, name)
    local out = i.trace_port
    if not out then return end
    local t = os.clock()

    local unknown = Symbol.new("???")
    local prefix = " worst`"

    local st = {}
    function write_st(name)
        if name then
            table.insert(st, prefix .. base.to_string_terse(name or unknown))
        end
    end

    write_st(name)
    write_st(i.frame.name)
    for pt = #i.parents, 1, -1 do
        local p = i.parents[pt]
        write_st(p.name)
    end

    -- for _,  p in ipairs(i.parents) do
    --     write_st(p.name)
    -- end
    -- write_st(i.frame.name)
    -- write_st(name)

    local trace = table.concat(st, "\n")
    return out, trace, t
end

function write_eval_trace(out, trace, t0)
    if not out then return end

    local t = os.clock()

    -- convert to float for some more precise rounding than just floor
    local ns = math.floor(tonumber(string.format("%f", ((t - t0) * 1000000))))

    out:write_string(trace .. "\n" .. tostring(ns) .. "\n\n")
end

local EVAL_BREAK = {}

function Interpreter:eval(v, name)
    -- local out, trace, t = start_eval_trace(self, name)
    if List.is(v) then
        self:step_into_new(v, name)
        self:into_parent()
        self:pause(EVAL_BREAK)
    elseif base.can.call(v) then
        local out, trace, t = start_eval_trace(self, name)
        v(self)
        write_eval_trace(out, trace, t)
    else
        self:stack_push(v)
    end
    -- write_eval_trace(out, trace, t)
end

-- prepare something to eval for next call to run()
function Interpreter:eval_next(v, name)
    if List.is(v) then
        -- set toplevel body
        -- TODO and name == nil?
        if self:is_toplevel() and List.length(self.frame.body) == 0 then
            self.frame.body = List.create(v)
        else
            self:step_into_new(v, name)
            self:into_parent()
        end
    elseif base.can.call(v) then
        table.insert(self.frame.threads, coroutine.create(v))
    else
        -- TODO step_into_new(List.new{v})
        if v == nil then v = "<nil>" end -- ?
        self:error("eval_next", v)
    end
end

-- returns nil on completion, error on error, any other value on pause
function Interpreter:run()
    while true do
        -- leave uplevels
        while true do
            local child = table.remove(self.frame.childs)
            if child then
                enter_child_frame(self, child)
            else
                break
            end
        end

        -- take paused coroutine first
        local thread = table.remove(self.frame.threads)
        if thread then
            local ok, res = coroutine.resume(thread, self)
            if coroutine.status(thread) ~= "dead" then
                table.insert(self.frame.threads, thread)
            end
            if not ok then self:error(res) end
            if res ~= nil and res ~= EVAL_BREAK then return res end
        else
            local c = self:body_read()
            if c == nil then
                if enter_parent_frame(self) == nil then
                    return nil
                end
            elseif Symbol.is(c) then
                local thread = coroutine.create(Interpreter.call)
                local ok, res = coroutine.resume(thread, self, c)
                if coroutine.status(thread) ~= "dead" then
                    table.insert(self.frame.threads, thread)
                end
                if not ok then self:error(res) end
                if res ~= nil and res ~= EVAL_BREAK then return res end
            else
                self:stack_push(c)
            end
        end
    end
end

function Interpreter:try_resolve(name)
    local def = self:resolve(name)
    if def == nil then
        self:error("undefined", name)
        def = self:stack_pop()
    end
    return def
end

function Interpreter:call(name)
    local def = self:try_resolve(name)
    self:eval(def, name)
end

function Interpreter:quote(purpose)
    local v = self:body_read()
    if v == nil then
        self:error("quote-nothing", purpose)
        v = self:stack_pop()
    end
    return v
end

function Interpreter:stack_push(v)
    if v == nil then
        self:error("stack_push(nil)")
    elseif type(v) == "table" and getmetatable(v) == nil then
        self:error("stack_push: unknown type", v)
    else
        self.stack:push(v)
    end
end

function Interpreter:assert_type(v, ty, purpose)
    if not Type.is(ty, v) then
        self:error("wrong-type", Type.name(ty), v, purpose)
        return nil
    else
        return v
    end
end

function Interpreter:stack_ref(i, ty)
    local v = self.stack[#self.stack - (i - 1)]
    if v == nil then
        self:error("stack-empty")
    elseif ty ~= nil then
        return self:assert_type(v, ty)
    else
        return v
    end
end

function Interpreter:stack_pop(ty, purpose)
    local v = self.stack:pop()
    if v == nil then
        self:error("stack-empty")
        return self:stack_pop(ty)
    elseif ty ~= nil then
        return self:assert_type(v, ty, purpose)
    else
        return v
    end
end

function Interpreter:stack_length()
    return self.stack:length()
end

function Interpreter:stack_get()
    local l = List.empty()
    for _, v in ipairs(self.stack) do
        l = l:push(v)
    end
    return l
end

function Interpreter:stack_set(l)
    while self.stack:pop() ~= nil do end
    for v in l:reverse():iter() do
        self:stack_push(v)
    end
end

return Interpreter

