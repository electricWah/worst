
# building a bundled binary:
# luarocks install luastatic
# statically link minizip
# move lworsti to lib/lworsti.w and turn lworsti into basic shell script
# luarocks install amalg
# amalg bundle_main.lua minizip.lua {...everything else in lib} > bundle_all.lua
# luajit -b bundle_all.lua lwbundle.o
# luastatic binary using bundle.lua, minizip.a, and lwbundle.o
# ^ DONE
# turn WORST_LIBDIR into WORST_PATH
# bundle.lua:
# - write FFI wrapper for minizip
# - overloads io.open with zip.openfile if mode = "r"
# - set env WORST_LIBPATH = WORST_LIBPATH:$0:$0/lib/worst
#   both to load files from inside the zipped binary
# - run interpreter reading lib/lworsti.w
# ^ DONE
# zip file containing:
#	lib
#	lib/lua		lua_modules/lib/lua/ -- but not if linked in
#	lib/worst	lib/ -- including the lworsti entry point script
#	bin/lworsti	a copy of the original binary for further bundling
# ^ DONE
# concat the binary with the zip

LUA_ROCKS=

MINIZIP_L=-Wl,--whole-archive $(shell pkg-config --libs --static minizip) -Wl,--no-whole-archive
LUAJIT_L=$(shell pkg-config --libs-only-L luajit) -lm -ldl -l:libluajit-5.1.a
CCLIBS=$(MINIZIP_L) $(LUAJIT_L)

LUA_PATH:=$(LUA_PATH);./?.lua;../lib/lworst/?.lua

LWSRCMODULES=base interpreter builtins reader list map port
LWSRCFILES=$(LWSRCMODULES:%=../lib/lworst/%.lua)

.PHONY: all
all: setup $(LUA_MODULES_PATHS) bundle
	@true

lworsti.lua: $(LWSRCFILES) minizip.lua bundle.lua
	eval `luarocks path`; \
	amalg.lua -s bundle.lua minizip $(LWSRCMODULES) -o $@

lworsti.o: lworsti.lua
	luajit -b $< $@

bundle_raw: bundle_entrypoint.lua lworsti.o
	eval `luarocks path`; \
	CC="" luastatic $<
	$(CC) -Os bundle_entrypoint.luastatic.c lworsti.o $(CCLIBS) -rdynamic -o $@

lib: ../lib
	cp -Tru $< $@

bin:
	mkdir -p $@

bin/lworsti: bundle_raw bin
	cp $< $@

libworst.zip: bin/lworsti lib LICENSE
	zip -ru $@ bin/lworsti LICENSE
	zip -r $@ lib -i '*.w'

lworsti: bundle_raw libworst.zip
	cat $^ > $@
	chmod +x $@

.PHONY: setup
setup:
	luarocks init
	luarocks install luastatic
	luarocks install amalg

