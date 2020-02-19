Sphinx Bench
============

This crate acts as a wrapper around Nym's [Sphinx](https://github.com/nymtech/sphinx) implementation for performance testing purposes.

### Prerequisites

Linux is required. You'll need a working Perl to run the bundled FlameGraph visualizer. 

You'll also need a copy of Linux `perf`: `sudo apt install linux-tools-common` will get you that. 

### Running it

If you want your results to be as accurate as possible, stop running as many other applications as is practical - you don't want to other applications interfering your test run. 

Start a terminal, `cd` into the top-level of this project, and run:

```
scripts/bench.sh
```

This will attempt to create 100000 Sphinx packets. The Linux [perf](https://perf.wiki.kernel.org/index.php/Main_Page) utility will then output data about time spent, and the [FlameGraph](https://github.com/brendangregg/FlameGraph) stack trace visualizer will give you a graphical analysis of time spent in the different parts of the program stack. 

`firefox output/pretty-graph.svg` shows the output visually in your browser. Click through different aspects of the flamegraph to see time taken within each part of the call stack. You'll want to click into the `sphinx-bench` part of the graph to disregard startup and shutdown time.

As expected, most of Sphinx's time during packet creation is spent in elliptic curve multiplication for on-the-fly creation of shared keys. We're using the [curve25519-dalek](https://github.com/dalek-cryptography/curve25519-dalek) library for this, and a high majority of our time is quite predictably spent in that library. 

