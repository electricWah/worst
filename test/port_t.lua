
local t = require "test"
local port = require "lworst/port"

local mod = {}

mod["string port peek/read *a"] = function()
    local src = "abc def gh"
    local s = port.StringPort.new(src)
    t.check_equal(src, s:peek("*a"))

    t.check_equal(src, s:read("*a"))
    t.check_equal("", s:read("*a"))
end

mod["string port write_buf"] = function()
    local s = port.StringPort.new("")
    s:write_buf("abc")
    t.check_equal("abc", s:peek("*a"))
    s:write_buf(" def", " gh")
    t.check_equal("abc def gh", s:peek("*a"))
end

mod["string port peek n"] = function()
    local src = "abc def gh"
    local s = port.StringPort.new(src)
    for i = 0, string.len(src)  - 1 do
        t.check_equal(string.sub(src, 1, i), s:peek(i))
    end
end

mod["string port read n"] = function()
    local src = "abc def gh"
    local s = port.StringPort.new(src)
    t.check_equal("", s:read(0))
    t.check_equal("abc", s:read(3))
    t.check_equal(" def", s:read(4))
    t.check_equal(" gh", s:read(5))
    t.check_equal(nil, s:read(1))
    t.check_equal(nil, s:read(0))
end

mod["string port read *l"] = function()
    local s = port.StringPort.new("hello\nthere\n")
    t.check_equal("hello", (s:read("*l")))
    t.check_equal("there", (s:read("*l")))
    t.check_equal(nil, (s:read("*l")))
end

mod["string port seek"] = function()
    local s = port.StringPort.new("abcdefg")
    t.check_equal(1, s:seek())
    t.check_equal(2, s:seek("cur", 1))
    t.check_equal(1, s:seek("cur", -1))
    t.check_equal("a", s:peek(1))
    t.check_equal(2, s:seek("cur", 1))
    t.check_equal("b", s:peek(1))
end

mod["input port seek"] = function()
    local s = port.InputPort.string("abcdefg")
    t.check_equal(1, s:seek())
    t.check_equal(2, s:seek("cur", 1))
    t.check_equal(1, s:seek("cur", -1))
    t.check_equal("a", s:peek(1))
    t.check_equal(2, s:seek("cur", 1))
    t.check_equal("b", s:peek(1))
end

mod["input port peek n"] = function()
    local s = port.InputPort.string("hello\nthere")
    t.check_equal("", s:peek(0))
    t.check_equal("h", s:peek(1))
    t.check_equal("hello", s:peek(5))
    t.check_equal("hello\nthere", s:peek(11))
end

mod["input port peek *l"] = function()
    local s = port.InputPort.string("hello\nthere")
    t.check_equal("hello", (s:peek("*l")))
    t.check_equal("hello\n", (s:peek("*L")))
end

mod["string port read *l \\n"] = function()
    local s = port.StringPort.new("hello\nthere\n")
    t.check_equal("hello", s:read("*l"))
    t.check_equal("there", s:read("*l"))
    t.check_equal(nil, s:read("*l"))
end

mod["input port read *l \\n"] = function()
    local s = port.InputPort.string("hello\nthere\n")
    t.check_equal("hello", s:read("*l"))
    t.check_equal("there", s:read("*l"))
    t.check_equal(nil, s:read("*l"))
end

mod["input port read *l"] = function()
    local s = port.InputPort.string("hello\nthere")
    t.check_equal("hello", s:read("*l"), "hello")
    t.check_equal("there", (s:peek("*l")), "peek line")
    t.check_equal("there", (s:peek(5)), "peek 5")
    t.check_equal("there", s:read("*l"), "there")
    t.check_equal(nil, s:read("*l"))
end

function iostring(data)
    local filename = "test/testdata.txt"
    local w = io.open(filename, "w+")
    w:write(data)
    w:flush()
    w:close()
    t.check_equal(data, io.open(filename):read("*a"), "iostring")
    return io.open(filename), filename
