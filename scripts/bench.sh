#!/bin/bash
sudo perf record -F 997 -a -g --  target/release/sphinx-bench

sudo perf script | flamegraph/stackcollapse-perf.pl | flamegraph/flamegraph.pl > output/pretty-graph.svg
