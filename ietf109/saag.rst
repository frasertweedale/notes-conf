Security Area Open Meeting
==========================

AD-sponsored drafts
-------------------

- A File Format to Aid in Security Vulnerability Disclosure
  https://tools.ietf.org/html/draft-foudil-securitytxt-10

New work
--------

- Re-open OPENPGP WG.

Attack language taxonomy
------------------------

Discussion of differing, ambiguous, conflicting or unclear language
about attacks and threats.  e.g. "on-path", "man-in-the-middle",
etc.  Is a document needed?  Precise definitions are needed.

Requirements for Internet PKIs - Ryan Sleevi
--------------------------------------------

- For the open internet, not orgs

- "Painful lessons from Web PKI, and how you can avoid them"

- History; "the PKI" (1988), Netscape and SSL (1994), IETF 33 (1995)
  SSL BoF, IETF 34 pkix WG, X.509v3 (1995), many roots of trust,
  many ways to name things; RFC 2459 (1999)

- Web PKI vs Internet PKI (1995 - 2007)

  - Web PKI assumed user interactivity; PKIX did not

  - Web PKI assumed strong connectivity to Internet, PKIX did not

  - Web PKI was focused on browsers at first, but got messy quickly

  - Result: blurry lines between Web and Internet PKIs, default CAs,
    and policy expectations of CAs

  - Anyone could be a CA, as long as you were "trustworthy".
    Reputation management rather than risk management.  Audits came
    later (~1999-2001)

- CA/B Forum (2007 - 2011)

  - blurring of Web PKI (which started with legal names) and
    Internet PKI (which focused on Internet naming) created conflict

  - CAs unhappy with competitors "only" validating domain names

  - Browsers unhappy that CAs inconsistent in how they named things
    and what parts of PKIX they supported

  - Phishing was a big issue.

  - "Solution": EV certs

- Web PKI 1.0: Baseline Requirements (2011 - 2014)

  - EV raised the ceiling; but still concern about what the floor
    was.

  - DigiNotar (2011) shook trust in whole system

  - BR adopted shortly thereafter that preserved both DV and OV cert
    policies.

  - Deprecation of 1024-bit certs (adopted 2011, sunset 2013)

  - Deprecation of SHA-1 (adopted 2014, sunset 2016)

  - Browsers began to tell CAs what they can and can't do.
    Application community defining policies for CAs.

- Web PKI 2.0 - deprecation of legacy Symantec PKI (2017 -)

  - Different PKIs for different clients

  - Different PKIs for different protocols

Requirements for a PKI:

- Assumptions:

  - for the Internet
  - for a generally available service
  - that should work "out of the box"
  - interop

- WebPKI assumptions:
  - Client can talk to server
  - but not necessarily CA.
  - Probably have communication with browser vendor (for updates etc)

- Connectivity affects every aspect of technical design and policy

Naming

- context-dependent (civil/legal names) or globally unique (DNS,
  "The Directory")

- How many naming authorities involved?

  - DNS zones can have multiple naming authorities.  Should that
    be reflected in PKI hier?  Should that be addressed via lookup
    e.g. CAA?

  - rfc822 names have local-part@global-part.  Should it be done
    through constraint sub-CA? Should users be able to bring their
    own address without global-part's explicit approval?

- Services on different ports, sharing same name, e.g. SMTP and
  HTTP.  Should it be allowed?  SRVName?

- Are names stable or do they frequently change?

  - Domain names change more frequently than annually!  Is it a
    problem for lifetimes?  Revocation?  It depends!

- Are names unambiguously, interoperably machine parsable?

  - domains: kind of?  "preferred name syntax" vs underscores

  - URLs: real world of running code is unbearably messy

Issuance

- DNSSEC:

  - pro: every zone can be independently managed

  - con: every zone has to be practically managedk

- Web PKI:

  - pro: set of CAs that can issue for any domain

  - con: set of CAs that can issue for any domain

  - assumption: DNS is still "always" consulted by clients, so
    DNS/BGP have to lie or be coerced

- PKIX

  - pro: everyone can federate their PKIs using cross-signing and
    bridges so you can be selective in who you trust

  - con: everyone ends up federating their PKIs through cross
    signing an dbridges, making it unlcear who you actually trust

Who is the policy authority?

- common policy is necessary for security and interop

- every PKI needs a new policy; every policy needs a new PKI

- "someone" who can answer these questions


Certificate profile

- when everything is extensible, nothing ends up being extensible

- RFC 5280 is the start point, not the end point

- PKI is intrinsically part of, and depends on, the protocol being
  used

  - revocation and expiration depend on connectivity

  - acceptable CRL / OCSP sizes depends on bandwidth

  - acceptable protocols (LDAP, HTTP, etc) depend on purpose

  - naming authority determines how many CAs your PKI can expect,
    which affects how much time is spent verifying the cert, and how
    much work attackers can cause you to do

  - trust anchor management depends on updatability and connectivity

Audits

- do you like checkboxes or do you like descriptions?

  - checkboxes (e.g. ISO 17021/17065)

    - good for interop/compliance testing.

    - bad for security testing; list everything bad as a negative
      test

  - descriptions (e.g. ISAE 3000)

    - good for security: describe system and control for how they
      ensure they meet the requirement.  flexibility in how security
      goal is achieved.

    - bad for interop/compliance testing.  Good intentions don't
      always lead to good result.

  - either way, it will be specific to your protocol and PKI needs
    and requirements.

- who performs audits?
  - do you perform them? policy authority? independent third party?

- who defines criteria?

- are you more interested in past or future?

- who will verify audit results?

Conclusion

- there is not a single internet PKI.  PKis must be tailored to the
  use case, clients and protocols.

  - RFC 5280 provides tools necessary to build a car, pool or house.
    be careful not to build a floating RV because the maint costs
    are high!

- it is possible to build interoperable PKIs that are simple, easy
  to use and work OOTB.  Not trivial or cheap to do!

- ensure every single byte is critically necessary for your use case

  - every extension point introduces new challenges to interop,
    compliance and security

  - GREASE is hard to apply to PKIs

    - You need the cooperation of the CAs, and you need someone to
      send the certificate to someone else.  There's only one
      certificate.  If it fails, your certificate is broken (for
      that client), and you probably have to get another one.

- consider protocols that support multiple PKIs

  - S/MIME is OK because it supports multiple sigs; allows for
    versioning and migration

  - TLS doesn't allow to provide *multiple certificates*
