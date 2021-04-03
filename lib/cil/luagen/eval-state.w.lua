
local interp = ...

local base = require("base")
local List = require("list")

local S = base.Symbol.new


interp:define(S"cil/eval-state-setup", function(i)
    local statements = {}
    -- local arguments = {}
    local gensym = 0
    local indentation = 0
    local indentation_value = "    "

    i:define(S"cil/indent>", function() indentation = indentation + 1 end)
    i:define(S"cil/indent<", function() indentation = indentation - 1 end)

    i:define(S"cil/emit-statement", function(i)
        local stmt = i:stack_pop(List)
        local indent = string.rep(indentation_value, indentation)
        stmt = stmt:push(indent)
        table.insert(statements, stmt)
    end)

    i:define(S"%cil/statements", function(i)
        i:stack_push(List.create(statements))
    end)
end)


