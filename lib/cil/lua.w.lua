
local base = require "lworst/base"
local List = require "lworst/list"
local Interpreter = require "lworst/interpreter"

local luabase = require "cil/lua/base"
local luaeval = require "cil/lua/eval"
local luabuiltins = require "cil/lua/builtins"

local S = base.Symbol.new

return function(i)

i:define(S"cil/eval->lua-chunk", function(i)
    local body = i:stack_pop(List)
    local interp = Interpreter.empty()
    luabuiltins(interp)

    local stmts, args, rets = luaeval.evaluate(i, body, interp)

    local sb = {}
    local initargs = luabase.assignment(args, {S"..."}, true)

    if initargs then table.insert(sb, table.concat(List.to_table(initargs))) end
    for _, stmt in List.ipairs(stmts) do
        table.insert(sb, table.concat(List.to_table(stmt)))
    end

    if rets:length() > 0 then
        local ret = {"return "}
        luabase.csv_into(ret, rets)
        table.insert(sb, table.concat(ret))
    end
    i:stack_push(table.concat(sb, "\n"))

end)

i:define("eval->lua", function(i)
    i:stack_push(i:quote())
    i:call(S"cil/eval->lua-chunk")
end)

end

