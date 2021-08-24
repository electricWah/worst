
local base = require "lworst/base"
local Type = base.Type
local List = require "lworst/list"

local luabase = require "cil/lua/base"
local luaexpr = require "cil/lua/expr"

local S = base.Symbol.new

return function(i)

i:define(S"cil/lua-interpreter-eval", function(i)
    EvalContext.expect(i, function(i, ectx)
        local interp = i:stack_pop()
        local body = i:stack_pop(List)

        EvalContext.eval(i, body, List.new{}, function(i, outs, ins, stmts)
            for _, v in List.ipairs(ins) do
                local e = luaexpr.method_call(ectx, interp, "stack_pop", true, {})
                luabase.emit_assignment(ectx, {v}, {e}, true)
            end
            for _, s in List.ipairs(stmts) do ectx:emit_statement(s) end
            for _, v in List.ipairs(outs) do
                luaexpr.method_call(ectx, interp, "stack_push", 0, {v})
            end
        end)
    end)
end)

end

