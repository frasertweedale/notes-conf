The Machine: Hardware and Software
==================================

Keith Packard, HP


- Photonics to connect special purpose SoCs with massive memory
  pools

- If you have a problem that doesn't fit in a computer's memory, you
  are bound up by communication overhead

- *Memory-driven computer architecture*

Memory-driven computing:

- Application directly connected to fabric-attached memory
- mmap, msync go out of the question

The memory fabric:

- Next-generation memory interconnect (NGMI) switch
- Connect processors and memory at memory level
- Imagine getting rid of PCI bus

Fabric-attached memory node:

- Contains *processing element* and memory
- processing element does not manage memeory
- processing element talks through fabric, even to memory
  alongside it on a node

First version (prototype) of hardware:

- ARM SoC
  - has a little bit of local DRAM (*only* 256GB)
  - runs Linux
- FPGA on which NGMI is implemented
- Memory - 4TB
- An enclosure contains 8 of these nodes
- 10 of those in a rack
- Full rack as 320TB of memory
- 75-bit address space

Linux for The Machine:

- HPE modifications to linux to spuport
  - fabric-attached memory
  - FS abstractions that reach Fabric Attached Memory
  - Kernel changes needed for The Machine
- Additional support for
  - prims to handle fabric attached memory shared across SoCs
  - "Library File System" for maintaining shared data
  - remote virtual memory access
  - configuration and management capabilities

Secure communications:

- TLS is not available in the kernel
- Implementing Librarian File System via (fork of) FUSE
  - TLS bits done in userspace
  - a bit round-about but convenient development environment

(Later discussion of PKI needs)

- Atalla: hardware key manager
- memory encryption: key manager for provisioning memory encryption
  key
- Looking at putting unique assymetric keys in ROMs on the next
  version of the machine
  - something like TPM
  - can enrol the key in corporate PKI, so machines can authenticate
    each other
- also looking at using TPM on the *processing element*
- Overall it looks like a PKI would be needed to secure deployments
  of *The Machine*
