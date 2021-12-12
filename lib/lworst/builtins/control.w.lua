
local base = require "lworst/base"
local List = require "lworst/list"
local Port = require "lworst/port"
local Reader = require "lworst/reader"

local Symbol = base.Symbol
local S = Symbol.new

-- This is in lua because (a) fast and (b) it's loaded before lexical define

return function(i)

-- [ quote quote quote uplevel uplevel ] quote upquote definition-add
i:define("upquote", function(i)
    if not i:into_parent() then
        i:error("root-uplevel")
    else
        i:stack_push(i:quote("upquote"))
    end
end)

-- define const [
--     [quote] swap list-push list-reverse
--     upquote
--     quote definition-add uplevel
-- ]
i:define("const", function(i)
    local c = i:stack_pop()
    local name = i:quote("const")
    i:define(name, function(i)
        i:stack_push(c)
    end)
end)

-- ; bool if [if-true] [if-false]
-- define if [
--     upquote upquote
--     ; cond true false => false true cond
--     swap dig
--     quote swap when drop
--     quote eval uplevel
-- ]
i:define("if", function(i)
    local ift = i:quote("if")
    local iff = i:quote("if")
    local cond = i:stack_pop("boolean")
    if cond then
        i:eval(ift)
    else
        i:eval(iff)
    end
end)

-- ; while [-> bool] [body ...]
-- define while [
--     upquote quote %%cond definition-add
--     upquote quote %%while-body definition-add
--     [
--         %%cond if [%%while-body %%loop] [[]] current-context-set-code
--     ] const %%loop
--     %%loop current-context-set-code
-- ]
i:define("while", function(i)
    local cond = i:quote("while")
    local body = i:quote("while")
    while true do
        i:eval(cond)
        if not i:stack_pop("boolean") then break end
        i:eval(body)
    end
end)

end

