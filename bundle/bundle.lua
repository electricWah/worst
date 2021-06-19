
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

local Main = require "lworst/main"

local mod = {}

function mod.run(thisfile, arg, ...)
    zipio.zip_file = thisfile
    Main.run_file(arg, thisfile, ...)
end

return mod

