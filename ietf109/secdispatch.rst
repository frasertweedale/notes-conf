DANE for IoT (client auth)
==========================

- Original dane WG concluded, but regular requests to continue the
  work

- Assumption: client has a DNS domain name identity.
  Corresponding TLSA record published there.

- New TLS extension to convey client identity

  - convey client DNS idnet when used with TLS raw pub key auth (RFC
    7250)

  - ECH (encrypted client hello) for privacy

- Object security

  - Neither TLSA nor SMIMEA are suited

  - new RR = longer devel and adoption lifecycle

  - therefore, current proposal is to expand scope of TLSA to cover
    object security

- Who wants to advance this?

  - NIST, ICANN, LoRA Alliance (LP1 device manufactuers)

- Question: pls elaborate on IoT motivation / use case?
  Answer: (coming on next slide!)

- Detailed motivation:
  - Identity = a name + a way to prove ownership of the name

- value of identity systems:
  - how widely recognised is the name?
  - how resistant is the proof-of-identity to impersonation

- IoT challenges:

  - discovery of public key for message authn/encr -> proprietary
    APIs

  - subjective entity names -> namespace collision across CAs

  - manufacturer to enterprise PKI bootstrapping -> costly and time
    consuming

  - constrained platforms; decoupled architecture
    - small RAM, small storage (as little as 4MB flash)

- DANE for certificate discovery (deferred)

- DNS is a widely recognised namespace; public key crypto is
  resistant to impersonation.  DANE binds DNS names to public keys.

  - Eliminates naming collisions across CAs

  - SDK for cert discovery is already in the OS

  - attribution to responsible party via DNS hierarchy

  - current public key is always in DNS (simplify certificate
    rotation)

  - no need to distribute CA certs to devices

    - DNS solved hosts file distribution; DNS can also solve CA cert
      distribution

DANE for IoT: implementation
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

DNS labels:

- ``_device`` for org / delegation point

- ``a1b2c3._device.example.com``

- does not carry RFC 8162 complexity for hashing email local part
  for DNS name

- sub-identities represented by left-hand labels (see BCP 222)

- underscore challenge: disallowed for publicly trusted
  certificates

- TLSA record type

- revocation by record deletion

Simplification:

- network authn (802.1x, EAP-TLS)

- transport authn (mutual TLS; DNS-SD/mDNS companion)

- simplify RADIUS config/management
  
  - allow list is just a list of permitted entity DNS names

  - no CA cert management

  - less need to re-key enterprise PKI

- simplify support for raw public keys

  - RFC 7250 - TLS with raw public keys 


- DANE for cert discovery

  - "DANE-lite"; TLSA for cert discovery only, still validate via
    PKIX

  - ultimate goal is to use DNSSEC authenticated TLSA everywhere

  - spare deployment of DNSSEC is a challenge to DANE adoption

  - immediate benefits for JOSE/COSE/OSCORE?

- Working group placement: TLS, UTA, DNSOP?

Outcome: more discussion. BOF and iot-onboarding@ ML


SVT - Signature Validation Token
--------------------------------

- simple solution for validation signature in the distant future

- Swedish Government funded research for Archivable electronic
  signatures

- open source implementations; eduSign signing service for school
  system.

- Approaching IETF for standardisation - not rubber-stamping.  We
  have v1, let's do v1.1

Requirements:

- full integrity protection of signed document and its signatures

- easy to implement

- predictable outcome of future validation

- avoid size explosion and cascading evidence collection

- avoid repeated storage of large common validation data object
  (e.g. CRL)

- evidence renewal without increasing validation complexity

- fast verification

- offline: possible to validate without access to external online
  services

- compatible with current document parsers and signature validation
  software

Evidence:

- while sig cert is valid:
  - is it revoked?
  - when?
  - was sig created before revocation type?

- after cert expires:
  - a time when sig existed
  - validity status at that time

- when algorithms are no longer trusted
  - a time when the sig existed
  - the validity status at that time
  - the data that was signed (and the signature)
  - ...

- Cascading evidence collection: when each supporting evidence
  requires more than one new supporting evidence.  THIS is the
  problem that needs to be solved.

- Claim: we can reduce cascading evidence with ONE piece of
  evidence.  The SVT.

- SVT includes:
  - issuer, time, algorithms, expiry?
  - claims: hashes over signed data, sig content
  - cert references, verified timestamp evidence

- Based on JWT format

- Why IETF:
  - based on IETF standards
  - no other standards org is doing it
  - might fit with LAMPS WG

- Was dispatched at IETF 107 for more discussion

OUTCOME: more discussion in ML to be created; possible new working
group, not LAMPS WG.
