Managing Intenral PKI system at Scale with Anchor
=================================================

or: *why revocation is hard*

Stanisław Pitucha (formerly HP Cloud)

What happens if a certificate's private key is compromised?

- Hopefully you notice! (logs, IDS, notice use elsewhere)
  - realistically not going to be < 1d
- Revoke the key
  - CRL
  - CRLSets (ship CRLs in-browser)
  - OCSP
  - OCSP staping
  - Expiration

- have you ever specified CRL path when using CURL?
- do you know if your https lib supports OCSP?

- the new idea
  - expire early, expire often
  - remove humans
  - define rules of what's allowed
  - add "multi-factor" server authentication
    - have: cred/token
    - are: IP addr
    - know: dedicated channel communication w/ hypervisor/infra

- implementation
  - *Anchor* project
  - created originally for/in HP Cloud
  - official openstack security proj
  - extremely simplified CA
  - stateless (easy clusters)
    - cf. Dogtag

- pluggable components
  - authentication
    - e.g. ldap, keystone, flatfile, whatever
  - req validation
    - is the request for service.example.com?
    - is it from the addr range that's supposed to host that
      service?
    - add missing SAN
  - req modifications
  - signing backend
    - openssl
    - PKCS #11 backend

- safe X.509
  - *pyasn1*
  - no native (de)serialisation code
  - only signatures are generated outside of python code

- "simple" certificates
  - serial <- random UUID
  - no Certificate Policies extn (by default)
  - minimal extensions (usually SAN)

- simple requests
  - stateless
  - only one request API
  - immediate result (signed cert, or nada)

- deployment
  - requires own CA / intermediate CA
  - periodic refresh
    - *certmonger* or cron + curl
  - service restart/reload to read new cert
    - a problem without proxies
    - stunnel helps where apps don't care

- wins
  - refresh becomes normal event
  - expiry automatic
  - permanent expiry of service is automatic
  - less human involvement in process
  - incidental: generated certificates can be reported to config
    manager db.

- issues
  - daily cert refresh becomes critical part of infra
  - low incident response time required
  - not all software deals well with cert refresh

- also from HP Cloud security
  - Bandit: Python code sec scanning
  - Gas: Go code sec scanning
  - Killick: PoC semi-automated CA based on Anchor
  - Recon: system hardening canner

Questions
---------

- How long before expiry do you start trying to get a new cert
  - at HP Cloud: 48 cert, start trying after 24h

- is anchor useful if you're not running openstack?
  - yep

- CT?
  - log to CADF audit system (public/private cloud audit system)

- certmonger renewal helper - integrate w/ certmonger?
  - should be under tools/ in the main repo
  - certmonger state machine did not encompass single request,
    immediate response
