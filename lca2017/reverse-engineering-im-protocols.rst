Lobbing cats into the walled garden
===================================

Eion Robb

- Reverse engineering IM protocols for Pidgin
- Do people still use {IRC,MSN,AIM,Yahoo,GTalk,...}
  - Yes.  More than you could possibly imagine.

- The IM Wars
  - sameroom.io list
  - https://ē.nz/chatt
  - http://nplusonemag.com/chat-wars
  - AIM blocking MSN
    - daily changes to auth
    - required downloading ads
    - buffer overflow code injection

- XMPP golden era
  - looked like everyone was going to use it
    - GTalk (2005), AIM XMPP (2008), Facebook (2011), MSN XMPP (2011)
  - lots of people still use XMPP, but has declined
    - theory: it's just not cool any more
    - theory: it's not so good for mobile devices
      - overhead
      - doesn't handle network transition well
    - theory: spam

The old days:

- Wireshark
- oSpy - Windows security libraries interception
- assembly debugging

Nowadays:

- JSON over Web
  - HTTP, HTTP/2, WebSocket
  - push, long-poll
- Protobuf / Thrift / MQTT / other off-the-shelf
- REST APIs

Why am I sharing?

- IM Freedom.org
  - US-based NFP
  - wiki of protocols
    - documentation for clean-room impl
  - use quite a lot by security researchers

IANAL

- what country's laws do you have to care about?
  - the country you're in
  - the country where the code is hosted
  - the internet police

- NZ law
  - §80 of copyright act 1994 allows reverse engineer
  - can't EULA away your rights
  - **can** be sued under contract law

- AU law
  - §10 Copyright Act 1968
  - "Ideas vs Expressions"
    - allowed to copy an idea, but not an expression of the idea
  - she'll be right, mate
  - use cleanrooming to protect yourself

- US law
  - allowed to rev eng and circumvent protection for interop
  - can EULA away your rights
  - DMCA takedowns are real

How to?

- tools:
  - mitmproxy
  - fiddler2
  - Carles Proxy
  - Browser Web Inspector

- pattern recognition

- clean room
  - keep a paper trail.  I use pen and paper
  - event stream / polling
  - URL & Method (GET / POST / PUT)

- cert pinning
