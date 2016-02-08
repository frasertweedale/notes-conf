Changes in the security module infrastructure
=============================================

Casey Schaufler, Intel
(Leader of the "Loyal Opposition to SELinux")

- Security module infra was designed early on
- had serious problems
  - could only have one security module at a time, e.g. SELinux

Outline:

- stacking infra in 4.2
- first major/minor stacking, then extreme stacking
- basic handling of multiple modules

Stacking

- multiple security modules, with some restrictions
- extreme stacking = anything goes.  e.g. SELinux + Smack + AppArmor

Stacking as of 4.2

- "minor modules"
  - don't use security blobs
  - as many as you want
  - fixed order

- "major modules"
  - use security blobs
  - one only
  - checked last

- the only minor module we have is Yama

Stacking as of 4.x

- major modules have improved inode performance
- can specify order

Extreme stacking

- all lmodules treated equally
- may or may not use security blobs
- as many as you want
- specified order

Linus' inode request

- Put the blob into the inode
- Cleaner code; performs better

Module selection

- comma separated list of module names
- capabilities modules is not presented
- order matters
- list ``/sys/kernel/security/lsm``
- boot line option: ``... security=yama,smack ...``
- Kconfig::

    config DEFAULT_SEUCIRYT
    string "Ordered list of LSMs to register"
    depends on security
    ...


About secids

- Kernel internal interger that represents a security context
- Move secid<->secctx mapping out of modules (SELinux, Smack, etc)
  into the infrastructure, under
  ``CONFIG_SECURITY_EXTREME_STACKING``.

Not addressed

- user spaces changes for extreme contexts (will need ``liblsm``)
- dynamic module loading and unloading
- blob size optimisation
- netlabel oeorientation
  - can be made to work, but won't without SELinux changes


Security blob:

- a file has security information associated with it
- create extattr and put that information there
- when system creates inode for file, it reads sec info from
  extattr and read it into inode
- thus, it is blob to system; system doesn't know/care, only the
  security module cares and knows how to interpret it.
