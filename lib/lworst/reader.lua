
local base = require("base")
local List = require("list")
local Symbol = base.Symbol
local Char = base.Char

local mod = {}

local StringReader = {}
mod.StringReader = StringReader
StringReader.__index = StringReader
function StringReader.new(s)
    return setmetatable({s = s, i = 1}, StringReader)
end

function StringReader:is_eof()
    return self.i >= string.len(self.s)
end

function StringReader:drop(n)
    self.i = self.i + n
end

function StringReader:take(n)
    if self.i >= string.len(self.s) then return nil end
    local s = self.i
    self.i = self.i + n
    return string.sub(self.s, s, s + n - 1)
end

function StringReader:peek()
    if self.i >= string.len(self.s) then return nil end
    return string.sub(self.s, self.i, self.i)
end

function StringReader:match(pat)
    return string.match(self.s, pat, self.i)
end

function mod.read_next(reader)

    function whitespace(m)
        reader:drop(string.len(m))
        return nil
    end

    -- #! -> .+ !#
    function shebang_comment()
        local bang = false
        while true do
            local c = reader:take(1)
            if c == "#" and bang then return nil end
            bang = (c == "!")
        end
    end

    function semi_comment(n)
        reader:drop(string.len(n))
        return nil
    end

    function dquote_string()
        reader:drop(1)
        local str = ""
        local esc = false
        while true do
            local c = reader:take(1)
            if c == nil then error("unmatched \" string") end
            if c == "\"" and not esc then break end
            esc = (c == "\\")
            str = str .. c
        end

        str = string.gsub(str, "\\(.)", {
            ["\""] = "\"",
            ["n"] = "\n",
            ["e"] = "\027",
        })
        return str
    end

    function symbol(s)
        reader:drop(string.len(s))
        return Symbol.new(s)
    end

    function boolean(s)
        reader:drop(2)
        return (s == "#t")
    end

    function base10_number(s)
        reader:drop(string.len(s))
        return tonumber(s)
    end

    function char_octal(s)
        reader:drop(string.len(s))
        return Char.of_int(tonumber(string.sub(s, 3), 8))
    end

    function char_single(s)
        reader:drop(2)
        return Char.of_str(reader:take(1))
    end

    local list_kinds = {
        ["("] = { list = true, kind = "()", start = true },
        [")"] = { list = true, kind = "()", start = false },
        ["{"] = { list = true, kind = "{}", start = true },
        ["}"] = { list = true, kind = "{}", start = false },
        ["["] = { list = true, kind = "[]", start = true },
        ["]"] = { list = true, kind = "[]", start = false },
    }
    function is_list_start(v)
        return type(v) == "table" and v.list and v.start
    end
    function is_list_end(v)
        return type(v) == "table" and v.list and not v.start
    end

    function start_list(ty)
        local kind = list_kinds[ty]
        if not kind then error("not a list delimiter: ", ty) end
        reader:drop(1)
        return kind
    end

    function end_list(ty)
        local kind = list_kinds[ty]
        if not kind then error("not a list delimiter: ", ty) end
        reader:drop(1)
        return kind
    end

    local match_lut = {
        {"^%s+", whitespace},
        {"^#!", shebang_comment},
        {"^;[^\n]*", semi_comment},
        {"^\"", dquote_string},
        {"^[%(%{%[]", start_list},
        {"^[%)%}%]]", end_list},
        {"^#[tf]", boolean},
        {"^#\\[0-7][0-7][0-7]", char_octal},
        {"^#\\.", char_single},
        {"^[%d]+%.[%d]+", base10_number},
        {"^[%d]+", base10_number},
        {"^[^%s%(%)%[%]%{%}\"]+", symbol},
    }

    function read_token()
        local match_i = 1
        while true do
            if reader:is_eof() then return nil end
            local match = match_lut[match_i]
            if not match then error("unknown syntax") end
            local pat, f = unpack(match)
            local m = reader:match(pat)
            -- print("token", pat, string.format("%q", m))
            if m ~= nil then
                if not f then error("unimplemented: " .. pat) end
                local v, unconsumed = f(m)
                if v ~= nil then
                    return v
                elseif unconsumed then
                    match_i = match_i + 1
                else
                    match_i = 1
                end
            else
                match_i = match_i + 1
            end
        end
    end

    function valuify_token(tok)
        function read_until_list_end(ty)
            local acc = {}
            while true do
                local t = read_token()
                if t == nil then
                    error("unmatched opening " .. ty)
                elseif is_list_end(t) then
                    if t.kind == ty then
                        return List.create(acc)
                    else
                        error("expected " .. ty .. " but got " .. t.kind)
                    end
                else
                    table.insert(acc, valuify_token(t))
                end
            end
        end

        if tok == nil then
            return nil
        elseif is_list_start(tok) then
            return read_until_list_end(tok.kind)
        elseif is_list_end(tok) then
            error("unmatched closing " .. ty)
        else
            return tok
        end
    end

    return valuify_token(read_token())

end

return mod

