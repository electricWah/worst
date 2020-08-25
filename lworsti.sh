#!/bin/sh
# profiling: -jp -jp=s -jp=l2 -jp=a
# -jp=6 = 6 stack context, -jp=l6 = with line numbers
export WORST_LIBPATH="$WORST_LIBPATH:./lib"
export WORST_SRCPATH="${WORST_SRCPATH-./lib}"
export LUA_PATH="${LUA_PATH};${WORST_SRCPATH}/lworst/?.lua"
exec luajit -- ${WORST_SRCPATH}/lworst/lworsti.lua ${WORST_SRCPATH}/lworsti.w $0 $@

