Non Cryptographic Hash Functions - Adam Harvey
==============================================

- Sometimes you don't (have to) care about security

- Hash function that takes arbitrary amount of data and turns it
  into a finite amount of data

- We are familiar with this concept from cryptographic hashes


- why
  - track things you've previously seen
  - checksums
  - bloom filter

- goals
  - reasonably fast
  - reasonably well-distributed
    - because you're implementing a hashtable/hashset
    - in python if you're implementing a hash table, chances are
      something has gone horribly wrong in your life

- how
  - avalance effect
    - each single bit change in input makes big change in output
    - bitwise operations, incl. shifts
    - magic numbers (usually prime)
    - many rely on integer overflow behaviour
  - ideal: on average 1/2 bits in output change for any single-bit
    change in input

- examples:
  - fnv1a32
    - operates 1-byte at a time
  - murmurhash3
    - operates with 4-byte blocks
  - xxhash
    - similar construction to murmurhash
    - 16-byte input blocks
  - superfasthash
    - no magic constants
    - a lot of collisions (bias)
  - jshash
    - massive bias
  - crc32
    - very fast
    - uses a LUT heavily to do its division

- conclusions
  - need to know your environment
