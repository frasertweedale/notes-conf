Control Flow Integrity in the Linux Kernel
==========================================

Kees Cook

What is Kernel CFI
------------------

- Most vulns are about gain exec control, where initial flaw is at a
  write boundary.  What can be written to, and how can it be turned
  into exec control?

- Attack method: write to kernel code
  - used by ancient rootkits (and AV)
  - target must be executable and writable

- NX etc mitigates

- Maybe I can call into the kernel and overwrite fun ptrs?
  - ROP universe of attacks
  - Need: Writable, contains function pointers for *indirect* fun
    calls (exec not needed)

  - what contains writable fun ptrs?
    - heap, stack
    - we're never gonna have a non-writable stack!

  - if you can exploit, you can jump to any executable kernel memory
    (huuuuge attack surface)

- CFI: ensure that each indirect call can only call to "expected"
  targets.

  - validate indir fun ptrs at call time, somehow
    - iff the same _prototype_, call site can choose any matching
      fun.  This narrows attack surface a lot
    - current hardware support has poor granularity (e.g. BTI)
      - BTI says "you can only jump to the start of a function...
        *any function*"
      - "BTI" = "Branch Target Identification"
  - what is required?
    - link-time optimisation (LTO); linker has visibility across entire
      code base as once
    - funs with same proto collected into jump tables and checks
      added at each call site

  - ::

      clang -fuse-ld=lld -flto -fvisibility=default -fsanitize=cfi \
        -fno-sanitize-cfi-canonical-jump-tables``

  - measurable but not terrible performance hit

Better impl ideas?

- hash bytes before fun start/ret dests, and check for matches at
  call and return sites (e.g. as done by PaX team's RAP)

  - not compat with execute-only memory though

- Research (why finer granularity is better)
  - 55% of indirect call have <=5 valid targets
  - but there is a long tail... 7% have >=100 tgts

CFI: backward-edge protection

- maintain integrity of saved return addresses
  - hardware "shadow stack" e.g. Intel CET, arm64 "Pointer
    Authentication"
    - on Intel it will be used implicity (no changes to asm to use)
  - in Clang, x86 impl was removed ∵ slow, race conds; arm64 can
    reserve a register (x18) fo rall shadow stack manipluations
  - shadow stack location needs to remain secret

- Clang Shadow Call Stack (SCS)
  - ``-ffixed-x18 -fsanitize=shadow-call-stack``
  - results in 2 stack registers: sp and unspilled x18
  - only loads of the return addr (link) register from shadow stack
    are used for return

Pixel phones and Android ecosystem

- LTO, CFI and SCS in Android upstream kernels
- Pixel (3+) as well as other vendors enabling the feature
- Android Compatibility Definition Document (CDD) says:
  - vendors STRONGLY RECOMMENDED to enable it
  - next year, it becomes REQUIRED

Gotchas

- massive LTO link times
  - final linking step under LTO was very slow, so switched to
    ThinLTO (``-flto=thin``)

- asm code
  - jump tables only built for C code; Clang was extended to
    generate jump table entries for all ``extern`` functions
    (``-fno-sanitize-cfi-canonical-jump-tables``)

- relative addrs
  - exception tables: calculated as delta from true fun addrs,
    ignored jump table addr, so disable CFI checks for exception
    tables (which are hard coded)

- linker aliases
  - ftrace made unusual calls to differing prototypes, but linker
    aliases satisfied CFI

- Kernel Page Table Isolation (KPTI)
  - jump tables were outside mapped entry stub, so had to also map
    the jump tables


Upstreaming status

- Clang: done? (LLVM 10; unreleased)
- Kernel: consistent progress
  - SCS support expected for v5.6
  - fun ptr prototype corrections (arm64 done; x86 1 patch remaining)
  - Clang LTO (20 patches; fingers crossed)
    - mostly mechanical build script and Kconfig changes
  - Clang CFI (14 patches; depends on LTO)
    - hopefully uncontroversial

What do failures look like?

- either panic kernel or kill a single thread
- if ``CONFIG_CFI_PERMISSIVE`` set, then you just get a warning and
  trace
