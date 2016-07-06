OpenSSL after HeartBleed
========================

Tim Hudson <tjh@openssl.org>, CryptSoft


- April 3, 2014: Heartbleed.  Re-key the internet
- The first "branded" vuln, lots of PR work
  - positive and negative ramifications for future vulns and malware
  - has changed how security researchers describe what they've done
  - vast majority of pop do not know what OpenSSL, but understood
    for first time that a major bug could compromise *their*
    internet security.
- Major media coverage
- A very simple bug; buffer overflow

"Sky is falling"

- There were serious vulns before, and after, HeartBleed
- Some vulns with much higher impact, got much less coverage

What was heartbleed?

- contributed code for TLS/DTLS heartbeat failed to validate a
  buffer length.
- OpenSSL team review missed it
- other team members missed it
- multiple external security reviewers and auditors missed it
- OpenSSL external devs and users missed it
- Security review teams in major orgs missed the bugs
- *all* existing code analysis tools missed the bug
- bug allowed client to attack server *and* server to attack client

Life before heartbleed:

- project had effectively become somewhat moribund
- releases were not pre-announced; no documented policies
- source code was complex and arcane, unapproachable
- hard to maintain, harder to contribute
- main developers overworked and overcommitted
- proj donations minimal (<USD$2000pa)
- 2012..2014: only two developers with >100 commits in tree
  - the next-nearest person had comparatively few contributions
  - so, basically a two-developer project

How did we let it happen?

- little time spent community building
- long lead time to understand code
- static project team membership
- need to focus on consulting dollars (FIPS140) to keep proj alive
- no ability to make, announce and keep to plans
- all added up to "ultra-cautious" about any change

After-effects

- Wide recognition of dependency on critical under-funded projects
- creation of Linux Foundation Core Infrastructure Initiative, a
  multi-million dollar effort to add effective resources to open
  source projects that make the Internet work.
  - There are no strings attached to this funding
- First ever face to face (11 of 15 team members) meeting in Düsseldorf
  - Many developers had never met even one of the other developers
- Drafted major policies
  - release strategy
  - security policy
  - coding style
- 15 project team members (vs 2)
  - Two full-time funded by CII
- Massive shift in project momentum
  - many more commits
  - greater influx of defect reports
  - much faster rate of closing defects
  - many more forks

What have we been doing?

- security researchers more actively looking for issues
- more fuzzing
- increased focus on automated testing
- static code analysis tools rapidly updated
- reported issues analysed much faster
- formal audit (external)
- **mandatory team member code reviews**
  - disputes go to full team vote

Project roadmap:

- https://www.openssl.org/policies/roadmap.html
- Retain commitment to "cryptography for the real world"
- Acceptance that project is more than a TLS stack
  - people use it for crypto primitives
  - will continue support for less common platforms

v1.1:

- better default key sizes and algos
- new build system
- new library init/final auto-handling
- async API for TLS, Intel HW, pipelining, IPv6, DANE TLSA
- ChaCha20, Poly1305
- TLS state machine rewrite; TLS version nego rewrite
- Opaque data types

FIPS140

- for five years, revenue from this work kept project alive
- anyone who wants to do business with or sell to US Govt
- validation process is time consuming
  - June 2002 .. March 2006 from go to final award
  - FIPS 2.0 module *only* works with v1.0.x
  - major update will be required for v1.1; no committed funding yet

Remember the lessons:

- relying on any single individual to perform superhuman feats
  ultimates results in disappointment
- code reviews actually require reviews to review code in detail
- assuming that users will review code is flawed strategy
- assuming that automated code analysis tools by themselves can
  completely replace experienced code reviews is incorrect
  - you absolutely should be using them, but not on their own

Future directions:

- we'd like more contributions
  - download and test pre-releases
  - join openssl-dev and/or openssl-users ML
  - report bugs
- we'd like more awareness of what's important to non-TLS users

Q&A:

- any cooperation with GnuTLS authors?
  - we've a good relationship with GnuTLS sharing TLS protocol
    defect information.  info flows in both directions.  likewise
    other TLS projects.
