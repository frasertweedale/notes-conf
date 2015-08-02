What is future of identity in OpenStack (Keystone)?


- Federation!
  - User-to-keystone authentication
    - Not keystone-to-LDAP or whatever
  - More protocols than just SAML
  - OpenID Connect really well supported
  - Kerberos
  - WebSSO
    - Q: what is this?
    - A: like social auth, but e.g. nova bounce thru keystone
  - TLS client cert
  - hopefully get rid of bearer tokens altogether

- Fernet tokens
  - best parts of PKI tokens and UUID tokens
  - UUID payload ; symmetric verification
  - Q: are you using python-cryptography?

- Weren't UUID or PKI tokens enough
  - PKI tokens were giant
    - Q: why not EC?
    - A:
      - problem wasn't the size of key/sig, but size of payload.
      - payload contains id and roles.  roles -> permission mappings
        live in services.
      - revocation was an issue with PKI tokens
        - cache expiry is just easier

- Tons of technical debt
  - KeystoneAuth library
    - for all the auth things
  - Let's get rid of V2 API everywhere; use V3

- Keystone Client
  - no longer going to be doing session code, authn code, all the
    stuff that everyone uses
  - there will be no keystone client CLI
    - we're done; we have put zero effort into it for 2yrs; major
      errors