end

function iocheckall(src, f)
    
end

mod["input port read file *l \\n"] = function()
    local src = "hello\nthere\n"
    local o, fn = iostring(src)
    local p = port.open_input_file(fn)
    local s = port.InputPort.string(src)
    t.check_equal("hello", o:read("*l"), "o1")
    t.check_equal("hello", p:read("*l"), "p1")
    t.check_equal("hello", s:read("*l"), "s1")
    t.check_equal("there", o:read("*l"), "o2")
    t.check_equal("there", p:read("*l"), "p2")
    t.check_equal("there", s:read("*l"), "s2")
    t.check_equal(nil, o:read("*l"), "o3")
    t.check_equal(nil, p:read("*l"), "p3")
    t.check_equal(nil, s:read("*l"), "s3")
end


mod["input port read hello *l"] = function()
    local src = "hello"
    local o, fn = iostring(src)
    local p = port.open_input_file(fn)
    local s = port.InputPort.string(src)
    t.check_equal(o:read(5), "hello", 1)
    t.check_equal(p:read(5), "hello", 2)
    t.check_equal(s:read(5), "hello", 3)
    t.check_equal(o:read(0), nil, 4)
    t.check_equal(p:read(0), nil, 5)
    t.check_equal(s:read(0), nil, 6)
    t.check_equal(o:read("*l"), nil, 7)
    t.check_equal(p:read("*l"), nil, 8)
    t.check_equal(s:read("*l"), nil, 9)
    t.check_equal(o:read(0), nil, 10)
    t.check_equal(p:read(0), nil, 11)
    t.check_equal(s:read(0), nil, 12)
end

mod["input port read \\n *l"] = function()
    local src = "\n"
    local o, fn = iostring(src)
    local p = port.open_input_file(fn)
    local s = port.InputPort.string(src)
    t.check_equal(o:read("*l"), "", 1)
    t.check_equal(p:read("*l"), "", 2)
    t.check_equal(s:read("*l"), "", 3)
    t.check_equal(o:read(0), nil, 4)
    t.check_equal(p:read(0), nil, 5)
    t.check_equal(s:read(0), nil, 6)
    t.check_equal(o:read("*l"), nil, 7)
    t.check_equal(p:read("*l"), nil, 8)
    t.check_equal(s:read("*l"), nil, 9)
end

mod["input port read hello\\n *l"] = function()
    local src = "hello\n"
    local o, fn = iostring(src)
    local p = port.open_input_file(fn)
    local s = port.InputPort.string(src)
    -- t.check_equal(o:read(6), "hello\n", 1)
    -- t.check_equal(p:read(6), "hello\n", 2)
    -- t.check_equal(s:read(6), "hello\n", 3)
    t.check_equal(o:read("*l"), "hello", 1)
    t.check_equal(p:read("*l"), "hello", 2)
    t.check_equal(s:read("*l"), "hello", 3)
    t.check_equal(o:read(0), nil, 4)
    t.check_equal(p:read(0), nil, 5)
    t.check_equal(s:read(0), nil, 6)
    t.check_equal(o:read("*l"), nil, 7)
    t.check_equal(p:read("*l"), nil, 8)
    t.check_equal(s:read("*l"), nil, 9)
end

mod["input port read hello\\nthere\\n *l"] = function()
    local src = "hello\nthere\n"
    local o, fn = iostring(src)
    local p = port.open_input_file(fn)
    local s = port.InputPort.string(src)
    t.check_equal(o:read("*l"), "hello")
    t.check_equal(p:read("*l"), "hello")
    t.check_equal(s:read("*l"), "hello")
    t.check_equal(o:read(0), "")
    t.check_equal(p:read(0), "")
    t.check_equal(s:read(0), "")
    t.check_equal(o:read("*l"), "there")
    t.check_equal(p:read("*l"), "there")
    t.check_equal(s:read("*l"), "there")
    t.check_equal(o:read(0), nil)
    t.check_equal(p:read(0), nil)
    t.check_equal(s:read(0), nil)
    t.check_equal(o:read("*l"), nil)
    t.check_equal(p:read("*l"), nil)
    t.check_equal(s:read("*l"), nil)
    t.check_equal(o:read(0), nil)
    t.check_equal(p:read(0), nil)
    t.check_equal(s:read(0), nil)
