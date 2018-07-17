Isolation without Containers
============================

Tyler McMullen @tbmcmullen (Fastly)

Multi-tenancy

Isolation is about being able to run some code without it affecting
other running code in unexpected ways.

Resource isolation

FDIR (Fault-detection, isolation & recovery)

- from control engineering; not directly but morally applicable
- isolation: limit and *know* the blast radius
- recovery: system can continue

Control flow integrity & memory safety

- control flow: set of possible paths a program execution can take

- memory safety: analysis of locations that a program should be able
  to store & retrieve data from

Fault domain

- what is fucked if this program blows up?

*Fault Isolation is about reducing the set of possible faults to a
knowable, recoverable set.*

I want...

- to isolate every request individually
  - want to run untrusted code on behalf of customers
- 10,000s of RPS/server
- P95 lifetime: ~1ms
  - the longer lifetime an isolate has, the easier it's going to
    be to pop it

Process-based isolation:

- By default, the memory region is the fault domain of a process
- Virtual Memory
- the code *is in the fault domain*
- so we can ignore the problem of control flow!
- but... SPECTRE.  The only isolation boundary has been smashed
- syscalls


Containers

- resource isolation
- we want certain resources to be shared, and not others

``dlopen``

- the fault domain of a dylib is the *entire process*


Execution: timers and fuel

- timers: obvious
- fuel: on reverse jumps, decrement a counter
