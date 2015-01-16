seL4 is Free!  What does it mean for you?
========================================

seL4
----

- latest and greatest member of L4 microkernel family
- used in Qualcomm modems, iThing security co-processor
- mathematic proof of security
- GPLed 2014-07-29


Proof
-----

- Abstract mathematical model of kernel

- C implemention (~9k lines)

- Refinement proof that C implementation correctly implements the
  abstract model

- No buffer overflows, uninitialised vars, NUL-deref, stack
  smashing, code injection, ROP, etc.

- Also have a correctness proof that machine code is correct

  - No need to trust the C compiler not to stuff up.

  - No need to assume that the compiler makes the same assumptions
    about C.

- Proof of timeliness of kernel operations.  Real upper bounds for
  IRQ latencies.

- CIA security properties: confidentiality, integrity, availability

  - Proof that abstract model enforces these for each of CIA.

- All of these proofs are world firsts.

- Proofs are machine checked (Isabelle)

  - Much more code in the proofs than in the kernel

  - Functional correctness proof is 180k LOC


Exclusions (at present)
-----------------------

- initialisation
  - boring from research point of
  - give NICTA money and they will do it

- privileged state and caches
  - no formalisation of memory model yet
  - don't formally know when we have to flush caches

- multicore
  - have a high level concept proof

- covert timing channels


Performance
-----------

- World's fastest microkernel

- No need to sacrifice performance for correctness

- Proofs allow aggressive optimisation


What seL4 is NOT
----------------

- An operating system
  - All device drivers, OS services are usermode processes

- seL4 does provide strong isolation and controlled IPC between
  applications


Differences from other L4 microkernels
--------------------------------------

- no memory management in kernel (design for isolation)
- Global Resource Manager takes all memory not used by kernel
- Resource Managers are nested and can ask higher RMs (GRM is root)
  for an allocation
- RMs *hand memory to kernel* when needed


High-assurance systems on seL4
------------------------------

- Boeing manned/unmanned Little Bird (AH-6) helicopter
- SMACCM Drone (Galois?)


Current NICTA work on seL4
--------------------------

- High-perf multicore support
- Full support for virtualisation extensions (ARM, x86)
- 64-bit support (amd64 is close, ARM64 later)
- Timing channel elimination (ETA 2015)
- Temporal isolation and mixed-criticality scheduling (ETA 2015)
- Hardware failure resistance (DMR/TMR on multicore; ETA 2015)

Longer term:

- Cost reduction by automation and abstraction
  - Present seL4 cost: $400/LOC
  - Other "high assurance": $1k/LOC (no proof, poor performance)
  - Low assurance (Pistachio): $200/LOC (no proof, high performance)
  - Want to close gap with low assurance methods; expect to pull it
    off in next five years

- Device driver synthesis
  - synthesise driver code from hardware and OS interface specs
  - works already for simple devices

- Code and proof co-generation
  - High-level spec in DSL describes logic; generate C code and
    proofs
  - File systems as case study

- Type and memory-safe high-level langauges
  - Do verification cheaper in HLL semantics
  - Requires verified HLL runtime and compilers

Development
-----------

- stable (the verified kernel) and experimental (on the path to
  verification) branches
- developers have private branches

Contribute
----------

- Libraries; presently rudimentary
- Platform ports, esp. popular ARM boards
- Drivers; very few at moment
- Network stacks and filesystems
- Tools
- Languages
  - Core C++ support just released; lacks std template lib
  - Haskell presently in progress (Galois)
  - Python would be nice

Why NOT use seL4?
-----------------

- Very rudimentary programming environment
  - But you can help with that.

- I like unsafe/insecure systems or the thrill of danger!
  - OK go shoot yourself

- Actually, I want to use seL4!
  - Right answer :)
