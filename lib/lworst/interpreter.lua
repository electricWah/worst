
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

function frame_empty(name)
    return { body = List.empty(), childs = {}, defs = {}, name = name }
end

function Interpreter.empty()
    return setmetatable({
        data = {
            -- little baby hack to allow defs access to types
            Symbol = Symbol,
            List = List,
        },
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

function Interpreter:body_read()
    local body, v = self.frame.body:pop()
    self.frame.body = body
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
    error(Error({name, ...}), 0)
end

function Interpreter:handle_error(stack, name, ...)
    local irritants = List.create({...})
    local handler = self:resolve(Interpreter.ERROR_HANDLER)
    if type(handler) == "function" then
        self:stack_push(stack, irritants)
        self:stack_push(stack, Symbol.new(name))
        handler(self, stack)
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
    -- if true then
    --     local st = {}
    --     for _,  p in ipairs(self.parents) do
    --         table.insert(st, base.to_string_terse(p.name or "???"))
    --     end
    --     table.insert(st, base.to_string_terse(self.frame.name or "???"))
    --     table.insert(st, base.to_string_terse(name or "???"))

    --     io.stderr:write(table.concat(st, " "), "\n")
    -- end
    if List.is(v) then
        enter_body(self, v, name)
    elseif type(v) == "function" then
        local ok, err = pcall(v, self, stack)
        if not ok then
            print("Error in", name or "???", stack, err)
            for _, p in ipairs(self.parents) do
                print("...", p.name or "???")
            end
            print("...", self.frame.name or "???")
            if type(err) == "table" then
                self:handle_error(stack, err[1], unpack(err, 2))
            else
                self:handle_error(stack, err)
            end
        end
    else
        self:stack_push(stack, v)
    end
end

function Interpreter:call(stack, name)
    -- print("call", name)
    local def = self:resolve(name)
    if def == nil then
        self:handle_error(stack, "undefined", name)
    else
        self:eval(stack, def, name)
    end
end

function Interpreter:stack_push(stack, v)
    if v == nil then
        error("stack_push(nil)")
    elseif type(v) == "table" and getmetatable(v) == nil then
        self:error("stack_push: unknown type", v)
    else
        stack:push(v)
    end
end

function Interpreter:stack_ref(stack, i, ty)
    local v = stack[#stack - (i - 1)]
    if v == nil then
        self:error("stack-empty")
    elseif ty ~= nil and not Type.is(ty, v) then
        self:error("wrong-type", Type.name(ty), v)
    else
        return v
    end
end

function Interpreter:stack_pop(stack, ty)
    local v = stack:pop()
    if v == nil then
        self:error("stack-empty")
    elseif ty ~= nil and not Type.is(ty, v) then
        self:error("wrong-type", Type.name(ty), v)
    else
        return v
    end
end

function Interpreter:step(stack)
    local code = self:code_read()
    if code == nil then
        return false
    elseif Symbol.is(code) then
        -- print(">", code)
        self:call(stack, code)
    else
        self:stack_push(stack, code)
    end
    return true
end

return Interpreter
