
local base = require "lworst/base"
local List = require "lworst/list"
local Interpreter = require "lworst/interpreter"

local cl = require "compile/lua"

return function(i)

i:define(base.Symbol.new "compile->lua-chunk", function(parent)
    local body = parent:stack_pop(List)

    function body_wrapped(i)
        cl.install_builtins(i, parent)
        local ctx = cl.context(i)
        -- put interp in ctx and wrap it?
        local stmts, args, rets = ctx:evaluate(i, body)
        if List.length(args) > 0 then
            ctx:emit(cl.assignment(args, {base.Symbol.new "..."}, true))
        end
        for _, stmt in List.ipairs(stmts) do
            ctx:emit_raw(stmt)
        end
        if List.length(rets) > 0 then
            ctx:emit("return ", cl.csv(rets))
        end
    end

    local stmts, _args, _rets = cl.evaluate(parent, body_wrapped)
    i:stack_push(table.concat(List.to_table(stmts), "\n"))
end)

end

