libtls: rethinking the TLS/SSL API
==================================

Joel Sing <jsing@openbsd.org> (Canonical, libressl co-founder)

- make it easier and safer to write apps that use tls
- new tls lib with clean, obvious and simple API (hopefully)
- designd to make it easier to write foolproof apps
- fourth component of libressl
- uses libssl "under the hood"
- TLS is complex and messy
  - but it should be easy to use!

libtls philosophy:

- easy to use as possible
- safe and secure by default
- consistent, obvious, well documented
- support pledge(2), chroot(2) and other sandboxes
  - ensure access to files is deterministic
    - access at well known point, or not at all
    - accept config via memory instead of files

functions:

- ``tls_connect*()`` / ``tls_accept*()``
  - socket, fd (tls over pipes), take callbacks, etc
- ``tls_handshake()``
  - can be explicitly called, otherwise implicit by ``tls_read()``
    or ``tls_write()``

API design:

- consistent, deterministic return values (-1 or NULL on failure)
  - we do not set errno, and sometimes deliberately clear it
- ``tls_read(3)`` / ``tls_write(3)`` almost have
  ``read(2)`` / ``write(2)`` semantics
- opaque data structures
- take copies of strings and mem, instead of storing pointers
- no X.509 or ASN.1 exposed

General API design rules:

- keep it simple as possible
- don't be afraid to iterate
- only add features when there is code that actually uses them
  - ensures API is appropriate
  - helps to manage feature bloat
- have symmetrical functions (for client/server) where possible
- consist ways to indicate failure
- consider what user's might overlook

SNI:

- in libtls, for each additional certificate:
  - ``tls_config_add_keypair_file(server_cfg, cert_path, key_path)``


Questions
---------

- OCSP stapling.  What does libtls do by default, and what can it be
  configured to do?  Does it grok TLS Feature extension f.k.a. "OCSP
  no-staple"
  - client validates stapled OCSP response
  - server does not fetch response but can be told to send one
  - OCSP no-staple: dunno
