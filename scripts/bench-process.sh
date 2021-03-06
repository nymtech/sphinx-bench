#!/bin/bash
sudo perf record -F 997 -a -g --  target/release/sphinx-bench-process

sudo perf script | flamegraph/stackcollapse-perf.pl | flamegraph/flamegraph.pl > output/process-graph.svg
