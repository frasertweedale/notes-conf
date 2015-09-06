Connascence
===========

- Thomi Richards <thomir@gmail.com>
- Canonical
- http://tech-foo.net
- @thomir


- As an industry our priorities are often wrong
- Correctness... or flexibility to change
- Solve today's customer's need... but that's not tomorrow's
  customer need
- As a programmer you have an imperfect understanding of the world;
  we have imperfect models
- The enemy of flexibility is rigid coupling
- "high cohesion, low coupling"
  - this is useless advice
  - there's lots of different types of coupling
  - doesn't help you write better software
  - need a taxonomy of "coupling"

Connascence

- Two components are connascent if a change in one would require
  the other to be modified in order to maintain the overall
  correctness of the system
- a taxonomy of coupling

- properties of connascence
  - strength: how easy it is to change something
  - locality: how close connascent elements are
  - degree: how many pieces are affected by the coupling
  - strength is predetermined; locality and degree depend on
    your codebase

- connascence of name (connascence.io/name)
  - the weakest connascence
  - you can do search-replace

- connascence of type
  - still relatively benign

- connascence of meaning
  - when something is assigned meaning that inherently doesn't have
    it
  - e.g. string treated a credit card number

- connascence of position
  - e.g. parameter lists, tuples or lists of data

- connascence of algorithm
  - where multiple components must agree on a particular algorithm

- static connascences:
  - you can reason about them knowing only the code

- dynamic connascences:
  - have to understand behaviour / properties of the runtime
  - hard to come up with small examples :)

- connascence of execution (dynamic)
  - multiple instructions must be run in a certain order
  - e.g. resource contention / locks

- connascence of timing (dynamic)
  - timing of execution is important
  - happens a lot in distributed services

- connascence of values (dynamic)
  - happens a lot in test assertions, e.g. "result should be ..."

- connascence of identity (dynamic)
  - when multiple components must reference same entity
