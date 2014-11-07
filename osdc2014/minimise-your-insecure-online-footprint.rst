Embrace the Impact: minimise your (insecure) online footprint
=============================================================

Peter Mosmans

- peter.mosmans@go-forward.net
- work in infosec (pen tester)
- passionate about (online) freedom
- love american IPA
- https://www.go-forward.net
- https://www.onwebsecurity.com

What is privacy?

- the right to be left alone
- the right to stay anonymous
- online privacy: communication between client and server

Agenda

- L1: your browser
- L2: whose local network?
- L3: our internet
- L4: their server
- reduce your footprint (Tor, Tails)

Thread modelling:

- which assets to protect?
- look at the attacker.
  - neighbour, boss, other company, state-sponsored entity?

Fingerprints and footprints:

- Know what you leave behind.

L1: browser:

- active fingerprinting
  - explicit: (ever)cookies
  - implicit: machine characteristics (fonts, plugins)
- Passive fingerprinting
  - http protocol headers
- Browser fingerprint uniqueness test:
  - panopticlick.eff.org

Mitigations:

- use generic OS
- use generic browser
- disable JS
- use incognito/private browsing

L2: local network:

- TLS cipher; good configuration can be difficult
  - downgrade attacks
- Pen-testing gadget.  Raspberry Pi tool.
- ``arpspoof`` for imposter gateways
- ``sslstrip`` redirect https to http

Mitigations:

- be aware of "surroundings"
- use a VPN connection
- prohibit non-secure connections
- type https:// directly
- use EFF's HTTPS everywhere plugin
  - also: HTTP nowhere (disallow all non-secured HTTP)

L3: internet

- passive fingerprinting
  - IP/TCP/HTTP protocol headers
- lots of hops == "men-in-the-middle"
- lots of metadata

Mitigations:

- minimise third party includes
- reduce data intake: minimise logging
- VOTE
- https://stopthespies.org

L4: their server

- identification and authentication
- explicit versus implicit trust
- know who / what you trust

Redirection or misdirection?

- Clients SHOULD NOT include a Referer header when switching
  from HTTPS to HTTP (RFC 2616)
- ∴ Google redirects to HTTP first to get the referer into the HTTP
  request.
- What does DDG do?
- Workaround: https://encrypted.google.com/ preserves privacy.

Mitigations:

- Configure browser to disable OCSP
- Use OCSP stapling when possible
- Make TLS standard for *all* websites.

Reduce for footprint:

- Tor
- Tails

General recommendations:

- make privacy and security the only option
- stand for privacy online
- be aware of your foot and fingerprints
- spread the word on privacy

*It's not the technology that's bad; it's the people who abuse the
technology that are bad.*
