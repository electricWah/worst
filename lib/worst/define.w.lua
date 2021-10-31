
local base = require "lworst/base"

local S = base.Symbol.new

-- define #t name [ body ... ] maybe? for export?
-- define (attributes) name [ body ... ]

return function(i)

i:define("define", function(i)
    local what = i:quote(S"define")
    if base.Symbol.is(what) then
        local body = i:quote(S"define")
        i:define(what, body)
    else
        i:error("todo clever define")
    end
end)

end

