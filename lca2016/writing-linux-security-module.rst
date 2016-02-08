How to write a Linux Security Module that makes sense for you
=============================================================

Casey Schaufler, Intel

Why write a security module?

- already have great set of security modules
- you can do anything you want with SELinux already
  - that's what Dan Walsh will tell you
- because it's your best option
- existing security modules are showing their age; we designed in a
  different time (before "cloud")
- there *are* things you can't do with SELinux
  - or that you wouldn't want to try to do
- the right way to control kernel resources

Restrictive controls:

- traditional checks are still done
- UID-based check
- capability checks
- can't override a denial

Security module don'ts:

- don't duplicate an existing module
  - you want to do something different
  - if you want to do something that SELinux already does, use /
    work with SELinux
- don't depend heavily on userspace helpers
- don't inflame Al Viro

The most important principle:

- always do your research
- let no one else's work evade your attention


Components of an LSM
--------------------

Hooks:

- security module data management
- access checks
- pick and choose as needed
- a sprinking of functions starting with ``security_``

Hook return values:

- ``ENOMEM``
- ``EACCES``: policy denies access
- ``EPERM``: privilege is required to do this:

Object based hooks:

- object is the passive entity in LSM parlance
- subjects access objects
- e.g. file, other process
- access based on attributes attached to the object
- it may be difficult for a human to identify

Path based hooks:

- associated with pathnames
- "I don't care what it is... anything at ``/etc/passwd``"
- may not uniquely identify an object
  - symlinks
  - mount points
- human friendly


Security blobs:

- hang off kernel data structures
- managed by the module
- called blobs because system doesn't look / know / care - only the
  LSM calls
  - all it cares is that you allocate and free them properly
- presence/contents are completely up to the needs of the module

Blob, secid and secctx:

- blob contains whatever you like
- secctx is a string describing it
- secid is a 32-bit number
  - one per secctx
  - never exported
  - volatile

Major security module:

- use security blobs
- *any* module that uses blobs
- you (currently) only get one
- called last
- SELinux, Smack, AppArmor are major security modules

Minor security modules:

- requires no blobs
- called after:
  - traditional controls
  - capabilities
- called before any major module
- Yama is a minor module (currently the only one)


Designing your security module
------------------------------

What do you want to protect?

- objects
  - objects created by a particular user
- pathnames
- processes
- hunks of data
- resources

What do you want to protect it from?

- users (malicious, stupid)
- applications (malicious, badly written)
- network access

How do you want to protect it?

- deny access
- log the attempt
- change some attributes
- something clever
  - rate limiting
  - delete something from disk

Process attributes

- ``/proc/pid/attr``
- ``security_getprocattr``
- ``security_setprocattr``
- defined in ``procfs``
- don't reuse entries

Object attributes:

- information about things
- use traditional attrs to heart's content
  - uid/gid, file type, size, mode, locks, fs info
- don't change/overload what they mean!

Extended attributes:

- attached by filesystems
- privilege required to change them
- as big as you like
- efficient

Pathname:

- ``struct path``
- not very convenient
- not definitive (mount points, symlinks, hard links, etc)

Networking
----------

- You may not want to go there
- Try netfilter first
  - it probably does what you want to do

Socket operations:

- checks on bind, listen, connect et al
- packet delivery
- ``SO_PEERSEC`` to pass security attrs

Unix domain sockets:

- access to the filesystem object
- access to both sockets
- hooks for connect and send

Internet sockets:

- only one end of the operation
- packet header available on receive
- support for attr passing using CIPSO
  - common internet protocol security option
  - support for CALYPSO (IPv6 version of CIPSO) is coming


Audit trail
-----------

Define your audit data:

- ``include/linux/lsm_audit.h``
- common_audit_data (under #ifdef in a union)

Format the audit record:

- your_log_callback
- audit_log_format
- common_lsm_audit


Security module interface
-------------------------

Why have an interface?

- sysfs entry
- load or change rules
- read gathered stats
- module config
- avoid adding syscalls or ioctls

Mechanics for sysfs:

- sysfs_create_mount_point
- register_filesystem
- kern_mount


Security module stacking
------------------------

Stacking minor modules:

- ``<module>_add_hooks``

Stacking major modules:

- one at a time
- boot line
  - ``security=module``
- there is a way to cheat

Future:

- extreme stacking is under development


Wrap up
-------

Have a good reason:

- do something useful
- it should be something the kernel can and should do
- follow up with user space support and documentation

Don't reinvent the wheel:

- Generic has been done
- It's the 21st century
- No one liked Bell & LaPadula
  - or SELinux
  - or Smack

Show us something new:

- a model for Application Resources has not been done
- sensor-based controls could be fun
  - use webcam?!
- security doesn't have to be dull


Questions
---------

What is it that selinux can't do?

- Not do the things you don't care about
- e.g. you can't opt out of type enforcement
