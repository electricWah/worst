
local base = require "lworst/base"
local Type = base.Type
local List = require "lworst/list"
local Interpreter = require "lworst/interpreter"

local Expr = require "cil/expr"
local S = base.Symbol.new

local mod = {}

local context_request = {}
function mod.context(i)
    i:pause(context_request)
    return i:stack_pop()
end

function mod.evaluator(ctx, interp, body)
    interp:eval_next(body)
    function iterator()
        while true do
            -- print("run")
            local r = interp:run()
            -- print("ctx", r, ctx)
            if r == context_request then
                interp:stack_push(ctx)
            else
                return r
            end
        end
    end
    return iterator, nil, nil
end

return mod

