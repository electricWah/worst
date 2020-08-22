
local ffi = require "ffi"

ffi.cdef [[

typedef void * unzFile;

unzFile unzOpen(const char *path);

// 0 or -100 for not found
// case sensitivity: 0 = same as OS, 1 = sensitive, 2 = insensitive
int unzLocateFile (unzFile file, const char *szFileName, int iCaseSensitivity);

// 0 or error
int unzOpenCurrentFile (unzFile file);

// 0 for EOF, positive for number of bytes read, negative for error
// -1 (IO error) or other (zlib error)
int unzReadCurrentFile (unzFile file, void *cbuf, unsigned len);

// 1 if EOF or 0 if not
int unzeof (unzFile file);

int unzCloseCurrentFile (unzFile file);
int unzClose (unzFile file);

]]

local null = ffi.new('void*')

local zipfile = {}
local zipentry = {}

function gc_zipfile(zf)
    ffi.C.unzCloseCurrentFile(zf)
    ffi.C.unzClose(zf)
    ffi.gc(zf, nil)
end

function zipfile.open(zippath, path, mode)
    mode = mode or "r"
    if mode ~= "r" then error("zipfile: cannot open in mode " .. mode) end
    local zf = ffi.C.unzOpen(zippath)
    if not zf or zf == null then return nil end
    zf = ffi.gc(zf, gc_zipfile)
    local r = ffi.C.unzLocateFile(zf, path, 0)
    if r < 0 then return nil, r end
    r = ffi.C.unzOpenCurrentFile(zf)
    if r < 0 then return nil, r end
    return setmetatable({
        zipfile = zf
    }, { __index = zipentry })
end

function zipentry:close() gc_zipfile(self.zipfile) end

-- function zipentry:flush() end
-- function zipentry:seek() end
-- function zipentry:setvbuf() end
-- function zipentry:write() end
-- function zipentry:lines() end

-- Read data from the zipfile
function zipentry_read_cbuffer(ze, size)
    if not ze.cbufsize or ze.cbufsize < size then
        ze.cbuf = ffi.new("char[?]", size)
        ze.cbufsize = size
    end
    local r = ffi.C.unzReadCurrentFile(ze.zipfile, ze.cbuf, ze.cbufsize)
    if r < 0 then return nil, r end -- error
    if r == 0 then return nil, nil end -- eof
    return ffi.string(ze.cbuf, r), nil
end

-- Read from the zipfile in chunks
function zipentry_read_cbuffer_large(ze, size, acc)
    size = size or math.huge
    acc = acc or {}
    while size > 0 do
        local chunksize = math.min(size, 1024)
        size = size - chunksize
        local r, err = zipentry_read_cbuffer(ze, chunksize)
        if err then
            return nil, err
        elseif not r then
            break
        else
            table.insert(acc, r)
        end
    end
    local r = table.concat(acc)
    return r
end

-- Read data from the line buffer if it is enabled
function zipentry_read_buffer(ze, size)
    if not ze.lbuf then
        return zipentry_read_cbuffer_large(ze, size)
    elseif #ze.lbuf < size then
        local lb = ze.lbuf
        ze.lbuf = nil
        return zipentry_read_cbuffer_large(ze, size - #lb, {lb})
    end
end

function zipentry_read_line(ze)
    local buf = ze.lbuf or ""
    local err
    local m
    while true do
        m = string.match(buf, ".-\n")
        if m then break end

        buf, err = zipentry_read_buffer(ze, 128)
        if not buf then return nil, err end
        if buf == "" then break end
    end
    if m then
        ze.lbuf = string.sub(buf, #m + 1)
        m = string.sub(m, 1, -2)
    end
    return m
end

function zipentry:read(fmt)
    fmt = fmt or "*l"

    if fmt == "*a" then
        return zipentry_read_buffer(self, math.huge)
    elseif fmt == "*l" then
        local r, err = zipentry_read_line(self)
        return r, err
    elseif type(fmt) == "number" then
        if fmt < 0 then error("invalid fmt " .. fmt) end
        local r, err = zipentry_read_buffer(self, fmt)
    end
    error("format not implemented: " .. fmt)
end

return zipfile

