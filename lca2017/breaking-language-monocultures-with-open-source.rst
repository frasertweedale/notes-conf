Stranger in a Strange Land: Breaking language monocultures with open source
===========================================================================

- despite widespread interest in python, it has typically been
  available only in desktop and server environments
  - what about phones, tablets, watches, set top boxes, etc?
  - if you want iOS app -> ObjC / Swift
  - if you want andrdoid app -> java
  - if you want web app -> js

- industry currently requires polylingualism
  - not a huge challenge for professionals, but a barrier for
    hobbyists or other professionals.

- you have to *be* polylingual, but you don't get to *choose* the
  language in many contexts.  Many environments / platforms are
  monolingual.

- can you break the monoculture?  Yes!
- python is used as the example in this talk.  the concepts apply to
  other languages.

- approaches:
  - (embedded) CPython
  - ctypes and FFI
  - when you don't have a C compiler? (e.g. android)
    - java has JNI, android has NDK
    - but still has heavy focus on using java
  - cpython may be too big to use on a given platform (e.g. embedded
    microcontrollers)
  - Python that isn't CPython, native to target platform.
    - e.g. enscripten: C -> JS

- inside a python:
  - parser
  - compiler
  - eval loop
  - stdlib
    - native and pure-python bits

- Python from scratch:
  - jython, iron, micropython, etc

- CPython already has a good Parser.
  - take AST, convert to other bytecode
  - VOC outputs Java bytecode

- implement a *python* bytecode interpreter
  - run the .pyc file
  - it's a stack-based VM.
  - ``dis`` module (disassembler)
  - bytecode format is not specified.  Impl details of CPython.
  - *batavia*: CPython VM impl in JS.
  - can be done in 500LOC or fewer of pure python
  - biggest complication: CPython makes no guarantees about
    stability across major versions.

- the missing pieces
  - part of your stack won't be running on your new platform.
    - effectively cross-compiling the program
    - won't have a repl (but the platform might not be one that
      makes sense for running a repl on).
    - doesn't preclude repl, though.

- Why not cross compile?
  - python scoping rules quite different from JS'

- deployment and delivery
  - some new platforms make it easier, not harder

- WebAssembly
  - the subset of JS that runs fast
  - W3C have standardised a binary format for it
  - compiler backends to WebAssembly

- Let a hundred flowers^Wlanguages bloom
  - monolingualism isn't healthy
- people engaged in other fields are not stupid, but they shouldn't
  have to learn 4 languages to deliver an app!
