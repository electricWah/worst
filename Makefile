
perf.svg: perf.perf
	stackcollapse-perf.pl --all perf.perf | flamegraph.pl > perf.svg

perf.perf:
	perf record -F 25500 -g -- ./worst
	perf script > perf.perf


worst-perf:

# RUST_LOG=hell::interpreter=debug/Eval ./worst -A 2>worst-swap.log
# then
# awk 'BEGIN {t=0;tt=0;p="start"} /DEBUG/ {tt=t; t=substr(gensub(/[^0-9]/, "", "g", $2),9); print p, t-tt; p=$5}' worst-swap.log |sort -h|awk '{arr[$1]+=$2;ab[$1]+=1} END {for (i in arr) {print arr[i], ab[i], i}}' > worst-swap.timed
# Most time:
# sort -h -k 1,1 <worst-swap.timed | tail
# Most calls:
# sort -h -k 2,2 <worst-swap.timed | tail 


