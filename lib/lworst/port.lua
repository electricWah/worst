
local io = require "io"
local base = require "lworst/base"
local Error = base.Error
local Type = base.Type

local mod = {}

local Eof = Type.new("eof")
function Eof.__tostring() return "<EOF>" end
local EOF = setmetatable({}, Eof)
mod.Eof = Eof
mod.EOF = EOF

local StringPort = Type.new("string-port")
mod.StringPort = StringPort
function StringPort.new(src)
    return setmetatable({ src = src, i = 1 }, StringPort)
end
function StringPort:close() end

function StringPort:is_eof()
    return self.i > string.len(self.src)
end

function StringPort:read(fmt)
    local i = self.i
    if fmt == 0 then
        if self:is_eof() then return nil else return "" end
    elseif type(fmt) == "number" then
        if self:is_eof() then return nil end
        self.i = i + fmt
        return string.sub(self.src, i, i + fmt - 1)
    elseif fmt == "*a" then
        if self:is_eof() then return "" end
        self.i = string.len(self.src) + 1
        return string.sub(self.src, i)
    elseif fmt == "*l" or fmt == "*L" then
        if self:is_eof() then return nil end
        local e = string.find(self.src, "\n", i, true)
        if not e then
            self.i = string.len(self.src) + 1
        else
            self.i = e + 1
            if fmt == "*l" then e = e - 1 end
        end
        return string.sub(self.src, i, e)
    else
        return nil, "unsupported read format"
    end
end

function StringPort:peek(fmt)
    local i = self.i
    local r, e = self:read(fmt)
    self.i = i
    return r, e
end

function StringPort:seek(whence, offs, saturating)
    whence = whence or "cur"
    offs = offs or 0
    local i
    if whence == "cur" and offs == 0 then
        return self.i
    elseif whence == "cur" then
        i = self.i + offs
    elseif whence == "set" then
        i = offs
    elseif whence == "end" then
        i = string.len(self.src) + offs
    else
        return nil, "unknown whence"
    end
    if not saturating and (i < 1 or i > string.len(self.src)) then
        return nil, "out of range"
    else
        self.i = math.min(string.len(self.src) + 1, math.max(1, i))
        return self.i, i ~= self.i
    end
end

function StringPort:write_buf(...)
    self.src = self.src .. table.concat({ ... })
end
function StringPort:write(...)
    self:write_buf(...)
    self.i = string.len(self.src)
end

function StringPort:truncate()
    if self:is_eof() then
        self.src = ""
    else
        self.src = string.sub(self.src, self.i + 1)
    end
    self.i = 1
end

local InputPort = Type.new("input-port")
mod.InputPort = InputPort
function InputPort:__tostring()
    return "InputPort(" .. self.name .. ")"
end

function InputPort.new(src, name)
    return setmetatable({
        src = src,
        name = name,
        buf = StringPort.new(""),
    }, InputPort)
end

local STDIN = nil
function InputPort.stdin()
    if not STDIN then
        STDIN = InputPort.new(io.stdin, "stdin")
    end
    return STDIN
end

function InputPort.string(s)
    local p = InputPort.new(StringPort.new(s), "<string>")
    return p
end

function InputPort.file(fh, path)
    return InputPort.new(fh, string.format("%q", path))
end

function InputPort:close() self.src:close() end

function InputPort:peek(fmt)
    if fmt == 0 then
        if self.buf:is_eof() and not self.src:read(0) then
            return nil
        else
            return ""
        end
    elseif type(fmt) == "number" then
        local r = self.buf:peek(fmt) or ""
        local l = string.len(r)

        if l < fmt then
            local rr = self.src:read(fmt - l)
            if rr then
                self.buf:write_buf(rr)
            end
        end
        return self.buf:peek(fmt)
    elseif fmt == "*a" then
        self.buf:write_buf(self.src:read("*a"))
        return self.buf:peek("*a")
    elseif fmt == "*l" or fmt == "*L" then
        local r = self.buf:peek("*L")
        if not r or string.sub(r, -1) ~= "\n" then
            local rr = self.src:read("*l")
            if rr then
                self.buf:write_buf(rr, "\n") -- , self.src:read(0) and "\n")
            end
        end
        return self.buf:peek(fmt)
    else
        return nil
    end
end

function InputPort:has_buffered_data()
    return self.buf:read(0) ~= nil
end

function InputPort:read(fmt)
    if fmt == 0 then
        return self.buf:read(0) or self.src:read(0)
    elseif type(fmt) == "number" then
        local r = self.buf:read(fmt)
        if not r then
            return self.src:read(fmt)
        else
            return r
        end
    elseif fmt == "*a" then
        return self.buf:read("*a") .. self.src:read("*a")
    elseif fmt == "*l" or fmt == "*L" then
        local rl = self.buf:peek("*L")
        if not rl or string.sub(rl, -1) ~= "\n" then
            local rs = self.src:read("*l")
            if rs then
                self.buf:write_buf(rs, "\n")
            end
        end
        return self.buf:read(fmt)
    else
        return nil
    end
end

function InputPort:seek(whence, offs)
    whence = whence or "cur"
    offs = offs or 0
    if self.buf:is_eof() then
        return self.src:seek(whence, offs)
    elseif whence == "cur" and offs > 0 then
        local c = self.buf:seek()
        local o, saturated = self.buf:seek("cur", offs, true)
        if saturated then
            self.buf:truncate()
            return self.src:seek("cur", offs - o)
        else
            return o
        end
    else
        return nil, "unsupported seek with non-empty buffer"
    end
end


local OutputPort = Type.new("output-port")
mod.OutputPort = OutputPort

function std_output_port(name)
    local port = nil
    return function()
        if not port then
            port = setmetatable({
                fh = io[name],
                mode = name
            }, OutputPort)
        end
        return port
    end
end
OutputPort.stdout = std_output_port("stdout")
OutputPort.stderr = std_output_port("stderr")

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
    local fh, err = io.open(path, "r")
    if not fh then
        return nil, err
    else
        return InputPort.file(fh, path), nil
    end
end

mod.open_output_file = function(path)
    local fh, err = io.open(path, "w")
    if not fh then
        return nil, err
    else
        return OutputPort.file(fh), nil
    end
end

function mod.stdin() return InputPort.stdin() end
function mod.stdout() return OutputPort.stdout() end
function mod.stderr() return OutputPort.stderr() end

return mod



