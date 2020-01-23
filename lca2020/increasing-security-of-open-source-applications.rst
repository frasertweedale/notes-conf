Increasing the security of open source applications
===================================================

Peter Burnett, Catalyst

Case study: Moodle
------------------

learning management system; PHP; est 1997

- good: pwd storage, access controls
- bad: pw policy, authn flow not layered, hard to customise


Efforts to increase sec in pain points

Password security
-----------------

- NIST password standards are sane
- avoid restrictions that encourage repetition, sequencing and char
  subst
  - i.e. don't force a pattern
  - check for dict-based and identifying pwds
  - check for compromised pwds (HIBP)

- HIBP
  - free, easy
  - send first 5 pwds to the HIBP
  - get matching hashes back
  - check for a match

- dict checks
  - pwds based on single word are weak
  - don't have to perform exhasutive check
  - top 10k EN words by freq cover 97% of words used in pwds

- identifying info
  - name, bday, city
  - if you do store user's identifying data, check pwds against it


Moodle account authentication
-----------------------------

- support in Moodle core for common authn backends e.g. LDAP

- need another layer of protection in place e.g. to test/protect
  against compromised password

- everything was "on top of password authn"; web sso was not
  possible

- work was done to modify / make more flexible the authn flow
  - additional logic / checks can be injected
