Federated Identity Providers and the Ipsilon Project
====================================================

- Simo Sorce

What is federation?
-------------------

- Dealing with users you do not control on your own
- To do that you need to trust a 3rd party

Trusting 3rd party:

- org wants to offer services to another which "owns" identities
- user's org controls what is disclosed about user
- user does not need additional credentials
- 3P does not need full view of user store

Trusting a 3rd-3rd party:

- fed also used when another party needs access to data *on user's
  behalf*
- known as *delegation*
- 3P only gets access to specific user data
- user/org is in control of permissions granted


Not federation:

- surrendering credential
- user's org has no control; breach of privacy
- user has no control on what 3P will do with creds
- 3rd party has liabilities it shouldn't want
- no SSO

Federation protocols:

- most fed protos are web/HTTP oriented
- some authn flows depend on user sitting in front of browser
- non-interactive modes are available in some cases
- delegation modes are non-interactive
  - but may depend on interactive modes for establishment
- SAML, OpenID, OpenID Connect, Persona


How does it work?
-----------------

Glossary:

- IdP
  - authenticates users
  - or provides enough data to verify an authn assertion
- Service Provider / Relying Party
  - server that needs authn by 3P IdP
  - the system the user is trying to access (directly or indirectly)


SAML
----

- requires agreement between parties
  - exchange of metadata and public keys
- IdP can choose what data to send
  - 3Ps receive *assertions* with *attributes*
  - data can be encrypted
- SSO friendly
  - also single-logout and administrative-logout
- enterprise oriented
  - based on XML and SOAP on HTTP
  - OASIS spec
- no direct comms between RP and IdP during auth flow


OpenID Connect
--------------

- Supports user-driven consent
  - users may be allowed to tell IdP to trust arbitrary 3Ps (IdP
    does not need to trust RP)
  - users can decide whether to allow or deny authn reqs and what
    data to send
- Completely different from OpenID 1.0/2.0
- Consumer-oriented
  - based on REST, JSON, OAuth 2.0
- RP must have access to IdP to verify assertions


Persona
-------

- Privacy-oriented
  - IdP doesn't get to know each every user's move
- Required browser plugin or complex JS
- Based on email addr for identity
  - requires public website to host IdP public certificate
  - uses crypto to generate custom user certificates
- Uses custom public/private key proto
  - Protocol is called BrowserID


Ipsilon
=======

- pluggable IdP
- supports multiple authn protocols
- supports multiple federation protocols
- provites tools for easy install, config and mgmt
- *not* an IdM server
- built in python
  - mod_wsgi or standalone via cherrypy
  - plugins are "drop-in"
- clients available for apache
  - native C modules
    - mod_auth_mellon for SAML

Merged with FedOAuth
--------------------

FedOAuth:

- current fedora authn system
- implemented OpenID
- also Python
- merged with Ipsilon December 2014
- rolling merged Ipsilon project in Fedora Infra right now
- you will soon start using Ipsilon without knowing it in Fedora


Ipsilon authn
-------------

Supports authn via:

- Any apache module
- direct LDAP binds
- Kerberos
- IPA / AD / etc
- TODO: chaining to other IdP

Supports fetching info via:

- LDAP
- nsswitch
- TODO: other IdP

Ipsilon protocol support
------------------------

- SAML
  - uses lasso/xmlsec1 libs
  - main focus when project was started
  - ECP profile in the making
- OpenID
  - ported from FedOAuth
  - uesd for Fedora Infrastructure
- Persona
  - ported from FedOAuth
- OpenID Connect (next)


Roadmap
=======

- integration with FreeIPA should be seamless
  - automatic configuration during setup
- REST API
  - for all admin operations
  - for SAML RP registration
- Protocols
  - improve SAML support
  - OpenID Connect
  - more auth/info plugins
  - kx509?
