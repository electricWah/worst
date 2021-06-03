
import cil/lua/declare
import cil/lua/interp

define exdef [
    upquote const %idefname
    upquote
    [ cil/lua-interpreter-eval* ] swap list-push eval
    ; interpreter-dump-stack
    lua-load-string
    false? if [ abort ] []
    quote exdefinition definition-add
    quote exdefinition %idefname export-as
]

exdef if [
    interpreter-expect-value
    interpreter-quote
    interpreter-quote
    interpreter-emit-if-then-else
]

exdef loop [
    interpreter-quote
    interpreter-emit-loop
]

exdef clone [ interpreter-expect-value clone ]

exdef swap [
    interpreter-expect-value const a interpreter-expect-value a
]

; and the rest ...

lua-declare "assert" (args (v [msg]))
lua-declare "collectgarbage" (args ([opt] [arg]))
; lua-declare "dofile" (args (filename)) ; returns however many values
lua-declare "error" (args (message [level]))
; lua-declare "_G" (args ()) ; variable
lua-declare "getfenv" (args ([f]) return 1)
lua-declare "getmetatable" (args (object) pure)
lua-declare "ipairs" (args (t) return (ifunc table zero))
; func, nil or nil, error
lua-declare "load" (args (func [chunkname]) return (func err))
lua-declare "loadfile" (args ([filename]) return (func err))
lua-declare "loadstring" (args (string [chunkname]) return (func err))
lua-declare "next" (args (table [index]) return (nextidx value))
lua-declare "pairs" (args (t) return (nextfunc t retnil))
; lua-declare "pcall" (args (f args...)) ; varargs and variable returns
lua-declare "print" (args (str)) ; varargs
lua-declare "rawequal" (args (a b) pure)
lua-declare "rawget" (args (table index) pure)
lua-declare "rawset" (args (table index value)) ; returns table
; lua-declare "select" (args (i ...)) ; what
lua-declare "setfenv" (args (f table))
lua-declare "setmetatable" (args (table meta)) ; returns table
lua-declare "tonumber" (args (e [base]) pure)
lua-declare "tostring" (args (e) pure)
lua-declare "type" (args (v) pure)
; lua-declare "unpack" (args (list [i] [j])) ; returns j-i
; lua-declare "_VERSION" (args ()) ; string
lua-declare "xpcall" (args (f args...)) ; see pcall

lua-declare "coroutine.create" (args (f) return (thread))
; lua-declare "coroutine.resume" (args (c args...)) ; varargs
lua-declare "coroutine.running" (args () return (thread/nil))
lua-declare "coroutine.status" (args (c) return (status))
lua-declare "coroutine.wrap" (args (f) return (func))
; lua-declare "coroutine.yield" (args (values...)) ; varargs

; lua-declare "module" (args (name ...)) ; varargs
lua-declare "require" (args (modname) return 1)
; lua-declare "package.cpath" ; var
; lua-declare "package.loaded" ; table
; lua-declare "package.loaders" ; table
; lua-declare "package.loadlib" ; scary stuff
; lua-declare "package.path" ; var
; lua-declare "package.preload" ; table
; lua-declare "package.seeall" ; module nonsense

; lua-declare "string.byte" (args (s [i] [j])) ; returns j-i values
; lua-declare "string.char" (tupled)
lua-declare "string.dump" (args (f) return 1)
; lua-declare "string.find" (args (s patt [init] [plain]) return _)
; lua-declare "string.format" (args (s ...)) ; varargs
lua-declare "string.gmatch" (args (s pattern) pure)
lua-declare "string.gsub" (args (s pattern repl [n]) pure)
lua-declare "string.len" (args (s) pure)
lua-declare "string.lower" (args (s) pure)
lua-declare "string.match" (args (s pattern [init]) pure)
lua-declare "string.rep" (args (s n) pure)
lua-declare "string.reverse" (args (s) pure)
lua-declare "string.sub" (args (s [i] [j]) pure)
lua-declare "string.upper" (args (s) pure)

lua-declare "table.concat" (args (table [sep] [i] [j]) return (string))
lua-declare "table.insert" (args (table [pos] value))
lua-declare "table.maxn" (args (table) return 1)
lua-declare "table.remove" (args (table [pos]) return (value/nil))
lua-declare "table.sort" (args (table [comp]))

lua-declare "math.abs" (args (x) pure)
lua-declare "math.acos" (args (x) pure)
lua-declare "math.asin" (args (x) pure)
lua-declare "math.atan" (args (x) pure)
lua-declare "math.atan2" (args (y x) pure)
lua-declare "math.ceil" (args (x) pure)
lua-declare "math.cos" (args (x) pure)
lua-declare "math.cosh" (args (x) pure)
lua-declare "math.deg" (args (x) pure)
lua-declare "math.exp" (args (x) pure)
lua-declare "math.floor" (args (x) pure)
lua-declare "math.fmod" (args (x y) pure)
lua-declare "math.frexp" (args (x) pure)
; math.huge ; value
lua-declare "math.ldexp" (args (x) pure)
lua-declare "math.log" (args (x) pure)
lua-declare "math.log10" (args (x) pure)
; lua-declare "math.max" (args (x ...) pure) ; varargs
; lua-declare "math.min" (args (x ...) pure) ; varargs
lua-declare "math.modf" (args (x) pure)
; math.pi ; value
lua-declare "math.pow" (args (x y) pure)
lua-declare "math.rad" (args (x) pure)
lua-declare "math.random" (args ([m] [n]) pure)
lua-declare "math.randomseed" (args (x) pure)
lua-declare "math.sin" (args (x) pure)
lua-declare "math.sinh" (args (x) pure)
lua-declare "math.sqrt" (args (x) pure)
lua-declare "math.tan" (args (x) pure)
lua-declare "math.tanh" (args (x) pure)

lua-declare "io.close" (args ([file]))

lua-declare "io.flush" (args ())
; lua-declare "io.input" (args ([file]) ; returns something only with arg
lua-declare "io.lines" (args ([filename]) return (iterator)) ; iterator
lua-declare "io.open" (args (filename [mode]) return (fh error))
; lua-declare "io.output" (args ([file]) ; same as io.input
lua-declare "io.popen" (args (prog [mode]) return (fh))
lua-declare "io.read" (args ([format]) return (v/nil)) ; not doing multiple formats
lua-declare "io.tmpfile" (args () return (fh))
lua-declare "io.type" (args (obj) return (ty))
; lua-declare "io.write" (args (···)) ; varargs
; file:close ()
; file:flush ()
; file:lines ()
; file:read (···)
; file:seek ([whence] [, offset])
; file:setvbuf (mode [, size])
; file:write (···)

lua-declare "os.clock" (args () return (t))
lua-declare "os.date" (args ([format] [time]) return (d))
lua-declare "os.difftime" (args (t2 t1) return (tdiff))
lua-declare "os.execute" (args ([command]) return (status))
lua-declare "os.exit" (args ([code]))
lua-declare "os.getenv" (args (varname) return (value))
lua-declare "os.remove" (args (filename) return (nil-if-err error))
lua-declare "os.rename" (args (oldname newname) return (nil-if-err error))
lua-declare "os.setlocale" (args (locale [category]) return (locale))
lua-declare "os.time" (args ([table]) return (time))
lua-declare "os.tmpname" (args () return (filename))

; def _swap [ cil/expect-value const a cil/expect-value ]
; quote _swap quote swap export-as

export-all

; vi: ft=scheme

