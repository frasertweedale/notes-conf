Using persistent memory for fun a profit
========================================

Matthew Wilcox, Intel

What's persistent memory?

- Retains data without power
- NV-DIMMs available now from multiple companies
- Intel 3D XPoint™ DIMMs shipping in 2017

How do we use it (as software people)?

- Total system persistence
  - Problem: CPU cache is not persistent
- Application-level persistence
- Complete redesigned operating system
- A special-purpose filesystem
- A really fast block device
- Small modifications to existing filesystems

- New CPU instructions (CLFLUSHOPT, CLWB, PCOMMIT)
- Special-purpose programming language
- Managed code
- Non-volatile Memory Library (NVML)

What does NVML provide?

- libpmem - low level pmemsupport library
- libpmemblk - arrawy of atomic blocks
- libpmemlog - atomic appends of arbitrary size
- libpmemobj - transactional memory object store
- libvmem - use persistent memory in a volatile way
- libvmmalloc - transparent replacement of malloc

Transactional memory object store:

- locking
- type safety
- doubly linked lists
- non-transactional object manip
- KV stores
- replication
- C++ support

Resources:

- http://pmem.io/
- http://www.intel.com/nvm

Q&A:

- implications for sensitive data (e.g. private keys) in memory?
  - our security people take it very seriously
  - some govts prohibit crypto being used so as to prevent their
    agencies looking at storage
  - there are crypto-enabled DIMMs
