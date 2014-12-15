Functionally Oblivious and Succinct - Ed Kmett
==============================================

Building better tools

- cache-oblivious data structures
- succinct data structures

Data.Map

- in use since 1998
- built on *tree of bounded balance*
- designed for the Pointer/RAM model
  - assumption that every pointer in memory has same cost as every
    other

- need efficient range queries
- efficient writes
- unboxed
- I don't want to give up all the conveniences of Haskell
- I can let point query perf suffer a bit

- [(key, value)]
  - terrible insert performance
  - binary search for lookup
    - *necessarily* thrashes the cache
  - eventually you have it all in a single cache line
  - *offset* binary search avoids thrashing the same lines

- now we have *lots* of caches
  - lots of constants to tune
  - optimising for one cache recessarily sub-optimises for other
    caches

- leads to *cache-oblivious* memory model
