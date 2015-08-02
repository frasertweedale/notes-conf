Privsep - Angus Lee
===================

- use kernel capabilities to limit what a process can do


Brief history of Rootwrap:

- Attempt 1: run as unprivileged user, but allowed to do certain
  things via sudo
  - problem: sudoers becomes really long and hard to manage

- Attempt 2: add Rootwrap - a new, more expressive wrapper
  - ``sudo nova-rootwrap /etc/nova/rootwrap.conf``
  - Only one command in sudoers
  - filters match command, user, etc.
  - Has worked pretty well for a number of years in OpenStack

- Attempt 3: avoid repeated python startup cost
  - ``nova ALL = (root) NOPASSWD: /usr/bin/nova-rootwrap-daemon ...``
  - ``rootwrap-daemon`` outputs token on stdout
  - main process talks to ``rootwrap-daemon`` over unix socket,
    passing token
  - 10x speedup
  - uses python ``multiproessing`` lib
  - main process kills ``rootwrap-daemon`` when finished
  - uncertainty as to how long ``rootwrap-daemon`` should hang around
  - problem: insufficient context to make a security decision
  - problem: command lines are clumsy and often not intended to be
    security interface

- Attempt 4: *privsep*
  - based around python function calls, not command lines
  - *must* be easy for developers to use securely
    - otherwise they're not going to
  - use modern Linux security features
    - selinux
  - main process starts ``sudo rootwrap privsep-helper``
    - ``setuid`` and drop privs
  - exits when stdin gets closed
  - send ``(func, args, kwargs)`` on stdin
    - verify func is appropriately decorated and call
      ``func(*args, **kwargs)``
    - serialises result back on stdout
    - wireformat: original version uses messagepack; currently json
  - Q: can you run multiple privsep processes?
    - A: yes
  - Q: why stdin/stdout vs socket?
    - A: mostly to work within existing rootwrap deployment approach
    - No exposed communications point to attack from outside - only
      stdio with parent

Status:

- working prototype written as change against Neutron
- agreement at Vancouver summit to create oslo privsep proj
- Oslo spec in review
- co-exists with rootwrap so migration is boring
