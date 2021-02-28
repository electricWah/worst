
local t = require "test"
local port = require "port"
local reader = require "reader"
local base = require "base"
local List = require "list"

local mod = {}

mod["read some symbols"] = function()
    local p = port.InputPort.string("a thingy \n yea ")
    t.check_equal(base.Symbol.new("a"), reader.read_next(p))
    t.check_equal(base.Symbol.new("thingy"), reader.read_next(p))
    t.check_equal(base.Symbol.new("yea"), reader.read_next(p))
    t.check_equal(nil, reader.read_next(p))
end

mod["read a list part"] = function()
    local p = port.InputPort.string("()")
    t.check_equal_with(List.empty(), reader.read_next(p), List.equal)
    t.check_equal(nil, reader.read_next(p))
end

mod["read some list parts"] = function()
    local p = port.InputPort.string("()  {}  \n[]")
    t.check_equal_with(List.empty(), reader.read_next(p), List.equal, "()")
    t.check_equal_with(List.empty(), reader.read_next(p), List.equal, "{}")
    t.check_equal_with(List.empty(), reader.read_next(p), List.equal, "[]")
    t.check_equal(nil, reader.read_next(p))
end

mod["read string"] = function()
    local p = port.InputPort.string("\"awooo\"")
    t.check_equal("awooo", reader.read_next(p))
    t.check_equal(nil, reader.read_next(p))
end

mod["read simple escaped string"] = function()
    local p = port.InputPort.string("\"\\\"\"")
    p.booboo = true
    t.check_equal("\"", reader.read_next(p))
    t.check_equal(nil, reader.read_next(p))
end

mod["read escaping string"] = function()
    local p = port.InputPort.string("\"a b \\\"c\\\" d\"")
    t.check_equal("a b \"c\" d", reader.read_next(p))
    t.check_equal(nil, reader.read_next(p))
end

mod["read strings"] = function()
    local p = port.InputPort.string(" \"awooo\"\"ban\nana\" \"coconut\"")
    t.check_equal("awooo", reader.read_next(p))
    t.check_equal("ban\nana", reader.read_next(p))
    t.check_equal("coconut", reader.read_next(p))
    t.check_equal(nil, reader.read_next(p))
end

mod["read bools"] = function()
    local p = port.InputPort.string("#t #f#t\n#f")
    t.check_equal(true, reader.read_next(p))
    t.check_equal(false, reader.read_next(p))
    t.check_equal(true, reader.read_next(p))
    t.check_equal(false, reader.read_next(p))
    t.check_equal(nil, reader.read_next(p))
end

mod["read numbers"] = function()
    local p = port.InputPort.string(" 4 45\n45.6")
    t.check_equal(4, reader.read_next(p))
    t.check_equal(45, reader.read_next(p))
    t.check_equal(45.6, reader.read_next(p))
    t.check_equal(nil, reader.read_next(p))
end

mod["skip semi comment 1"] = function()
    local p = port.InputPort.string("1;skip\n;skips again")
    t.check_equal(1, reader.read_next(p))
    t.check_equal(nil, reader.read_next(p))
end

mod["skip semi comment 2"] = function()
    local p = port.InputPort.string(" ;skippo\n1 ")
    t.check_equal(1, reader.read_next(p))
    t.check_equal(nil, reader.read_next(p))
end

mod["skip semi comment 3"] = function()
    local p = port.InputPort.string(";skippy\n1; skips\n2 ;also skip 3\n")
    t.check_equal(1, reader.read_next(p), "skippy")
    t.check_equal(2, reader.read_next(p))
    t.check_equal(nil, reader.read_next(p))
end

mod["overeager semi comment"] = function()
    local p = port.InputPort.string("; boing\n\"hello\"\n")
    t.check_equal("hello", reader.read_next(p))
    t.check_equal(nil, reader.read_next(p))
end

mod["shebang comment"] = function()
    local p = port.InputPort.string("1 2 #!3 #\n4 !# 5")
    t.check_equal(1, reader.read_next(p))
    t.check_equal(2, reader.read_next(p))
    t.check_equal(5, reader.read_next(p))
    t.check_equal(nil, reader.read_next(p))
end

mod["escaped string in list"] = function()
    local p = port.InputPort.string("(\"\\\"\")")
    local l = reader.read_next(p)
    t.check_equal(true, List.is(l))
    local e
    l, e = l:pop()
    t.check_equal("\"", e)
    t.check_equal_with(List.empty(), l, List.equal)
    t.check_equal(nil, reader.read_next(p))
end

return mod

