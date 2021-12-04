
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
    local exports = base.Place.new(Map.new())
    i:define("current-module", List.new({Map.new({
        [S"name"] = name,
        [S"exports"] = exports
    })}))
    -- i:step_into_new()
    i:eval_next(mod, name)
    while true do
        local r = i:run()
        if r == nil then
            break
        else
            return parent:error("module import failed", name, r)
        end
    end
    i:definition_remove(S"current-module")
    return i, exports:get()
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

function export_def(i)
    if not i:resolve(S"current-module") then
        return i:error("export: not in a module!")
    end
    i:call(S"current-module")
    local current_module = i:stack_pop(Map)

    local exports_place = current_module:get(S"exports")
    local exports = exports_place:get()

    if exports == true then
        return i:error("export: already did export #t!")
    end

    local names = i:quote("export")

    if names == true then
        if exports:count() > 0 then
            return i:error("export #t after exporting specific names", exports:keys())
        end
        exports_place:set(true)
    elseif Symbol.is(names) then
        local def = i:resolve_value(names)
        if not def then
            return i:error("export: not defined", names)
        end
        exports_place:set(exports:set(names, def))
    elseif List.is(names) then
        for name in List.iter(names) do
            local def = i:resolve_value(name)
            if not def then
                return i:error("export: not defined", name)
            end
            exports = exports:set(name, def)
        end
        exports_place:set(exports)
    else
        return i:error("cannot export this")
    end
end

i:define("import", function(i)
    local names = i:quote("import")

    if List.is(names) then
        -- default
    elseif Symbol.is(names) then
        names = List.new({names})
    else
        return i:error("cannot import this", names) -- yet?
    end

    i:call(S"WORST_LIBPATH")
    local libpath = i:stack_pop(List)

    local defs = i:all_definitions()
    defs[S"export"] = export_def
    local import_defs = {}
    for im in List.iter(names) do
        -- set import_cache as it goes?
        if not import_cache[im] then
            import_cache[im] = {}
            local exports = import_cache[im]
            local mod, err = resolve_module(im, libpath)
            if not mod then return i:error("import failed", im, err) end
            local interp, export_names = eval_module(i, mod, im, defs)
            if export_names == true then
                for k, v in pairs(interp:all_definitions()) do
                    -- TODO change this to check if defined module == this
                    -- and not Value.private
                    if not defs[k] then -- and defs[k] == v then
                        exports[k] = v
                    end
                end
            elseif Map.is(export_names) then
                for name, def in Map.iter(export_names) do
                    exports[name] = def
                end
            elseif List.is(export_names) then
                for name in List.iter(export_names) do
                    local def = interp:resolve_value(name)
                    -- pause and expect something on stack?
                    if not def then
                        i:error("export: not defined", name)
                    end
                    exports[name] = def
                end
            else
                return i:error("export_names bug", export_names)
            end
        end
        for k, v in pairs(import_cache[im]) do
            i:define(k, v)
        end
    end
end)

end

