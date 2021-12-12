
local mods = {
    "core",
    "interpreter",
    "list",
    "numeric",
    "place",
    "port",
    "string",
    "system",

    "control",
    "module",
    "define",
}

return function(i)
    for _, m in ipairs(mods) do
        require("lworst/builtins/"..m)(i)
    end
    return i
end

