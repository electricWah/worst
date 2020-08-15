#!/bin/sh
# profiling: -jp -jp=s -jp=l2 -jp=a
# -jp=6 = 6 stack context, -jp=l6 = with line numbers
exec env "LUA_PATH=${LUA_PATH};${WORST_LIBDIR-./lib}/lworst/?.lua" luajit -- ${WORST_LIBDIR-lib}/lworst/main.lua ${WORST_LIBDIR-lib}/lworsti.w $@

