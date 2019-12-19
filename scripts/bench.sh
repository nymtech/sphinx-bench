
sudo perf record -F 997 -a -g --  target/release/sphinx-bench

sudo perf script | ~/workspace/misc/FlameGraph/stackcollapse-perf.pl | ~/workspace/misc/FlameGraph/flamegraph.pl > ~/Desktop/pretty-graph.svg
