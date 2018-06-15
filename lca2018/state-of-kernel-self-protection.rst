Kees Cook
@kees_cook

https://outflux.net/slides/2018/lca/kspp.pdf

State of Kernel Self Protection
===============================

- bug lifetimes are huge
- even if all upstream bugs were instantly fixed, old devices.
- avg time between intro & fix = ~5y.
  - it is creeping up towards 6y on avg

- attackers are good at finding bugs & dont boast about when they
  found their 0day

- whack-a-mole isn't a solution (but it's what we're currently
  doing)

- linux now is where car ind. was in 1960s
  - designed to run, not to fail
  - we must handle failures (attacks) safely
  - lives depend on linux

- some truth to security bugs being "just normal bugs"
- your security bug may not be my security bug
- bugs might be in out-of-tree code
  - non-upstreamed vendor drivers
  - not an excuse to claim "not our problem"

Killing exploitation is best

- we'll always have bugs
- stop their exploitation
- elim expolit targets and methods
- elim info leaks
- elim anything that assists attackers
- *even if it makes kernel dev more difficult*

Typical exploit chains

- modern attacks often use >1 flaw
- need to know where targets are
- need to inj (or build) malicious code
- need to locate malicious code

What can we do?

- exploit mitigation tech already exist (grsecurity) or
  have been researched but haven't been in upstream kernel
- there IS demand for kernel self-protection in upstream kernel

Out of tree defenses?

- downstream kernel forks
  - Red Hat, Ubuntu (AppArmor), Android (Samsung KNOX), grsecurity
  - fewer eyeballs; takes eng resources
- upstreaming means:
  - no more forward-porting
  - more review
- putting stuff into upstream is the only way for everyone to get it

Defenses that are only used by a subset of uses, it may be
accidentally effective

- example: spam protection measure.  when ever server implements it,
  bots adapt

Kernel Self Protection Project (KSPP)

- started 2015 after WaPo article
- ~12 orgs and ~10 individuals working on ~20 technologies
- slow and steady progress
- mainly focused on kernel tech to protect *userspace* from attack

Probabilistic protections

- derive strength from some system state unknown to attacker
- weaker than determanistic protection
- examples: passwords, stack protected canary value, ASLR
- makes attacks more expensive

Deterministic protections

- system always blocks attackers
- examples (modulo meltdown/spectre): RO mem, bounds-checking

Bug classes

- stack overflow & stack exhaustion
  - mitigations: canaries, guard pages, alloca checking, shadow stack
- integer over/underflow
  - mitigation: refcount overflow check, compiler plugins to detect
    overflow
- buffer overrun
  - mitigation: runtime size validation, FORTIFY_SOURCE, heap
    canary, linked-list hardening, ...
- format string injection
  - context: %n writes to memory
  - dropped %n entirely from kernel printf impl
  - can we get rid of %p?
  - detect non-const strings at compile time
- kernel pointer exposure
  - mitigation: obfuscate output of %p
- uninit variables
  - mitigation: compiler plugin, stackleak, clear kernel stack
    between syscalls, instrument compiler to fully init all structs
- use after free
  - mitigation: clear mem on free, segregate mem used by kernel and
    userspace, randomise heap allocs

Exploit methods:

- finding the kernel
  - mitigation: kASLR, hide symbols and pointers, runtime
    randomisation of kernel funcs, exec-but-not-readable mem,
    per-build struct layout randomization (GRKERNSEC_RANDSTRUCT)
- direct kernel overwrite
  - mitigation: exec memory cannot be writable
- func ptr overwrite
  - mitigation: read-only func tables, make sensitive targets only
    writable during updates, struct timer_list.data field removal
- userspace execution
  - hardware segregation (SMEP, PXN)
  - emulated memory segregation via page table swap, PCID, etc
  - compiler instrumentation to set high bit on function calls
- userspace data access
  - hardware segregation (SMAP, PAN)
  - emulated memory segregation via page table swap, PCID, etc
  - eXclusive Page Frame Ownership
- reused code chunks
  - mitigation: JIT obfuscation
  - compiler instrumentation for control flow integrity

Challenges:

- conservatism, technical difficulty, takes time

Q&A:

- A: some of the contributions are not going to be coding.  We need
  guides for better devel practices or even FAQ ("is it safe to pass
  a function pointer on the stack?")

- Q: are we making the kernel un-debuggable?
  - A: short answer: yes.  part of the technical burden of the
    defenses.  challenge: how to do it while still helping
    developers but hindering attackers.
