
local minizip = require "minizip"

local zipio = setmetatable({}, { __index = require "io" })

zipio.open = function(f, mode)
    mode = mode or "r"
    if mode ~= "r" or string.sub(f, 1, 2) ~= "%/" then
        return io.open(f, mode)
    end
    return minizip.open(zipio.zip_file, string.sub(f, 3), "r")
end

package.loaded["io"] = zipio

local io = require "io"

local base = require("base")
local reader = require("reader")
local builtins = require("builtins")
local Interpreter = require("interpreter")

local Symbol = base.Symbol

local mod = {}

function mod.run(thisfile, arg, ...)
    zipio.zip_file = thisfile
    -- print(thisfile, arg, ...)

    local scriptfile, err = io.open(arg)
    if not scriptfile then
        error("could not open script: " .. arg .. " " .. tostring(err))
    end
    local script = scriptfile:read("*a")
    local r = reader.StringReader.new(script)

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

    while interp:step() do end
end

return mod

