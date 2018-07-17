WebAssembly, Past, Present and Future
=====================================

Ben Titzer (Google) & Andreas Rossberg (Dfinity)

What is WA?

- universal compilation target
- new capability for web
- virtual instruction set architecture
- predictable performance model

What is WA not?

- not a value judgement about langs
  - not designed to "fit" a particular language or paradigm
  - abstraction over *hardware* not language
- not a user-facing language
  - there's a text format but you're not supposed to write it
- not a compiler backend / IL
- not exclusive to the web


Data types / functions / memory

- i32, i64, f32, f64
- + - * / % & | << >> etc
- conversion, reinterpretation
- flat, single namespace
- static binding
- call call_indirect
- linear memory (bounds-checked)
- no implicit access to memory (have to load/store)
- structured, no goto
- block loop if br
- operands push/pop values on stack
  - typically the stack will be optimised away

The past

- 2011 Google releases Native Client (NaCl)
- 2013 Mozilla releases asm.js
- 2013 V8 starts work on TurboFan (JS JITer)
- late 2014 Luke & Ben embark on replacing asm.js with bytecode
- 2015 full on "side proj"
- March 2015, NaCl team gets involved (clash of world views)
- May 2015, Beer with Pizlo (Apple), talks with Microsoft
- June 2015: announcement of collab, W3C group

Towards release

- mid-2015: prototypes in V8 and FF
- Aug 2015: reference interpreter
- Mar 2016: first demo at GDC; Three browsers executing same demo
- late 2016: full formalisation; submission of PLDI paper
- Mar 2017: shipping in FF and Chrome
- Sep 2017: safari and edge shipped

Edge of madness; performance vs portability

- not everybody agrees from the outset
- nasal demons, IEEE rounding modes
- mem page size, details of validation

edge of control; control flow and stack machine

- structured control flow instead of goto
- evolution from pre-order to post-order to stack machine

edge of destruction; rigour and specification

- first mainstream language designed hand-in-hand with formalisation
- central to official language spec

Goals & constraints

- semantics:
  - lang/platform/hardware independent
  - fast/safe to exec
  - deterministic
  - easy to reason about
- representation
  - compact
  - easy to generate
  - fast to decode/validate/compile
  - extensible
  - streamable

did we deliver?

- fast single-pass decode+validate (>100MB/s) with single thread
- single-pass translation to SSA-based compiler IR (V8/TurboFan)
- fast optimising compiler (1.8MB/s single thread, 7MB/s with 8 thr)
- fast baseline compiler (10MB/s single thread, 60MB/s with 8 thr)
- within 20% of native code perf (geo mean, vs 80% for asm.js)

formal semantics

- complete
- no undefined behaviour
- using OTS techniques from 4 decades of PL research
- machine-verified proof of soundness
- almost embarassingly-simple type system; type deduction
- small-step reduction
- mechanisation: Isabelle, Coq, K

open standard

- W3C, github
- lots of public contributions; a few crazies
- no copyright or patents

roadmap

- v1 (shipped): low-level languages
- v2 (next year): high-level langs
 -v3 (later): support "dynamic" langs better?

proposal process:

- spec text (the prose)
- formalization of the feature
- extend reference interpreter
- comprehensive test suite
- convince >= 2 production engines to implement the feature

future features

- threads
- tail calls
- exception handling
- stack switching
- GC
- SIMD

threads:

- ability to emulate pthreads
- instructions for atomic shared memory access
- main challenge: weak memory model
- impl in Chrome (stalled by Spectre)

spectre impact

- all browsers disabled shared memory
- offensive research in VM teams
- compiler mitigation work
- **not** a solved problem

tail calls

- dedicated instructions
- tricky to implement in some impls (Microsoft)
  - "caller pop" calling convention
- allow "amortised" tail calls

exceptions

- enable "zero cost" exception handling
- kind of like ML exception
- impl in Chrome a WIP

stack switching

- to support control abstraction like coroutines,
  continuations, lightweight threads, async/await
- formally, need some form of delimited continuations
- investigating *effect handlers*
- no proposal yet

GC

- to support high level langs and host references
- lightweight structs and arrays
- reuse existing GCs in JS engines
- in vear early stages; huge design space

SIMD

- for niche performance use cases
- exposing 128-bit SIMD instructions of modern CPUs
- "only" 200 instructions

other embeddings

- embeddings specified as explicit layers
- mobile platforms
- content delivery networks (e.g. Fastly)
- blockchain cloud computing (e.g. Dfinity)
- embedded devices
- standalone impls

summary

- WA = efficient, universal, safe, sandboxed code format
- open, public standard process
  - already participation from non-browser interests
- formal rigous and machine verification
- neither "web", nor "assembly"
