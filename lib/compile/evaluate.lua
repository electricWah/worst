
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
            local r = interp:run()
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

