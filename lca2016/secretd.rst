Secretd: storing and distributing secrets
=========================================

Tollief Fog Heen / @tfheen, fastly


What is the problem?

- code can be secret
  - need to protect from unauthorised modifications
- configuration can be secret
  - need to protect from unauthorised modifications
- credentials are secret
  - you don't publish API keys, DB passwords, TLS private keys, etc
  - need to protect from unauthorised modifications
  - need protection from disclosure

Some history:

- secrets are in...
  - code
  - config file
  - a pre-encrypted store
    - e.g. gpg encrypted file, per-machine gpg keys
    - prod creds on dev machines -> compliance issues (e.g. PCI-DSS)
  - an online store


Problems with most stores

- complex or insecure
- manual work to re-encrypt
  - e.g. adding password for new server to gpg-encrypted file
  - cyypto-wise it is fine; UX-wise it is bad
- updating is hard


Requirements for a fix:

- dynamic environment support
- central storage
  - PCI-DSS requirement
- policy-based access controls, live
- APIs for updating

What was our use case?

- Hardware (re)bootstrapping
  - we run on physical machines; no persistent storage
  - potentially use TPM to authenticate machine
- hand-off / live hardening
- PCI-DSS: auditing


Existing options (one year ago)

- pwstore
- chef-vault
- (Hashicorp Vault)


(Some) options today

- pwstore/chef-vault: pre-encrypted
- etcd: X.509
- Hashicorp vault: distributed, complex, TTL on secrets


Secretd

- go, SQL, ssh, tree structure, positive ACLs, PostgreSQL
- Apache license
- client talks SSH to server (only for authentication)
- server forks off ``secret-shell`` which talks to ``secretd``
- ``secretd`` talks to PostgreSQL (over TCP or Unix socket)

Database structure:

- principals have group
- acls reference groups and secrets
- acls have "acl_type"s

What's missing / future development

- doesn't encrypt secrets on disk
  - going to use libsodium wrapper for go
- admin tools / other UIs
- auditing
- tool integration
  puppet / chef / ansible et al
- enrolment key support
- TPM integration


Live demo
---------

Live demo showed:

As admin:

- enrolment (by client ssh public key)
- add group (ACLs act on groups only, not individual users)
- add client as member of group
- create an empty secret
- set ACL to allow group to access secret

As client:

- update secret
- fetch secret

SSH server:

- uses stock openssh-client
- add client to ``authorized_keys``
- plenty of locking down
  - principal is hard-coded in force-command (``command="..."``)
  - ``no-agent-forwarding``, ``no-pty``, ``no-X11-forwarding``, etc


Conclusion
----------

Should I use it?

- NO
- Secretd is a prototype
- limited tests (esp. for SQL bits)


Questions
---------

- Why PostgreSQL?
  - because I wanted a database, not an SQL interface to a flat file

- With advent of row privileges in PostgreSQL, have you considered
  using that directly?
  - Given they didn't exist one year ago, I did not

- How to protect SSH private key on client?
  - Uses SSH host key
  - If that becomes insecure, then you need to look at how you
    manage your systems

- Distributing the ``authorized_keys`` file? (Jamie)
  - you can use built-in replication in PostgreSQL
  - periodically regenerate they key ???
  - (didn't seem to really understand the question)

- PCI-DSS - is it truly forbidden to store keys on target machines
  *encrypted*?
  - Get legal advice
  - Do you want to risk it?
  - If they keys aren't there (even encrypted) they cannot be
    compromised
