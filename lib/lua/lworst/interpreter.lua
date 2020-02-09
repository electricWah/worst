
local base = require("base")
local List = require("list")
local Error = base.Error
local Symbol = base.Symbol
local Stack = base.Stack
local Type = base.Type
local Readonly = base.Readonly

local Interpreter = {}
Interpreter.__index = Interpreter

Interpreter.ERROR_HANDLER = "current-error-handler"

function Interpreter:reset()
    self.defs = {}
    self.body = List.empty()
    self.childs = {}
    self.parent = nil
end

function Interpreter.empty()
    local new = setmetatable({}, Interpreter)
    new:reset()
    return new
end

function Interpreter.create(body)
    local i = Interpreter.empty()
    i.body = List.create(body)
    return i
end

function Interpreter:assume(source)
    self.defs = source.defs
    self.body = source.body
    self.childs = source.childs
    self.parent = source.parent
end

function Interpreter:clone()
    local r = Interpreter.empty()
    r:assume(self)
    return r
end

function Interpreter:resolve(name)
    if Symbol.is(name) then name = Symbol.unwrap(name) end
    if self.defs[name] ~= nil then
        return self.defs[name]
    elseif self.parent then
        return self.parent:resolve(name)
    else
        return nil
    end
end

function Interpreter:enter_body(body)
    local parent = self:clone()
    self:reset()
    self.parent = parent
    self.body = List.create(body)
end

function Interpreter:set_body(body)
    self.body = List.create(body)
end

function Interpreter:body_read()
    local body, v = self.body:pop()
    -- print("body_read", self.body, body, v)
    self.body = body
    return v
end

function Interpreter:code_read()
    while true do
        -- ctx->child-innermost
        while true do
            local child = table.remove(self.childs)
            if child then
                local parent = self:clone()
                self:assume(child)
                self.parent = parent
            else
                break
            end
        end

        local body = self:body_read()
        if body ~= nil then
            return body
        end

        if self.parent then
            self:assume(self.parent)
        else
            return nil
        end
    end
end

function Interpreter:into_parent()
    if not self.parent then
        return false
    end
    local child = self:clone()
    child.parent = nil
    self:assume(self.parent)
    table.insert(self.childs, child)
    return true
end

function Interpreter:define(name, def)
    if Symbol.is(name) then name = Symbol.unwrap(name) end
    self.defs[name] = def
end

function Interpreter:define_all(t)
    for name, def in pairs(t) do
        self:define(name, def)
    end
end

function Interpreter:definition_get(name)
    if Symbol.is(name) then name = Symbol.unwrap(name) end
    return self.defs[name]
end

function Interpreter:definition_remove(name)
    if Symbol.is(name) then name = Symbol.unwrap(name) end
    self.defs[name] = nil
end

function Interpreter:error(name, ...)
    -- print("Error", name, ...)
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
        self:enter_body(handler)
    else
        local irr_messages = {}
        for _, v in ipairs(irritants:to_table()) do
            table.insert(irr_messages, tostring(v))
        end
        error(name .. " " .. table.concat(irr_messages, ", "), 0)
    end
end

function Interpreter:eval(stack, v, name)
    if type(v) == "function" then
        local ok, err = pcall(v, self, stack)
        if not ok then
            if name then print("Error in", name, stack) end
            if type(err) == "table" then
                return self:handle_error(stack, err[1], unpack(err, 2))
            else
                return self:handle_error(stack, err)
            end
        end
    elseif List.is(v) then
        self:enter_body(v)
    else
        self:stack_push(stack, v)
    end
end

function Interpreter:call(stack, name)
    -- print("call", name)
    if Symbol.is(name) then name = Symbol.unwrap(name) end
    local def = self:resolve(name)
    if def == nil then
        return self:handle_error(stack, "undefined", name)
    end
    self:eval(stack, def, name)
end

function Interpreter:stack_push(stack, v)
    if v == nil then error("stack_push(nil)") end
    -- print("+", v)
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
    if ty ~= nil and (Type.is(ty) and not ty.is(v)) and (type(v) ~= ty) then
        -- print("Stack:", unpack(stack))
        self:error("wrong-type", ty, v)
    else
        return v
    end
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

