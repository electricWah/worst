
local base = require("base")
local List = require("list")
local Error = base.Error
local Symbol = base.Symbol
local Stack = base.Stack
local Type = base.Type

local Interpreter = {}
Interpreter.__index = Interpreter

Interpreter.ERROR_HANDLER = "current-error-handler"

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

function frame_empty()
    return { body = {}, childs = {}, defs = {} }
end

function Interpreter.empty()
    return setmetatable({
        parents = {},
        frame = frame_empty(),
        defstacks = {}
    }, Interpreter)
end

function Interpreter:reset()
    self.parents = {}
    self.frame = frame_empty()
    self.defstacks = {}
end

function Interpreter.create(body)
    local i = Interpreter.empty()
    i.frame.body = List.create(body)
    return i
end

function Interpreter:resolve(name)
    -- if Symbol.is(name) then name = Symbol.unwrap(name) end
    if self.frame.defs[name] ~= nil then
        return self.frame.defs[name]
    end
    local namestack = self.defstacks[name]
    return namestack and namestack[#namestack]
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

function enter_body(interp, body)
    local f = frame_empty()
    f.body = List.create(body)
    enter_child_frame(interp, f)
end

function Interpreter:set_body(body)
    self.frame.body = List.create(body)
end

function Interpreter:body_read()
    local body, v = self.frame.body:pop()
    self.frame.body = body
    return v
end

function Interpreter:into_parent()
    local old_frame = enter_parent_frame(self)
    if not old_frame then return false end
    table.insert(self.frame.childs, old_frame)
    return true
end

function Interpreter:define(name, def)
    -- if Symbol.is(name) then name = Symbol.unwrap(name) end
    self.frame.defs[name] = def
end

function Interpreter:definition_get(name)
    -- if Symbol.is(name) then name = Symbol.unwrap(name) end
    return self.frame.defs[name]
end

function Interpreter:definition_remove(name)
    -- if Symbol.is(name) then name = Symbol.unwrap(name) end
    self.frame.defs[name] = nil
end

function Interpreter:code_read()
    while true do
        -- ctx->child-innermost
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
        end

        if enter_parent_frame(self) == nil then
            return nil
        end
    end
end

function Interpreter:error(name, ...)
    error(Error({name, ...}), 0)
end

function Interpreter:handle_error(stack, name, ...)
    local irritants = List.create({...})
    local handler = self:resolve(Interpreter.ERROR_HANDLER)
    if type(handler) == "function" then
        self:stack_push(stack, irritants)
        self:stack_push(stack, Symbol.new(name))
        handler(self, stack)
        return true
    elseif List.is(handler) then
        -- print("handle_error", handler)
        enter_body(self, handler)
    else
        local irr_messages = {}
        for v in irritants:iter() do
            table.insert(irr_messages, tostring(v))
        end
        error(name .. ": " .. table.concat(irr_messages, ", "), 0)
    end
end

function Interpreter:eval(stack, v, name)
    if List.is(v) then
        enter_body(self, v)
    elseif type(v) == "function" then
        local ok, err = pcall(v, self, stack)
        if not ok then
            print("Error in", name or "???", stack, err)
            if type(err) == "table" then
                return self:handle_error(stack, err[1], unpack(err, 2))
            else
                return self:handle_error(stack, err)
            end
        end
    else
        self:stack_push(stack, v)
    end
end

function Interpreter:call(stack, name)
    -- print("call", name)
    -- if Symbol.is(name) then name = Symbol.unwrap(name) end
    local def = self:resolve(name)
    if def == nil then
        return self:handle_error(stack, "undefined", name)
    end
    self:eval(stack, def, name)
end

function Interpreter:stack_push(stack, v)
    if v == nil then error("stack_push(nil)") end
    if type(v) == "table" and getmetatable(v) == nil then
        self:error("stack_push: unknown type", v)
    end
    stack:push(v)
end

function Interpreter:stack_ref(stack, i, ty)
    local v = stack[#stack - (i - 1)]
    if v == nil then self:error("stack-empty") end
    if ty ~= nil and (Type.is(ty) and not ty.is(v)) and (type(v) ~= ty) then
        -- print("Stack:", unpack(stack))
        self:error("wrong-type", ty, v)
    else
        return v
    end
end

function Interpreter:stack_pop(stack, ty)
    local v = stack:pop()
    if v == nil then self:error("stack-empty") end
    if ty ~= nil then
        if type(v) == ty then
        elseif Type.is(ty) and ty.is(v) then
        else
            self:error("wrong-type", ty, v)
        end
    end

    return v
end

function Interpreter:step(stack)
    local code = self:code_read()
    if code == nil then
        return false
    else
        if Symbol.is(code) then
            -- print(">", code)
            self:call(stack, code)
        else
            self:stack_push(stack, code)
        end
    end
    return true
end

return Interpreter

