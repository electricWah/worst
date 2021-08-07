
local os = require "os"
local base = require "lworst/base"
local List = require "lworst/list"
local Error = base.Error
local Symbol = base.Symbol
local Stack = base.Stack
local Type = base.Type

local Interpreter = Type.new("interpreter")
Interpreter.__tostring = function() return "<interpreter>" end

-- instead of a parent / [childs] tree with O(stack length) resolve
-- interpreter contains:
-- - stack of parent frames
-- - current frame
-- - dict of def stacks <-- NEW
-- frame contains:
-- - body
-- - stack of children
-- - definition table

-- moving between frames updates the dict of def stacks
-- if traversing into a child push all in defs
-- if traversing into a parent, pop all in defs

function frame_empty(name)
    return { body = List.empty(), childs = {}, defs = {}, name = name }
end

function Interpreter.empty()
    return setmetatable({
        parents = {},
        frame = frame_empty(),
        defstacks = {},
        stack = Stack.empty(),
    }, Interpreter)
end

function Interpreter:reset()
    self.parents = {}
    self.defstacks = {}
    local defs = self.frame.defs
    self.frame = frame_empty()
    self.frame.defs = defs
end

function Interpreter.create(body)
    local i = Interpreter.empty()
    i.frame.body = List.create(body)
    return i
end

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

function enter_body(interp, body, name)
    local f = frame_empty(name)
    f.body = List.create(body)
    enter_child_frame(interp, f)
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

function Interpreter:code_read()
    while true do
        while true do
            local child = table.remove(self.frame.childs)
            if child then
                enter_child_frame(self, child)
            else
                break
            end
        end

        local body = self:body_read()
        if body ~= nil then
            return body
        elseif enter_parent_frame(self) == nil then
            return nil
        end
    end
end

function Interpreter:error(name, ...)
    error(Error.new(name, List.new {...}), 1)
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

function Interpreter:eval(v, name)
    -- local out, trace, t = start_eval_trace(self, name)
    if List.is(v) then
        enter_body(self, v, name)
    elseif base.can_call(v) then
        local out, trace, t = start_eval_trace(self, name)
        v(self)
        write_eval_trace(out, trace, t)
    else
        self:stack_push(v)
    end
    -- write_eval_trace(out, trace, t)
end

function Interpreter:call(name)
    -- print("call", name)
    local def = self:resolve(name)
    if def == nil then
        self:error("undefined", name)
    else
        self:eval(def, name)
    end
end

-- Helper functions to evaluate Worst from within Lua followed by some more Lua.
function continuation_setup(i, k)
    local name = Symbol.new("k")
    -- step into a new context ready to uplevel k
    i:eval(List.create { name }, name)
    -- step back up to eval k()
    i:define(name, function(i)
        i:into_parent()
        i:eval(k)
    end)
    -- finally go back up for the next part
    i:into_parent()
end

function Interpreter:eval_then(body, k)
    continuation_setup(self, k)
    self:eval(body)
end

function Interpreter:call_then(name, k)
    continuation_setup(self, k)
    self:call(name)
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

function Interpreter:assert_type(v, ty)
    if not Type.is(ty, v) then
        self:error("wrong-type", Type.name(ty), v)
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

function Interpreter:stack_pop(ty)
    local v = self.stack:pop()
    if v == nil then
        self:error("stack-empty")
    elseif ty ~= nil then
        return self:assert_type(v, ty)
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

-- return (still running, error)
function Interpreter:step()
    local code = self:code_read()
    if code == nil then
        return false, nil
    elseif Symbol.is(code) then
        local ok, err = pcall(Interpreter.call, self, code)
        if not ok then return false, err end
    else
        self:stack_push(code)
    end
    return true
end

-- returns nil or error
function Interpreter:run()
    local continue, err
    repeat
        continue, err = self:step()
    until not continue
    return err
end

return Interpreter

