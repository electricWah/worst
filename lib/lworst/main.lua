
local base = require("base")
local Symbol = base.Symbol

local Interpreter = require("interpreter")

local reader = require("reader")

function run_file(arg)

    local scriptfile = io.open(arg) or error("could not open script: " .. arg)
    local script = scriptfile:read("*a")
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

    while interp:step() do end
end

run_file(arg[1])

