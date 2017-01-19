Python profiling and performance tuning in production
=====================================================

Joe Gordon (Pintrest)

- Problem: slow search
  - > 2s latency (not including network)
  - 70% of latency outside of Search system

- similar pattern across all API endpoints

- the problem was GEvent
  - gevent is coroutine-based Python networking library
  - greenlets on top of libev event loop
  - great for network/IO-bound systems, but be aware when mixing
    with CPU-intensive tasks

findings

- performance degrades with increased cpu usage, requests per
  process
- impact: reduce latency by doing fewer requests per process
  - simple config change
  - OS process preemption
  - reducing CPU usage improves performance and efficiency

goal

- API Cluster was:
  - significant source of latency
  - one of the most expensive clusters

- increase throughput and decrease latency by decreasing CPU usage
  - 33% efficiency gain
  - define measurable goal
  - metric for goal: requests per second per host (RPS/host)

profiles

- API cProfiler middleware
  - profiles function calls
  - save each request to separate file
  - large overhead (can't use in prod)

- python ``pstats`` module
  - good way of diving through call graph
  - find CPU-bound function calls

- line profiler
  - ``@profile`` decorator
  - way too slow to run in prod

statistical profiles

- event-based profiling, versus...
  - trakc events e.g. call, return, exception
  - deterministic
  - slow (overhead)
  - examples: cProfile, line_profiler
- statistical profiling
  - sample data
  - probe call stack periodically
  - non-deterministic
  - slightly lower accuracy
    - profile for longer
  - marginal overhead (run in prod!)
  - examples: stacksampler, vmprof

stacksampler:

- records stack every 5ms
- emits data over localhost:16384
- format: stack: count
- fuzzy
- flame graphs

vmprof (vmprof/vmprof-python):

- from PyPy
- statistical *line* profiling
- low overhead
- line profile entire codebase
- useful for C extensions
- a few rough edges

Diff of two flame graphs?

- what's only in A / only in B?
- easily analyse effect of a change

Examples:

- 30ms (10%) of ngapi spent in ``deepcopy``
  - removed unnecessary usages, down to 1.6%
  - measurable increase in RPS/host
- thrift (RPC library)
  - 13% of time in thrift codec
  - switch to C implementation → 3.3%

Outcome:

- 50% of latency outside search system
- height of API latency == height of search system latency
  (which is what we want)
- fluctuation: 50% -> 22%
- RPS/host increased by 40%
- ability to detect and understand regressions

Lessons:

- profile in prod
  - synthetic benchmarks aren't enough
- visibility
  - profile code
  - define efficiency metric
- finding what to fix is harder than fixing it
- there is always low-hanging fruit
- watch for performance regressions
