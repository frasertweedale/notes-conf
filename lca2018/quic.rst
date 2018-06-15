QUIC - The mother of all internet transports
============================================

Jana Iyengar - Google, IETF QUIC Co-editor

- QUIC was built for HTTP transport
- improved app performance
  - YouTube video rebuffers: 15-18%
  - google search latency: 3.6-8%
- 35% of Google's egress (7% of internet)
- IETF WG formed Oct 2016
  - modularise & standardise QUIC

What is QUIC?

- on top of UDP. Replaces TLS and most of HTTP/2.
- shim for HTTP over QUIC
- QUIC Crypto handshake algorithm
- crypto handshake and transport handshake are combined
  - TLS 1.3 has satisfied some of the needs that inspired QUIC
- TCP-like congestion control, loss recovery
- QUIC when standardised will use TLS 1.3 instead of QUIC Crypto

HTTP/1

- 1 round trip for TCP; 2 RT for TCP
- client requests each object explicitly
- head of line blocking
  - clients use multiple connections (not ideal)

HTTP/2

- HTTP/2 streams solve HOL blocking in single connection

Unresolved problems in HTTP/TCP

- connection setup latency
- middlebox interference with TCP
- TCP HOL blocking

How does HTTP over QUIC work?

- connection setup: 0 rount-trips to a known server (common)
- 1 round-trip if crypto keys are not new
- 2 round-trips if QUIC version negotiation required (infrequent/rare)
- QUIC streams at transport level -> no HOL blocking

Design aspirations:

- deployable and evolvable
- low-latency conn establishment
- stream muxing
- better loss recovery and flexible congestion control
  - richer signaling (unique packet number)
  - better RTT estimates
- resilience to NAT-rebinding
- userspace (hence being built on UDP)
  - NATs don't know how long to keep a UDP binding.  They have a
    timer.

UDP is not a transport; it is a building block for transports

- Q: why didn't just put QUIC on IP?
  - A: heard of SCTP?  It was never deployed. The problem was not
    fully understood at the time; or rather, we had our heads in the
    sand about middleboxes.
  - how often to people upgrade their routers?
  - also we didn't want to have to put it into all the OSes. Had to
    be userspace to deploy and iterate faster and avoid duplicate
    effort.

Metrics

- search latency: user enters search term -> results page fully loaded
  - mean % reduction: desktop 8%, mobile 3.6%
  - HUGE improvement (>10%) on higher latency connections
    - due to fewer round trips

- QUIC improvements by country:
  - South Korea: 10.1% reduction in mobile rebuffer rate
  - US: 3.4% search latency, 12.9% mobile rebuffer rate
  - India: 13.2% desktop search latency, 20% mob/desk rebuffer rate

Network Ossification

- middlebox ossification
  - vendor ossified *first byte* of QUIC packets (flags byte)
  - ...since it seemed to be the same on all QUIC packets
  - broke QUIC deployment when a flag was flipped
- encryption is the only protection against network ossification

Q&A

- Q: what other protocols over QUIC
  - A: Google is looking at WebRTC.  ATM we are constraining
    ourselves to HTTP/2 just so we can get stuff done.

- Q: How does client choose to use QUIC?
  - A: host/client-local policy, or server can advertise QUIC
    support over HTTP, for the next time.

- Q: does QUIC facilitiate content caching?
  - A: not explicitly.  everything is encrypted.  community as a
    whole is interested in how to solve this.

- Q: high speed, high latency.  how does QUIC perform?
  - A: QUIC's fewer round trips so that's good
