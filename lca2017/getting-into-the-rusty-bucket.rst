Getting into the Rusty Bucket
=============================

William Brown - Red Hat (RHDS core team)

- demarkation of unsafe code -> better auditing of at-risk code

- corrode: C -> Rust converter

- want to write plugins that can be called from C plugin infra

- rust cannot use a C header file (because it's literally just
  included in C file by CPP).  Got to duplicate all the constants.

  - have to redefine things as needed.  (SPOT violation)

- symbol export: rust mangles names

  - use ``#[no_mangle]`` pragma for entry points

- calling C code
  - need to wrap in ``unsafe { ... }`` block
    - suppresses pointer deref check and certain other checks

- debugging

  - use GDB like normal (with some tricks)

  - you can't break on a function, but you can break on a
    file:lineno.

- want to minimise calls to C, casts, void \*, etc.

- we want clean native rust types, traits to enforce plugin
  behaviour, etc.

- need "functional" or stateless C.  pass context pointers around.

- macros can hide ugly details

  - rust macros are their own DSL; type checked, can enforce
    trait bounds, etc.

  - must be explicitly exported and implicitly imported

- Cargo can't "install" extra data or objects

  - can we combine cargo and autotools?

  - make rust emit .a and .o and get libtool to build .so


Questions
---------

- Could you write a manual preprocessor to read constants from C
  header file and dump into a Rust unit?


feedback
--------

- name drop FreeIPA, didn't mention what it is
- what is LDAP / directory server ?

- code examples are small is small, so small
  - blow it up BIG
  - doesn't have to be the real var names, constants, etc
  - box highlighting is great
  - dark red on dark blue/grey is very hard to read

- I'm safe here at the top:
  - BG image, yikes.  be nice to vision imparied
    - even just darken the image with a filter?

- "C cannot be proven or reasoned about" - not quite true

- "cargo" - didn't mention what cargo is

- code comparison: get them both on one slide?  side by side?

- be clear that this talk is not Rust 101, will not include detailed
  discussion of syntax etc - but is rather about integrating Rust
  with existing projects.
