
local base = require("base")
local Stack = base.Stack
local Symbol = base.Symbol

local Interpreter = require("interpreter")

local reader = require("reader")

function run(args)

    local script = io.input(args[1]):read("*a")
    local r = reader.StringReader.new(script)

    local body = {}
    while true do
        local stx = reader.read_next(r)
        if stx == nil then break end
        table.insert(body, stx)
    end

    local interp = Interpreter.create(body)

    for name, def in pairs(require("builtins")) do
        interp:define(Symbol.new(name), def)
    end

    local stack = Stack.empty()
    while interp:step(stack) do
        -- print("Stack: ", unpack(stack))
    end
end

run(arg)

