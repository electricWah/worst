
local mods = {
    "core",
    "definition",
    "interpreter",
    "list",
    "map",
    "module",
    "numeric",
    "place",
    "port",
    "string",
    "system",
}

return function(i)
    for _, m in ipairs(mods) do
        require("lworst/builtins/"..m)(i)
    end
    return i
end

