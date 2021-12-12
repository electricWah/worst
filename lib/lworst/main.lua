
local base = require "lworst/base"
local port = require "lworst/port"
local reader = require "lworst/reader"
local Interpreter = require "lworst/interpreter"
local builtins_all = require "lworst/builtins/all"

local List = require "lworst/list"
local Symbol = base.Symbol

local mod = {}

function mod.run_file(path, ...)
    local r, err = port.open_input_file(path)
    if err then
        error("could not open script: " .. path .. " " .. tostring(err))
    end

    local body = {}
    while true do
        local stx = reader.read_next(r)
        if stx == nil then break end
        table.insert(body, stx)
    end

    local interp = Interpreter.create(body)

    local arglist = List.new({...})
    interp:define(Symbol.new("command-line-arguments"), function(i)
        i:stack_push(arglist)
    end)

    builtins_all(interp)

    -- wrap in a coroutine in case of toplevel yield
    local ok, err = coroutine.resume(coroutine.create(function()
        local err = interp:run()
        if err then
            print(err)
            print(err.lua_stack)
        end
    end))
    if not ok then
        print(err)
    end
    if base.Error.is(err) then
        print(err)
        print(err.lua_stack)
    end
end

return mod

