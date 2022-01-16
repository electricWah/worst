
local t = require "test"
local base = require "lworst/base"
local List = require "lworst/list"

local mod = {}

mod["new equal"] = function()
    t.check_equal(List.new(), List.new{})
end

mod["new round-trip"] = function()
    t.check_equal(List.new({1, 2, 3}), List.new(List.to_table(List.new({1, 2, 3}))))
end

mod["indexing"] = function()
    local l = List.new({1, 2, 3})
    t.check_equal(1, l:index(0))
    t.check_equal(2, l:index(1))
    t.check_equal(3, l:index(2))
end

mod["value"] = function()
    t.check_equal(base.value(List.new{}), List.new{})
end

return mod

