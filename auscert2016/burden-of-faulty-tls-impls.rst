TLS - The Burden of Faulty and Outdated TLS Implementations
===========================================================

Hanno Böck @hanno https://hboeck.de/

- Freelance journo (Golem.de, Zeit Online, taz, LWN)
- Fuzzing Project (supported by Linux Foundation Core Infrastructure
  Initiative)
- Bulletproof TLS newsletter (monthly)


Recent attacks:

- BEAST, CRIME, BREACH, Lucky13, RC4, gotofail, Heartbleed, BERserk,
  POODLE, FREAK, Logjam, MACE, Invalid curve attacks, DROWN and more
- Many of these attacks where described in theory several years
  earlier.
  - We are not learning from mistakes
  - When the cryptographer comes and says "here's a weakness", we
    should listen to them!

Deprecation is hard:

- deprecated SSLv2, SSLv3, RC4, SHA1, small DH groups
- old clients that cannot be updated
- DROWN example:
  - 2011: RFC 6176 (prohibit SSLv2)
  - 2016: DROWN attack, 33% vulnerable
- Chrome disabling small DH groups
  - broke access to various router admin pages; many users mad!
  - routers, old security applicances (hardware or software) are
    particularly difficult
  - many of these routers etc were created AFTER the weaknesses were
    known
    - e.g. Cisco RV042G was released in 2012.
- Problematic vendors:
  - 2010: Nokia/MS Lumia 800 (only supported SSLv3)
  - POODLE: Mail services disabled SSLv3.  MS refused to deliver fix

It is happening again

- At some point we want to deprecate TLSv1.0/1.1 due to padding
  oracles
- 2016: a lot of current software only supports TLSv1.0
  - e.g. Apple Mail
- Why is it acceptable to deliver sub-standard crypto?

TLS implementation bugs:

- Version intolerance (always a server bug)
- Browsers work around by trying to connect with inferior versions
  - "protocol dance"
- Virtual Host Confusion attack (Black Hat 2014)
- POODLE
  - Padding oracle attack against SSLv3 combined with version
    downgrade attack
- SCSV (RFC 7507): workaround for a workaround that got standardised
- Browsers have now largely removed downgrade behaviour
  - When we get TLS 1.3, we'll probably have to add them back again!

F5 load balancers

- Some F5 load balancers failed with handshakes between 256-512 bytes
- Soln: TLS Padding extension

Omitting checks

- Impls need to check for things like padding, MAC, etc.
- POODLE TLS (2014) because some implementations didn't check padding
  - F5, A10, Fortinet, Cisco, IBM, Juniper

More POODLEs

- Maybe you only checked some bytes of the padding?
  - Cisco (Cavium), Citrix, GnuTLS
- *There are more POODLEs in the forest*, Yngve Petterssen 2015

MACs

- MACE: complete omit MAC check (no authentication)
  - F5, Cisco, Fortinet
- Don't check FinishedMessage
  - F5, Juniper
- *The POODLE has friends*, Pettersen, 2015

RSA-CRT

- Chinese Remainder Theorem: split private key signature operation
  into two calculations
- Dangerous: if one calculation goes wrong private key is leaked
- Attack described in 1996
- *Factoring RSA Keys With TLS Perfect Forward Secrecy*,
  Florian Weimer (2015)
  - Several faulty implementations discovered

Zero-length extension

- server reject connection if last extension has zero bytes
- GCM nonce reuse
  - if one uses same nonce and key twice everything falls apart
- TLS standard gives no guidance on how to select nonce
  - counter is secure
- Some impls get it wrong:
  - duplicate nonces (Radware, Cavium)
  - random nonces (IBM, A10, Sangfor)
- *Nonce-disrespecting Adversaries: Practical forgery attacks on GCM
  in TLS*, Böck, Zauner et al 2016
  - VISA website was affected

- OpenSSL has many options to enable/disable workarounds for bugs
- Most are undocumented
- Many are now disabled

CVE-2014-3570

- bug in BN_sqr() function of OpenSSL
- produces wrong results in some cases
- fuzzing is effective way to find bugs like this
  - several similar bugs in OpenSSL, NSS, Nettle
  - often carry propagation bugs
  - fuzzer to produce inputs; diff implementations

Bugs in calculations

- hard or impossible to test remotely

Almost every imaginable TLS impl flaw can be found in wild

What can we do?

- Test tools
  - SSL labs
  - tlsfuzzer
  - et al
  - getting better but none is satisfying
  - no tool covers all the issues described here

Conclusions:

- old attacks keep coming back
- impl bugs introduce security risks for themselves and for the
  ecosystem
- avoid reimplementing TLS
  - OpenSSL has its problems, but better than rolling your own
- vendors ship sub-standard TLS stacks - this is not acceptable
- We need better test tools

Questions:

- TLS 1.3 when?
  - "soon", most likely this year

- How can we encourage browser / TLS client vendors to stand their
  ground and NOT write workarounds, forcing vendors with broken
  impls to fix their product, or at least feel some heat from users.
  - It's hard - the browser vendors take the heat
  - Better test tools could help, i.e. "before we buy your box, we
    will test its compliance"
  - I disagree with browser vendors but understand why they write
    workarounds.
