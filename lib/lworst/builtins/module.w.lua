
-- Module-based stuff

local base = require "base"
local List = require "list"
local Port = require "port"
local Map = require "map"
local Reader = require "reader"

local Symbol = base.Symbol
local S = Symbol.new

local cache = Map.empty()

function read_worst(path)
    local f, err = Port.open_input_file(path)
    if f == nil then return nil, err end
    return function(i)
        local l = List.empty()
        while true do
            local v = Reader.read_next(f)
            if v == nil then break end
            l = l:push(v)
        end
        local defs = {}
        i:eval(List.new{})
        i:define("export-as", function(i)
            local new = i:stack_pop(Symbol)
            local orig = i:stack_pop(Symbol)
            local def = i:definition_get(orig)
            defs[new] = def

        end)
        i:define("export-name", function(i)
            local b = i:body_read()
            local def = i:definition_get(b)
            defs[b] = def
        end)
        i:define("export-all", function(i)
            for k, v in pairs(i:definitions()) do
                defs[k] = v
            end
        end)
        i:eval_then(l:reverse(), function(i)
            i:stack_push(Map.new(defs))
        end)
    end
end

function read_lua(path)
    local f, err = Port.open_input_file(path)
    if f == nil then return nil, err end
    local lm = load(f:read("*a"), path)()
    return function(i)
        lm(i)
        i:stack_push(Map.new(i:definitions()))
    end
end

return function(i)

i:define("WORST_LIBPATH", function(i)
    local l = {}
    for entry in string.gmatch(os.getenv("WORST_LIBPATH") or "", "[^:]+") do
        table.insert(l, entry)
    end
    table.insert(l, "%/lib")
    i:stack_push(List.new(l))
end)

i:define("module-resolve", function(i)
    local path = i:stack_pop("string")
    i:call_then(S"WORST_LIBPATH", function(i)
        local paths = i:stack_pop(List)
        local res, err
        for p in List.iter(paths) do
            res, err = read_lua(p.."/"..path..".w.lua")
            if res ~= nil then break end
            res, err = read_worst(p.."/"..path..".w")
            if res ~= nil then break end
        end
        if res ~= nil then
            i:stack_push(res)
        else
            i:stack_push(err)
            i:stack_push(false)
        end
    end)
end)

i:define("module-cache-swap", function(i)
    local newcache = i:stack_pop(Map)
    i:stack_push(cache)
    cache = newcache
end)

i:define("module-import", function(i)
    local name = i:stack_pop("string")

    local defs = cache:get(name)
    if defs ~= nil then
        for k, v in defs:iter() do
            i:define(k, v)
        end
        return
    end

    i:stack_push(name)
    i:call_then(S"module-resolve", function(i)
        local mod = i:stack_pop({List, "function", false})
        if not mod then return i:error("module not found", name) end

        i:eval(List.create{})
        i:eval_then(mod, function(i)
            local defs = i:stack_pop(Map)
            -- print(defs:count(), "defs")
            cache = cache:set(name, defs)
            i:into_parent()
            for k, v in defs:iter() do
                i:define(k, v)
            end
        end)
    end)
end)

end

