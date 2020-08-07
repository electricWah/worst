
local base = require("base")
local Error = base.Error
local Type = base.Type

local mod = {}

local Mode = Type.new("mode")

local Eof = Type.new("eof")
local EOF = setmetatable({}, Eof)
mod.Eof = Eof
mod.EOF = EOF

local InputPort = Type.new("input-port")
mod.InputPort = InputPort
function InputPort.file(fh)
    fh:setvbuf("line")
    return setmetatable({
        fh = fh,
        mode = "file",
        buf = "",
        bufi = 1,
    }, InputPort)
end

local STDIN = nil

function InputPort.stdin()
    io.stdin:setvbuf("line")
    if not STDIN then
        STDIN = setmetatable({
            fh = io.stdin,
            mode = "stdin",
            buf = "",
            bufi = 1,
        }, InputPort)
    end
    return STDIN
end

InputPort.__tostring = function(p)
    return "InputPort(" .. p.mode .. ")"
end

function InputPort:fill(n)
    while n > string.len(self.buf) - self.bufi do
        local more = self.fh:read()
        if more == nil then return false end
        self.buf = string.sub(self.buf, self.bufi) .. more .. "\n"
        self.bufi = 1
    end
    return true
end

function InputPort:buffer_size()
    return string.len(self.buf) + 1 - self.bufi
end

function InputPort:is_eof()
    if self:buffer_size() == 0 then
        local more = self.fh:read()
        if more == nil then return true end
        self.buf = string.sub(self.buf, self.bufi) .. more .. "\n"
        self.bufi = 1
        return self:buffer_size() == 0
    else
        return false
    end
end

function InputPort:drop(n)
    if self:is_eof() then return nil end
    self.bufi = self.bufi + n
end

function InputPort:take(n)
    if self:is_eof() then return nil end
    local i = self.bufi
    self.bufi = i + n
    return string.sub(self.buf, i, i + n - 1)
end

function InputPort:peek()
    if self:is_eof() then return nil end
    return string.sub(self.buf, self.bufi, self.bufi)
end

function InputPort:match(pat)
    if self:is_eof() then return nil end
    return string.match(self.buf, pat, self.bufi)
end

function InputPort:close() self.fh:close() end

local OutputPort = Type.new("output-port")
mod.OutputPort = OutputPort

local STDOUT = nil

function OutputPort.stdout()
    if not STDOUT then
        STDOUT = setmetatable({
            fh = io.stdout,
            mode = "stdout"
        }, OutputPort)
    end
    return STDOUT
end

function OutputPort.file(fh)
    local p = setmetatable({
        fh = fh,
        mode = "file"
    }, OutputPort)
    return p
end

function OutputPort:close() self.fh:close() end

function OutputPort:write_string(s)
    self.fh:write(s)
end

mod.open_input_file = function(path)
    local fh, err = io.open(path, "rb")
    if fh == nil then
        Error.raise("open-input-file", err)
    else
        return InputPort.file(fh)
    end
end

mod.open_output_file = function(path)
    local fh, err = io.open(path, "wb")
    if fh == nil then
        Error.raise("open-output-file", err)
    else
        return OutputPort.file(fh)
    end
end

function mod.stdin() return InputPort.stdin() end
function mod.stdout() return OutputPort.stdout() end
function mod.stderr() return OutputPort.stderr() end

return mod

