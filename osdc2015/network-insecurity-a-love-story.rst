Network insecurity - a love story
=================================

Julien Goodwin

Bad assumption:

- Nobody's listening
- I'm too small a target
- My corporate network is secure
- Encryption is expensive
  - Computation cost is trivial


Storage

- all data at rest should be encrpyted
  - device level encryption if it has it
  - at drive, filesystem and/or file level
- don't forget about backups
- "encrypted databases" (not actually all that useful)


Transport

- very simple rule: don't put cleartext on the wire
- all data leaving a machine (or at most a secure rack) should be
  encrypted


Handling sensitive information

- if you don't need it, don't request it
  - does your service really need to know my mother's maiden name?
  - can you authenticate me with SSO?
  - inside a corporate network a good SSO system is wonderful

- Avoid storing it
  - example: outsource payment processing to avoid (most of) the PCI
    pain as well as toxic data

- Isolate it; monitor it
  - keep sensitive data away from other data; monitor for oddities


Handling user passwords

- hash using algo designed for passwords
  - bcrpyt, PBKDF2

- with per-system and per-user salt
  - per-system salt not stored in DB

- don't use password as input to other systems
  - example: user keys in cookies

- please don't use "secret questions" / password hints.


Network security
----------------

Firewalls

- stateful firewalls are a dangerous chokepoint in front of servers
  - try stateless ACLs instead

- remember to prevent IP spoofing

- if you must use then, avoid adding IDS/IPS, anti-malware etc

- most regulations (PCI etc) don't require stateful firewalls, but
  auditors will insist


NAT

- not a security measure
- a workaround for bad design


NAT traversal

- getting inbound traffic through a NAT
- either with assistance from NAT software (uPNP), or not
- UDP often causes this trivially
- *TCP Simultaneous Open* is one method for TCP


ICMP

- please don't block ICMP
- reqiured for various functions
  - ping, traceroute, etc.
- people who need to diagnose issues care
- ICMP loss may not be real packet loss
  - larger routers rate-limit ICMP
- some types of tunnelling hide from traceroute


Encryption
----------

- Please don't build your own encryption system
  - use standard implementations

Test your system anyway!

- Certificate validation
  - Hostname
  - Chain
  - Revocation
  - Validity period

- Interception
  - What happens when you MITM your system?


Useful tools:

- Qualys SSLlabs
  - for testing public HTTPS services
- testssl.sh
  - for testing non-public and non-HTTP services
- nogotofail https://github.com/google/nogotofail/
  - test clients for validation, etc
- mitmproxy https://mitmproxy.org/
  - useful to intercept traffic during development


Client authentication

- static keys compiled into apps should be assumed to be public
  - data encrypted with them equivalent to cleartext

- X.509 client certificate management is a pain, but works well once
  installed
  - AD can issue device client certifiates


Don't forget...

- HTTPS -> HTTP -> HTTPS redirects
- cookies should be marked "secure" and possibly "httponly"


Getting certificates

- Buying certificates used to be an expensive pain

- For now: SSL Mate https://sslmate.com/
  - command line tool & API to buy certs
  - cheap compared to most competitors

- Soon: Let's Encrypt


New application protocols
-------------------------

HTTP/2 (based on SPDY)

- Always (in practice) encrypted
  - unencrypted mode is defined, but noone implements it
- Benefits
  - header compression
  - server push
  - usable multiplexing


QUIC

- HTTP/2 over UDP transport
- experiments with congestion control & more

GRPC

- generic, protocol buffer based RPC system



Mobile and Enterprise Networks
------------------------------

Characteristics

- IDS
- IPS
- TCP "optimisation"
- firewalls (too many)
- VPN appliances that do weird things

Path MTU discovery

- some network transports don't use Ethernet's 1500-byte MTU
- path MTU discovery sends test packets to discover this problem
- MTU blackholes cause problems
- As does ICMP filtering

Buffer bloat

- some networks buffer too much data
- leading to multi-second ping times during congestion
- don't be afraid of dropped packets
  - usually better dropped than 5s late
- ICSI Natalyzr is a good test tool

Too many NATS

- double-NAT, triple-NAT & worse
- *Carrier-grade NAT*
- causes latency, MTU issues, state issues

Carrier VPNs

- *MPLS VPNs* are not encrypted
- even if they were, the carrier has the keys

IPv6
====

IPv6 deployment

- now broadly deployed on LTE networks
  - some don't even do v4 (464XLAT)
- Apple now require apps to run in v6-only* environment
- many of largest global consumer ISPs
  - In australia, still only Internode (for home plans)

IPv6 benefits

- unlikely to see NAT
- fewer middle-boxes messing with your traffic
  - latency win
  - Facebook saw 15% latency win on v6 from mobile networks

IPv6 security

- IPSEC no longer required for v6 impls
- privacy concerns due to SLAAC addressing
