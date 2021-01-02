
import cil/luagen/declare

lua-declare [

    extern "require" (args (modname) pure as lua-require)
    extern "tostring" (args (e) pure as lua-tostring)

    extern "setmetatable" (args (t mt) pure as lua-setmetatable)
    extern "getmetatable" (args (t) pure as lua-getmetatable)

]

export-all

; vi: ft=scheme

