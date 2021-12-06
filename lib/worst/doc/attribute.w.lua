
local base = require "lworst/base"
local S = base.Symbol.new

return function(i)

local the_docs

i:define("doc", function(i)
    the_docs = i:quote("doc")
end)

i:define("value-doc", function(i)
    i:stack_push(i:stack_ref_value(1).doc or false)
end)

-- maybe re-resolve this
local default_attributes = i:resolve(S"default-attributes")
i:define("default-attributes", function(i)
    if the_docs then
        local body_val = i:stack_pop_value()
        body_val.doc = the_docs
        the_docs = nil
        i:stack_push(body_val)
    end
    default_attributes(i)
end)

end

