
-- Module-based stuff

local base = require "lworst/base"
local List = require "lworst/list"
local Port = require "lworst/port"
local Reader = require "lworst/reader"
local Interpreter = require "lworst/interpreter"

local Type = base.Type
local Symbol = base.Symbol
local S = Symbol.new

local Export = Type.new("export")
function Export:__tostring() return "<export "..tostring(self.name)..">" end
function interp_export(i, name, def)
    i:pause(setmetatable({ name = name, def = def }, Export))
end
function interp_export_all(i)
    i:pause(setmetatable({ all = true, name = "all", }, Export))
end

function read_worst(path)
    local f, err = Port.open_input_file(path)
    if f == nil then return nil, err end
    return function(i)
        local l = {}
        while true do
            local v = Reader.read_next(f)
            if v == nil then break end
            table.insert(l, v)
        end
        i:eval(List.new(l))
    end
end

function lua_module_loader(mod)
    return function(i)
        local old_defs = i:definitions()
        if base.is_a(mod, "function") then
            mod(i)
            interp_export_all(i)
        else
            -- table
            for name, def in pairs(mod) do
                if old_defs[name] ~= def then
                    interp_export(i, name, def)
                end
            end
        end
    end
end

function read_lua_file(path)
    local f, err = Port.open_input_file(path)
    if f == nil then return nil, err end
    local mod = load(f:read("*a"), path)()
    return lua_module_loader(mod)
end

function read_lua_require(path)
    local ok, res = pcall(require, path)
    if not ok then return nil, res end
    return lua_module_loader(res)
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

function eval_module(parent, mod, name, defs)
    local i = Interpreter.empty()
    for k, v in pairs(defs) do
        i:define(k, v)
    end
    local old_defs = i:definitions()
    local exports = base.Place.new(List.new())
    i:define("current-module", List.new({List.new_pairs({
        [S"name"] = name,
        -- [S"exports"] = exports
    })}))
    -- i:step_into_new()
    local export_all = false
    i:eval_next(mod, name)
    while true do
        local r = i:run()
        if r == nil then
            break
        elseif Export.is(r) then
            if r.all then
                export_all = true
            else
                exports:set(exports:get():add_pair(r.name, r.def))
            end
        else
            return parent:error("module import failed", name, r)
        end
    end
    i:definition_remove(S"current-module")
    if export_all then
        for defname, def in pairs(i:definitions()) do
            if old_defs[defname] ~= def then
                exports:set(exports:get():add_pair(defname, def))
            end
        end
    end
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

    local names = i:quote("export")

    if base.unwrap_lua(names) == true then
        interp_export_all(i)
    elseif Symbol.is(names) then
        local def = i:resolve(names)
        if not def then
            return i:error("export: not defined (symbol)", names)
        end
        interp_export(i, names, def)
    elseif List.is(names) then
        for name in List.iter(names) do
            local def = i:resolve(name)
            if not def then
                return i:error("export: not defined (list)", name)
            end
            interp_export(i, name, def)
        end
    else
        return i:error("cannot export this", names)
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
                    -- TODO check Value.private
                    exports[k] = v
                end
            elseif List.is(export_names) then
                for name, def in List.pairs(export_names) do
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

