
local i = ...

local base = require("base")
local Type = base.Type
local List = require("list")

local cil = require("cil/base")
local EvalContext = cil.EvalContext

local luabase = require("cil/lua/base")

local S = base.Symbol.new

-- TODO decide how this interacts with cil/eval-args
-- maybe it just calls it and grabs stuff off the stack
-- to emit the args/returns after

function flatten (args)
    local sb = {}
    for _, v in List.ipairs(args) do
        table.insert(sb, luabase.value_tostring_prec(v))
    end
    return sb
end

i:define(S"cil/eval->lua-chunk", function(i)
    local body = i:stack_pop(List)
    EvalContext.eval(i, body, List.new{}, function(i, stack, args, stmts)
        local sb = {}
        local initargs = luabase.assignment(args, {S"..."}, true)

        if initargs then table.insert(sb, table.concat(List.to_table(initargs))) end
        for _, stmt in ipairs(stmts) do
            table.insert(sb, table.concat(List.to_table(stmt)))
        end

        if stack:length() > 0 then
            table.insert(sb, "return " .. table.concat(flatten(stack), ", "))
        end
        i:stack_push(table.concat(sb, "\n"))

    end)
end)

