Integrating Network Access and End Point Assessment with Trusted Network Connect (TNC)
======================================================================================

- Avesh Agarwal
- Red Hat, Inc.
- Previously worked on IPSec and FIPS, CC

Agenda

- Network Access Control (NAC)
- End Point Assessment
- Trusted Network Connect (TNC)
- Demo


Network Access Control
----------------------

- challenges client to provide credentials
- "who are you?"
- no other information is asked
- 802.1X, IPSec, TLS


End Point Assessment
--------------------

- what do you have?
- is it good enough to allow you access?
- Health Check, Posture Assessment, Integrity/Measurements
  Verification

Why?

- Incorrect software version?
- Is software operational?
- Incorrect configuration?
- blacklisted software?

What is missing?

- How to transmit end point information over network securely?
- How to tie it with network access control?
- TNC addresses these


Trusted Network Connect (TNC)
-----------------------------

- Delivery of information over network, securely
- Verification of the information
- Enforcement
- Remediation

Components:

- NAR: network access requestor
- PEP: policy enforcement point
- PDP: policy decision point
  - reference measurements
  - policy database
- *Allow* access to a protected network
- *Isolate* to a remediation network

Features:

- TCG/IETF specifications
  - TCG = Trusted Computing Group
- Open standards
- Interoperable
- Extensible
  - Vendors can create / extend messages
- Modular
- Plug-in architecture
- NAC-agnostic

Layered architecture:

- integrity collection layer
- integrity evaluatoin layer
- network access layer


TNC threat model / countermeasures
----------------------------------

Treat model:

- any entity that is part of TNC exchange could be compromised
- any communication that is part of a TNC exchange could be
  compromised

Countermeasures:

- relies on protection by existing network access protocols
- ...


Terminology
-----------

- TCG and IETF terminology differ
- TNC = Network End Point Assessment (NEA) RFC 5209
- IF-M protocol = PA-TNC RFC 5792
- IF-TNCCS = PB-TNC RFC 5793
- IF-T (EAP) = PT-EAP RFC 7171
- (no TGC spec) = PT-TLS RFC 6876


End Point Assessment
--------------------

- *components* and *attributes* key data points
- extensible


TPM Assisted Remote Attestation
-------------------------------

- Extending TCG's TNC architecture
  - not specified by IETF
- Platform Trust Services
- TSS


Current status
--------------

- TNC over IPsec/IKEv2: in Fedora now, not yet in RHEL
- Otherwise complete functionality
- Packages: strongswan (strongimcv in RHEL), tncfhh, tpm-tools,
  tpm-quote-tools, freeradius, wpa_supplicant, libtnc


Questions
---------

- FreeIPA integration?
  - Storing policy or expected state of host in IdM?
  - Hostgroup or even usergroup info used in policy decicions?
