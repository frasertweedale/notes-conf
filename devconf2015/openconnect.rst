Openconnect VPN
===============

- Nikos Mavrogiannopoulos
- Senior Software Engineer, Red Hat Security Technologies

Agenda:

- History
- Features
- Protocol
- Usage
- Future


History
=======

- mid-2000s there were multiple open VPN solns
  - IPsec: \*swan, vpnc (layer 3)
  - custom protocols: openvpn, pptp (layer 7)

- a few proprietary solns advertised as SSL/VPNs
  - Cisco Anyconnect SSL VPN
  - Microsoft SSTP
  - all layer 7

- layer 7 has no requirement on intermediate routers

- SSL VPN
  - in marketing slang, SSL VPN means connection establishment over
    HTTPS.
  - advantage: not easily distinguishable from play HTTPS session

- Openconnect
  - started in 2008 as a client for Cisco Anyconnect VPN
  - SSL VPN
  - goals:
    - better portability
    - NetworkManager integration
    - support for smart cards and TPM keys
    - reduce actions performed as root
      - openconnect doesn't need to run as root if tun device
        available
  - currently runs on Linux, BSD, OS X, Windows, Android
    - iPhone is missing

- Openconnect server
  - started in 2013
  - goals:
    - standards-compliant userspace VPN soln
    - anyconnect VPN was the only known VPN protocol which support TLS
      1.0 and draft version of DTLS 1.0
    - VPN server that integrates client management
  - currently available for Linux and BSD
  - interoperates with Anyconnect clients


Features
--------

Protocol:

- session provides two tunnels
  - TCP (TLS) and UDP (DTLS)
  - connection first established on TLS channel
  - once established, data is carried over UDP
- protocol designed for no need of manual client config
- IPv4 and IPv6
- supports compression (deflate and ZLS)
- HTTPS-based protocol
- allows for password and public key authentication

Client:

- designed for zero manual configuration
- integrated with network managers (GNOME, KDE)
- supports PKCS #11 smart cards, TPM keys
- doesn't require root (if tun dev is preallocated)
- supports LZS, LZ4 compression
- TLS 1.2, DTLS 1.2, AES-GCM

Server:

- supports authn with password file, PAM, RADIUS
- support setting resource limits per client or group (e.g. cgroups,
  bandwidth)
- processing scales with num CPUs
- compressoin
- privilege separation between main process and workers
  - isolation of workers (using seccomp)
- isolated software security module handles PAM/RADIUS and keys


Protocol
--------

- POST XML containing version info to server
- server replies with form asking for username
- authn process continues until complete
- client CONNECT verb
- server responds with connection details in response headers
- Someone asked about SPNEGO support.  It is not supported (but
  probably could be)


Setup and administration
------------------------

Client setup:

- run openconnect with server name

Server setup:

- Generate server certificate
- Edit server configuration file
- Create users' password file
  - ``ocpassword`` helper program
- Run ``ocserv``

Server administration:

- ``occtl`` tool
- server status
- connection status
- connection info
- disconnect connections
- reload config
- originally used D-Bus but was changed to unix sockets


Colocatoin with web server
---------------------------

- via sniproxy / haproxy
- via TLS termination on web server
  - no certificate authn
- both options reqiure copy of data between processes (TCP)
  - not for high volume sites
- DTLS channel remains under control of ocserv


Future plans
------------

- client support more SSL VPN variants e.g. Juniper
- reduce cost of web colocation


Resources
---------

- www.infradead.org/openconnect
- www.infradead.org/ocserv
