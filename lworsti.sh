#!/bin/sh
# profiling: -jp -jp=s -jp=l2 -jp=a
# -jp=6 = 6 stack context, -jp=l6 = with line numbers
export WORST_LIBDIR="${WORST_LIBDIR-./lib}"
export LUA_PATH="${LUA_PATH};${WORST_LIBDIR}/lworst/?.lua"
exec luajit -- ${WORST_LIBDIR}/lworst/main.lua ${WORST_LIBDIR}/lworsti.w $@

