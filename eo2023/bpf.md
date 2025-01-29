# bfptrace recipes: 5 real problems solved

- Trent Lloyd, Canonical

Intent:

- Performance analysis
- Behaviour analysis
- At runtime
- In production

Tradition tools, perf analysis

- fast sybsystem specific performance counters
- heavy use of averages which hide outliers
- limited per-process or per-device breakdown (iostat, iotop,
  vmstat, ...)
- instance snapshot misses data (top, netstat)

Traditional tools, behaviour analysis

- strace, gdb, blktrace, iptraf, debug logs
- heavyweight; dump a lot of data from kernel to process in
  userland.

It would be great if...

- custom metrics
- custom grouping
- high performance

Demos
