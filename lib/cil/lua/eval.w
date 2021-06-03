
define eval->lua [
    upquote const %luabody
    import cil/lua/stdlib
    %luabody
    cil/eval->lua-chunk
]

export-all

; vi: ft=scheme

