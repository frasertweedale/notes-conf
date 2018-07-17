Why VM benchmarking is probably misleading you
==============================================

Edd Barrett (King's College London) @vext01

Paper: Virtual Machine Warmup Blows Hot And Cold (OOPSLA 2017)

- the lifetime of a VM = "process execution"
- run benchmark in a loop = "in-process iterations"

JIT compilation phases:

1. "profiling interpreter mode"
   - what code is frequently executed
   - what types and values arise?
   - eventually VM has enough info to decide what to do in..
2. compilation
3. "peak performance"

In reality:

- noisy
- there can be several phases of compilation
- GC pauses

Warmup phase

- users do not like "bad warmup"
- VM authors dislike bad warmup
- in literature there was little research in VM warmup


Experiment:

- Hypothesis: *small deterministic programs reach a steady state of
  peak performance*
- Which benchmarks?
  - "Computer Language Benchmark Games" (CLBG)
    - CFG non-determinism removed
    - checksums added to all benchmarks
- How long to run the benchmarks?
  - 2000 in-process iterations
  - 30 process executions
- VMs: graal, HHVM, TruffleRuby, Hotspot, LuaJit, PyPy, V8, GCC
  - single GCC was used to build each VM
- Machines: Linux (2), OpenBSD; various RAM, CPUs
- Harness: Krun.  Control as many confounding variables as possible
  - constrain all the environment around the benchmark, but not the
    VM or benchmark itself
  - minimise I/O, fixed heap and stack ulimit, drop privs to "clean"
    user acct, reboots system prior to each proc exec, checks dmesgs
    for changes, checks system temp is within tolerance, enforces
    kernel settings

After data collection:

- *Changepoint analysis* and classification
  - ignore outliers (assumed GC pause)
  - identify *changepoint segments* with similar mean and variance
  - identify warmup, slowdown, no steady state
- very close segments are considered equivalent

Results

- exeriment has been run many times; same results
- many process executions slowed down (unexpected result)
- some did not stabilise
- sometimes different behaviours between process executions
- "good warmup" only occurs for 67-70% of process executions
- hypothesis not supported

Next steps:

- instrumented PyPy & HotSpot to reveal time spent in JIT or GC
  - (found but in PyPy GC)
- HotSpot JIT event at slowdown; may be a bug in JIT
- there are still mysteries to be uncovered here

What have we learned?

- benchmarks often don't warmup as we expect
- often not repeatable
- have we been mislead by our inability to to benchmark
- have we implemented bad / ineffectual optimisations

What can we do?

- run benchmarks for longer to uncover issues
- accept that peak performance may not occur
- can't alwayts blame GC or JIT
- always report warmup time (users care about it)
- engineer VMs for predictable performance
