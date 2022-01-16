#!/bin/sh
# profiling: -jp -jp=s -jp=l2 -jp=a
# -jp=6 = 6 stack context, -jp=l6 = with line numbers
export WORST_LIBPATH="$WORST_LIBPATH:./lib:./test"
export WORST_SRCPATH="${WORST_SRCPATH-./lib}"
export LUA_PATH="${LUA_PATH};${WORST_SRCPATH}/?.lua;${WORST_SRCPATH}/?.w.lua;./test/?.lua"
luajit test/main.lua
luajit -- ${WORST_SRCPATH}/lworst/lworsti.lua test/worst_t.w

