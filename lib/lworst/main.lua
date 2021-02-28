
local io = require "io"
local base = require "base"
local port = require "port"
local reader = require "reader"
local builtins = require "builtins"
local Interpreter = require"interpreter"

local List = require "list"
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

    for name, def in pairs(builtins) do
        interp:define(Symbol.new(name), def)
    end

    local arglist = List.create({...})
    interp:define(Symbol.new("command-line-arguments"), function(i)
        i:stack_push(arglist)
    end)

    while interp:step() do end
end

return mod

