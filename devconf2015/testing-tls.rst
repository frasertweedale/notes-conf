Testing TLS
===========

- Hubert Kario
- hkario@redhat.com


2014
----

- Basically every big crypto lib had a critical vuln
- Heartbleed
- OpenSSL CCS bug
- gotofail (Apple)
- Certificate handling problems in NSS and gnutls
  - gnutls considered some certificates to be CA certs when they
    were not
  - NSS didn't verify signatures correctly
- CVE-2014-6321 in schannel
  - Vuln allowed attacker to execute arbitrary code


Testing
-------

- Legacy code
  - not necessarily old, spaghetti code
  - no coverage
  - no unit testing
- Hard to refactor if there is a bug
  - hard to reason about side effects of changes


- 2011 study looked at open source projects
  - 20% used any kind of test planning
  - 40% used some kind of testing
  - just under 50% check coverage
  - 8% of problems were bugs in logic
  - 92% due to missing or incorrect error handling
  - 77% of bugs could be easily reproduced in a unit test

How do TLS libs compare?

-                OpenSSL  NSS  GnuTLS
- Test framework: N       Y    Y
- Number tests: 100-200  >7k  100-200
- Negative tests  N       Y    N


Test coverage (as ratio of test LOC to lib LOC):

- NSS bad, OpenSSL and GnuTLS abysmal


Invisible bugs

- timing attacks
- libraries and bad data
- TLS is hard to fuzz
  - (very) stateful protocol
  - most data is checksummed (post-processing of fuzzed data needed)


Fixing the problem
------------------

- don't want to duplicate effort
- create a single test tool that knows what an implementation
  supports and generates the correct requests / responses
- the fuzzer needs to understand the protocol
  - e.g. server should reject undefined extension


tlsfuzzer
---------

- generator generates valid messages
- fuzzer changes them and declares expected server response
- focusing on servers first (i.e. acting as client)
- end goal: CI for main implementations
- licence: GPLv2
  - tlslite is BSD
- https://github.com/tomato42/tlsfuzzer
- whole fuzzing session can be reproduced from seed
- related: Symbolic Executor
  - generate data to force branch execution
  - experimental technique
  - good crypto code avoids branching as much as possible
