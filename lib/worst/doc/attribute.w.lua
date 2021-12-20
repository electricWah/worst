
local base = require "lworst/base"
local S = base.Symbol.new

return function(i)

local the_docs

i:define("doc", function(i)
    the_docs = i:quote("doc")
end)

i:define("value-doc", function(i)
    i:stack_push(base.meta.get(i:stack_ref(1), "doc") or false)
end)

-- maybe re-resolve this
local default_attributes = i:resolve(S"default-attributes")
i:define("default-attributes", function(i)
    if the_docs then
        i:stack_push(base.meta.set(i:stack_pop(), "doc", the_docs))
        the_docs = nil
    end
    default_attributes(i)
end)

end

