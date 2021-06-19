
local base = require "lworst/base"
local List = require "lworst/list"
local Symbol = base.Symbol

local mod = {}

function mod.read_next(port)

    function whitespace(p)
        local consumed = false
        while true do
            local l = p:peek("*L")
            if not l then break end
            local s, e = string.find(l, "^%s+")
            if e then
                assert(p:seek("cur", e))
                consumed = true
            else
                break
            end
        end
        return consumed
    end

    function semi_comment(p)
        if p:peek(1) == ";" then
            p:read("*L")
            return true
        else
            return false
        end
    end

    function shebang_comment(p)
        if p:peek(1) ~= "#" or p:peek(2) ~= "#!" then return false end
        p:seek("cur", 2)

        while true do
            local l = p:peek("*L")
            if not l then break end
            local s, e = string.find(l, "!#")
            if e then
                assert(p:seek("cur", e))
                break
            else
                p:seek("cur", string.len(l))
            end
        end
        return true
    end

    function symbol(p)
        local l = p:peek("*L")
        if not l then return nil end
        local s, e = string.find(l, "^[^%s%(%)%[%]%{%}\"]+")
        if e then
            return Symbol.new(p:read(e))
        end
    end

    function dquote_string(p)
        if p:peek(1) ~= "\"" then return end

        p:seek("cur", 1)
        local buf = {}

        -- look for " not preceded by \
        while true do
            local l = p:peek("*L")
            if not l then return nil end

            local escaping = false
            local ss
            for si = 1, string.len(l) do
                local c = string.sub(l, si, si)
                if escaping then
                    escaping = false
                elseif c == "\\" then
                    escaping = true
                elseif c == "\"" then
                    ss = si
                    break
                end
            end
            if ss then
                -- found an unescaped "
                table.insert(buf, p:read(ss - 1))
                p:seek("cur", 1)
                break
            else
                table.insert(buf, p:read("*L"))
            end
        end
        
        local str = table.concat(buf)

        str = string.gsub(str, "\\(.)", {
            ["\""] = "\"",
            ["n"] = "\n",
            ["t"] = "\t",
            ["e"] = "\027",
        })
        str = string.gsub(str, "\\(x%x+)", function(v)
            return string.char(tonumber(string.sub(v, 2), 16))
        end)

        return str
    end

    function boolean(p)
        local s = p:peek(2)
        if s == "#t" then
            p:seek("cur", 2)
            return true
        elseif s == "#f" then
            p:seek("cur", 2)
            return false
        end
    end

    function base10_number(p)
        local l = p:peek("*L")
        if not l then return end

        local s, e = string.find(l, "^%d+%.%d+")
        if e then
            p:seek("cur", e)
            return tonumber(string.sub(l, 1, e))
        end

        s, e = string.find(l, "^%d+")
        if e then
            p:seek("cur", e)
            return tonumber(string.sub(l, 1, e))
        end

    end

    local list_kinds = {
        ["("] = { kind = "()", start = true },
        [")"] = { kind = "()", start = false },
        ["{"] = { kind = "{}", start = true },
        ["}"] = { kind = "{}", start = false },
        ["["] = { kind = "[]", start = true },
        ["]"] = { kind = "[]", start = false },
    }
    for k, v in pairs(list_kinds) do v.char = k end
    function is_list_start(v)
        return type(v) == "table" and list_kinds[v.char] == v and v.start
    end
    function is_list_end(v)
        return type(v) == "table" and list_kinds[v.char] == v and not v.start
    end

    function list_delimiter(p)
        local c = p:peek(1)
        local l = list_kinds[c]
        if l then
            assert(p:seek("cur", 1))
            return l
        end
    end

    local matchers = {
        boolean,
        list_delimiter,
        dquote_string,
        base10_number,
        symbol,
    }

    function read_blanks(p)
        while whitespace(p) or semi_comment(p) or shebang_comment(p) do end
    end

    function read_token()
        if not port:read(0) then return nil end
        for i, matcher in ipairs(matchers) do
            read_blanks(port)
            local r = matcher(port)
            if r ~= nil then return r end
        end
        return nil
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

        if is_list_start(tok) then
            return read_until_list_end(tok.kind)
        elseif is_list_end(tok) then
            error("unmatched closing " .. tok.kind)
        else
            return tok
        end
    end

    return valuify_token(read_token())

end

return mod

