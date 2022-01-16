
local t = require "test"
local base = require "lworst/base"
local I = require "lworst/interpreter"
local List = require "lworst/list"
local S = base.Symbol.new

local mod = {}

mod["run basic"] = function()
    local ran = false
    local i = I.new(function(i)
        ran = true
    end)
    t.check(not ran, "not ran")
    while true do
        if i:run() == nil then break end
    end
    t.check(ran, "ran")
end

mod["run inner"] = function()
    local ran = List.new()
    local i = I.new(function(i)
        ran = ran:push(1)
        i:eval(function(i)
            i:enter_new_frame() -- comment this out and do_uplevel fails
            ran = ran:push(2)
            i:do_uplevel(function(i)
                ran = ran:push(3)
            end)
        end)
        ran = ran:push(4)
    end)
    t.check_equal(List.new(), ran)
    while i:run() do end
    t.check_equal(List.new{ 4, 3, 2, 1 }, ran)
end

mod["run inner + uplevel + defs"] = function()
    local i = I.new(function(i)
        i:define("x", List.new{1})
        i:eval(function(i)
            i:define("x", List.new{2})
            i:enter_new_frame()
            i:define("x", List.new{3})
            i:do_uplevel(function(i)
                i:define("y", List.new{4})
            end)
        end)
        t.check_equal(List.new{2}, i:resolve(S"x"))
        t.check_equal(List.new{4}, i:resolve(S"y"))
    end)
    while i:run() do end
end

return mod

