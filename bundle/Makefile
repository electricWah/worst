
# build tools: luarocks install luastatic amalg
# concat all lua: amalg ../lib/**.lua bundle.lua minizip.lua > lworsti.lua
# jit compile: luajit -b lworsti.lua > lworsti.o
# make entrypoint: luastatic bundle_entrypoint.lua > $^.luastatic.c
# compile: cc entrypoint.luastatic.c lworsti.o minizip.a luajit.a > bundle_raw
# zip assets: zip ../lib/**.w bundle_raw > libworst.zip
# combine into final binary: cat bundle_raw libworst.zip > lworsti

# Possible future features:
# include additional luarocks packages or static libraries as build arguments

MINIZIP_L=-Wl,--whole-archive $(shell pkg-config --libs --static minizip) -Wl,--no-whole-archive
LUAJIT_L=$(shell pkg-config --libs-only-L luajit) -lm -ldl -l:libluajit-5.1.a
CCLIBS=$(MINIZIP_L) $(LUAJIT_L)

LUA_PATH:=$(LUA_PATH);./?.lua;../lib/lworst/?.lua

LWSRCMODULES=base interpreter builtins reader list map port main
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

