
-- Module-based stuff

local base = require "lworst/base"
local List = require "lworst/list"
local Port = require "lworst/port"
local Map = require "lworst/map"
local Reader = require "lworst/reader"
local Interpreter = require "lworst/interpreter"

local Type = base.Type
local Symbol = base.Symbol
local S = Symbol.new

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
        i:eval(l:reverse())
    end
end

function read_lua_file(path)
    local f, err = Port.open_input_file(path)
    if f == nil then return nil, err end
    local lm = load(f:read("*a"), path)()
    return function(i)
        lm(i)
        i:stack_push(Map.new(i:definitions()))
    end
end

function read_lua_require(path)
    local ok, res = pcall(require, path)
    if not ok then return nil, res end
    return function(i)
        res(i)
        i:stack_push(Map.new(i:definitions()))
    end
end

function resolve_module(path, libpath)
    if Symbol.is(path) then
        path = Symbol.unwrap(path)
    end
    local res, err
    for p in List.iter(libpath) do
        res, err = read_lua_file(p.."/"..path..".w.lua")
        if res ~= nil then break end
        res, err = read_worst(p.."/"..path..".w")
        if res ~= nil then break end
    end
    if res == nil then
        res, err = read_lua_require(path)
    end
    if res ~= nil then
        return res, nil
    else
        return nil, err
    end
end

local Export = Type.new("<export>")
function Export.new(name, def)
    return setmetatable({ name = name, def = def }, Export)
end

function eval_module(parent, mod, name, defs)
    local i = Interpreter.empty()
    for k, v in pairs(defs) do
        i:define(k, v)
    end
    i:eval_next(mod, name)
    local exports = {}
    while true do
        local r = i:run()
        if r == nil then
            break
        elseif Export.is(r) then
            exports[r.name] = r.def
        else
            return parent:pause(base.Error.new("module import failed", List.new({name, r})))
        end
    end
    return Map.new(exports)
end

function exporter(i)
    local names = i:quote("export")
    if List.is(names) then
        -- default
    elseif Symbol.is(names) then
        names = List.new({names})
    elseif names == true then
        names = List.new()
        for k, v in pairs(i:definitions()) do
            names = names:push(k)
        end
    else
        return i:error("cannot export this")
    end
    for name in List.iter(names) do
        local def = i:resolve(name)
        if not def then i:error("export", name) end
        i:pause(Export.new(name, def))
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

local import_cache = {}

i:define("import", function(i)
    local names = i:quote("import")

    if List.is(names) then
        -- default
    elseif Symbol.is(names) then
        names = List.new({names})
    elseif base.Type.is("string", names) then
        return i:error("TODO import <string>")
    else
        return i:error("cannot import this")
    end

    i:call(S"WORST_LIBPATH")
    local libpath = i:stack_pop(List)

    local defs = i:all_definitions()
    defs[S"export"] = exporter
    local import_defs = {}
    for im in List.iter(names) do
        if not import_cache[im] then
            local mod, err = resolve_module(im, libpath)
            if not mod then return i:error("import failed", im, err) end
            local exports = eval_module(i, mod, im, defs)
            import_cache[im] = exports
        end
        for k, v in import_cache[im]:iter() do
            import_defs[k] = v
        end
    end
    for k, v in pairs(import_defs) do
        i:define(k, v)
    end
end)

end

