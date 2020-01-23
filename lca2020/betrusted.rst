Betrusted: Better Security
==========================

bunnie, Sean "xobs" Cross, Tom Marble

Slides: https://p.xobs.io/lca20-bt/#/

1. complex things are hard to analyse
2. software isolation is dead
   - speed or safety, pick one.  spectre etc.
3. motivated adversaries
4. untrusable supply chain
5. There is no HMAC for hardware

State of the art: secure enclaves.
The I/O problem
- "this input method reads everything you type, including your
  passwords".
  - sure, ok.  what else are you gonna do, not type your passwords?

Bottom line: want trust?  There are tradeoffs.

- features & usability -vs- ease of verification

Betrusted: verification requires simplicity.  Simplicity requires
focus.

- are you doing something for mobile/server?  Type of user?
- what is the MVP?

Betrusted design concept:

- what: mobile device.
- why: comms, authn, wallet
- who: "at-risk" end-users: high value targets (politically or
  financially); devs & enthusiasts
- global demographic (not just EN)

Betrusted: a security chip with I/O
- I/O surface that you can verify.
- "single-chip" design target
  - minimise hardware attack surface
- end user verifiable I/O
  - physical keyboard; black and white LCD

Single-chip constraints:

- cost v. RAM creates a Goldilocks zone
  - >90nm -> a "few" MiB on-chip RAM makes die size too large
  - <40nm drives up-front costs too high; also no eFLASH options

Design envelope summary

- compute env: ~100MHz CPU, "few" MiB RAM
- I/O: phys kbd, black/white LCD, SPI bridge to untrusted network
  coprocessor
- **Verifiable at point of use**
- application requirements:
  - text chat, voice chat / calling, authentication, wallet

Linux is not going to cut it

- >25M LoC by 19k authors
- won't fit in the hardware, impossible to verify/audit

Xous: A Betrusted OS

- <=4 MiB RAM; safe language; process isolation
- microkernel
- want to be auditable by a single person

Too many cooks

- "if there is one primary contributor, the chances for a file to be
  buggy decreases significantly" - Microsoft Research

Felix's rule of thumb

- "the amount of security-related code that one person can
  reasonably audit is about 64KiB of binary data"

Principles of software

- safety, concurrency, speed (the principles of rust)

Rust OS landscape

- Tock
  - active proj, RISC-V port, C and Rust userspace libs
  - no MMU support, no runtime spawn(), limited messaging
- Redox
  - active proj, full rust stdlib, full userspace
  - x86_64 only, unix-like, desktop focus
- Tifflin
  - active proj, rust stdlib, full uspace
  - nighly only, mainly x86

- Other alternatives an dinspirations
  - ChibiOS, HelenOS, Solaris, QNX

Xous system design

- memory model: borrow checker, message passing, inter-process
  borrowing, mutable (no access)/immutable (read-only) borrow
- mutable borrow: draw()
- immutable borrow: font database
- interrupts: all in userspace

Xous: missing features (i.e. that we don't want)

- fork()
- filesystem syscalls
  - filesystem server in userspace instead, wrapped by system
    library; pass messages to filesystem driver
- scheduler (instead: a user space program with access to other process'
  program counters)
- threads
- locking prims
- shlibs

Xous: everything in userspace

- small kernel, message passing, protected memory
- understandable by one person; made by many

Betrusted software

- voice chat/call
- authn
- wallet
- notepad (e.g. for jounos)
- text chat
  - first target; encompasses most of the essential building blocks
    such as crypto, kbd input, network
  - targeting matrix protocol
    - many client  and server impls; bridging to other protos

Rust dev progress:

- start on linux/amd64
- basic chat use cases
- transitive dep analysis
- mem analysis (to get usage down to smallest possible)
- migrate to Xous / RISC-V

i18n: output

- EN, FR, DE, Chinese, Arabic & Hebrew (RTL)
- emojis?  we want to support

i18n: input

- localized, **replacable keyboards**
- multiple planes (meta keys)
- dynamic work prediction/correction
- input method editors
- prototype uses Dvorak

The device:

- Renode emulation (https://renode.io/) of processor and other
  hardware
- CI, simulation
- want full UI testing

Reproducible builds desired.

How can I get involved?

- help us reimagine the UX for secure msging
- stay tuned for **mtxcli**
- contribute (leverage our CI)

Contact:

- betrusted.io
- github.com/betrusted-io
- Matrix: #betrusted:matrix.org
