HTTP By The Numbers
===================

Christopher Neugebauer

A field guide to understanding how you send stuff over the web.

You shouldn't have to worry about network protocols.

Things are happening (with HTTP) for the first time in ages:

- iOS and non-secure HTTP
- HTTP/2

Know what your code does on the wire.

Measuring network performance:

- Throughput
- Latency
- Multiple requests
- http2bin by Cory Benfield (HTTP benchmarking tool)
- Android app: gh:chrisjrn/bilious-spork
- NAT increases number of machines in IP space, at expence of TCP
  port space
  - most mobile networks are NATed
- TCP is an inevitable source of latency
  - 3-way handshake
- Effective TCP
  - one connection, kept open for long time
  - flow control can kick in

HTTP/1

- Plain text protocol
- One-shot protocol (one request per connection)
- Keep-Alive
  - Server doesn't close connection immediately after writing
    response
  - No extra TCP handshake
  - TCP flow control maintained
- Getting more connections:
  - HTTP spec permits only two simultaneous conns
  - Browsers ignore / are configuration (default for Chrome is 6)
  - Parallel connections to same host
  - Server resources at different hostnames


App Transport Security Requirements

- iOS 9 and OS X 10.11 apps "require" HTTPS


HTTP/2

- HTTP for the "modern web"
- Still req-resp
- Still stateless
- Same semantics
- Binary protocol
- HTTPS-only
- Multiplexing
  - can fire off N requests at once
  - prioritised responses
  - no socket renegotiation
- Server push
  - no round-trip delay for requesting things that are definitely
    going to be requested
- Can you use it?
  - Major versions of all popular browsers now support it
- ALPN (fka NPN) allows browser to determine whether it should send
  HTTP/2 or HTTP/1.1 request.
