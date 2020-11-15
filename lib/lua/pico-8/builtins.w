
lua-declare [
    extern "clip" (tupled args ([x y w h] [inner]))

    extern "pget" (tupled args (x y) return 1)
    extern "pset" (tupled args (x y c))
    extern "sget" (tupled args (x y) return 1)
    extern "sset" (tupled args (x y c))
    extern "fget" (tupled args (n [f]) return 1)
    extern "fset" (tupled args (n [f] v))

    extern "print" (tupled args (str [x y] [c]))

    extern "cursor" (tupled args (x y [c]))

    extern "color" (tupled args ([c]))

    extern "cls" (tupled args ([c]))

    extern "camera" (tupled args ([x y]))

    extern "circ" (tupled args (x y r [c]))
    extern "circfill" (tupled args (x y r [c]))
    extern "oval" (tupled args (x0 y0 x1 y1 [c]))
    extern "ovalfill" (tupled args (x0 y0 x1 y1 [c]))
    extern "rect" (tupled args (x0 y0 x1 y1 [c]))
    extern "rectfill" (tupled args (x0 y0 x1 y1 [c]))

    extern "line" (tupled args (x0 y0 [x1 y1] [c]))

    extern "pal" (tupled args (c0/t [c1] [p]))
    extern "palt" (tupled args ([c] [p]))

    extern "spr" (tupled args (n x y [w h] [flip_x] [flip_y]))

    extern "sspr" (tupled args (sx sy sw sh dx dy [dw dh] [flip_x] [flip_y]))

    extern "fillp" (tupled args (p))

    extern "add" (tupled args (t v [i]) as tadd)
    extern "del" (tupled args (t [v]) return 1 as tdel)
    extern "deli" (tupled args (t [i]) return 1 as tdeli)
    extern "count" (tupled args (t [v]) return 1 as tcount)
    ; extern "all" (args (t) pure) ; pure hack for for-loops
    ; extern "foreach" (args (t f))
    ; TODO pairs?

    extern "btn" (tupled args ([i] [p]))
    extern "btnp" (tupled args ([i] [p]))

	extern "sfx" (tupled args (n [channel] [offset] [length]))
	extern "music" (tupled args ([n] [fade_len] [channel_mask]))

    extern "mget" (tupled args (x y) return 1)
    extern "mset" (tupled args (x y v))

	extern "map" (tupled args (cell_x cell_y sx sy cell_w cell_h [layers]))
	extern "tline" (tupled args (x0 y0 x1 y1 mx my [mdx mdy] [layers]))

    extern "peek" (tupled args (addr) return 1)
    extern "poke" (tupled args (addr v) return 1)
    extern "peek2" (tupled args (addr) return 1)
    extern "poke2" (tupled args (addr v) return 1)
    extern "peek4" (tupled args (addr) return 1)
    extern "poke4" (tupled args (addr v) return 1)

	extern "memcpy" (tupled args (dest_addr source_addr len))
	extern "reload" (tupled args (dest_addr source_addr len [filename]))
	extern "cstore" (tupled args (dest_addr source_addr len [filename]))
	extern "memset" (tupled args (dest_addr val len))

	extern "max" (tupled args (x y) pure)
	extern "min" (tupled args (x y) pure)
	extern "mid" (tupled args (x y z) pure)
	extern "flr" (tupled args (x) pure)
	extern "ceil" (tupled args (x) pure)
	extern "cos" (tupled args (x) pure)
	extern "sin" (tupled args (x) pure)
	extern "atan2" (tupled args (dx dy) pure)
	extern "sqrt" (tupled args (x) pure)
	extern "abs" (tupled args (x) pure)
	extern "rnd" (tupled args (x) return 1)
	extern "srand" (tupled args (x))

    extern "band" (tupled args (x y) pure)
    extern "bor" (tupled args (x y) pure)
    extern "bxor" (tupled args (x y) pure)
    extern "bnot" (tupled args (x) pure)
    extern "shl" (tupled args (x n) pure)
    extern "shr" (tupled args (x n) pure)
    extern "lshr" (tupled args (x n) pure)
    extern "rotl" (tupled args (x n) pure)
    extern "rotr" (tupled args (x n) pure)
    ; Operator versions are also available: & | ^^ ~ << >> >>> <<> >><

	extern "menuitem" (tupled args (index [label callback]))

    extern "tostr" (tupled args (val [use_hex]))
    extern "tonum" (tupled args (str))
	extern "chr" (tupled args (num))
	extern "ord" (tupled args (str [index]))
	extern "sub" (tupled args (str pos0 [pos1]))
	extern "split" (tupled args (str [separator] [convert_numbers] ))

	extern "cartdata" (tupled args (id))
	extern "dget" (tupled args (index))
	extern "dset" (tupled args (index value))

    extern "serial" (tupled args (channel address length))
    extern "stat" (tupled args (i) return 1)


    ; Lua
    ; extern "type" (tupled args (val))
    ; setmetatable t, m
    ; getmetatable t
    ; rawset t key value
    ; rawget t key
    ; rawequal t1 t2
    ; rawlen t

    ; cocreate f
    ; coresume c [p0 p1 ..]
    ; costatus c
    ; yield
]

		; @ADDR  -- PEEK(ADDR)
		; %ADDR  -- PEEK2(ADDR)
		; $ADDR  -- PEEK4(ADDR)
		; PRINT(9\2) -- result:4  equivalent to flr(9/2)

export-all

; vi: ft=scheme