end

mod["input port read file *l hello\\nthere"] = function()
    local src = "hello\nthere"
    local o, fn = iostring(src)
    local p = port.open_input_file(fn)
    local s = port.InputPort.string(src)
    repeat
        t.check_equal(o:read(0), p:read(0))
        t.check_equal(o:read(0), s:read(0))
        local expect = o:read("*l")
        local get = p:read("*l")
        local sget = s:read("*l")
        t.check_equal(expect, get, "port")
        t.check_equal(expect, sget, "str")
    until expect == nil
    t.check_equal(o:read(0), p:read(0))
    t.check_equal(o:read(0), s:read(0))
end

mod["string port *l blank line"] = function()
    local s = port.StringPort.new("\n")
    t.check_equal("", s:read("*l"))
    t.check_equal(true, s:is_eof())
    t.check_equal(nil, s:read("*l"))
end

mod["string port *l blank line with peek"] = function()
    local s = port.StringPort.new("\n")
    t.check_equal("", s:peek("*l"), 1)
    t.check_equal("", s:read("*l"), 2)
    t.check_equal(nil, s:peek("*l"), 3)
    t.check_equal(nil, s:read("*l"), 4)
end

mod["input port *l blank line"] = function()
    local s = port.InputPort.string("\n")
    t.check_equal("", s:read("*l"))
    t.check_equal(nil, s:read("*l"))
end

mod["input port *L two blank lines with peek"] = function()
    local s = port.InputPort.string("\n\n")
    t.check_equal("\n", s:peek("*L"), 1)
    t.check_equal("\n", s:read("*L"), 2)
    t.check_equal("\n", s:peek("*L"), 3)
    t.check_equal("\n", s:read("*L"), 4)
    t.check_equal(nil, s:read("*L"), 5)
end

mod["string port peek booboo *l"] = function()
    local s = port.StringPort.new("a\nbb\n")
    t.check_equal("a", s:peek(1))
    t.check_equal("a", s:read("*l"))
    t.check_equal("b", s:peek(1))
    t.check_equal("bb", s:read("*l"))
    t.check_equal(nil, s:read("*l"))
end

mod["input port peek booboo *l"] = function()
    local s = port.InputPort.string("a\nbb\n")
    t.check_equal("a", s:peek(1))
    t.check_equal("a", s:read("*l"))
    t.check_equal("b", s:peek(1))
    t.check_equal("bb", s:read("*l"))
    t.check_equal(nil, s:read("*l"))
end

mod["input port read *l or *L"] = function()
    -- local s1 = port.InputPort.string("a\nbb\nccc\ndddd")
    -- local s2 = port.InputPort.string("a\nbb\nccc\ndddd")
    local s1 = port.StringPort.new("a\nbb\nccc\ndddd")
    local s2 = port.StringPort.new("a\nbb\nccc\ndddd")
    t.check_equal("a", (s1:peek(1)))
    t.check_equal("a", s1:read("*l"))
    t.check_equal("a\n", s2:read("*L"))
    t.check_equal("b", (s1:peek(1)))
    t.check_equal("bb", s1:read("*l"))
    t.check_equal("bb\n", s2:read("*L"))
    t.check_equal("c", (s1:peek(1)))
    t.check_equal("ccc", s1:read("*l"))
    t.check_equal("ccc\n", s2:read("*L"))
    t.check_equal("d", (s1:peek(1)))
    t.check_equal("dddd", s1:read("*l"))
    t.check_equal("dddd", s2:read("*L"))
    t.check_equal(nil, s1:read("*l"))
    t.check_equal(nil, s2:read("*L"))
end

return mod

