Assorted Security Topics in Open Cloud
======================================

Jason Cohen, HP

Advanced Persistent Threats:

- typically target individuals to gain entry point, e.g. spear phishing
  - USB sticks: nice conference freebie or lurking APT?
- covert exfiltrations
- nation state actors
- lifecycle: recon, initial intrusion, backdoor establishment, admin
  cred acquisition, util installation, data exfiltration,
  persistence maint.
- many application security designs don't take APTs into account
- defenses
  - active patch mgmt
  - port restrictions, network segmentation
  - separation of roles
  - security even information monitoring (SIEM)
  - user security training
  - encryption and key mgmt w/ hardware key protections where possible
  - physical security and encrypted drives
  - practical runtime defenses e.g. SELinux
  - measured/verified launch of hypervisor to detect rootkits

Trusted Compute Pools

- how does concept of "trust" translate to computing (cloud in particular)
- Intel TXT (combined with tboot) and OpenAttestation Server (OAT)
  verify trust status
- TPM required on server (standard issue these days)
- SHA1 hash values built in registered called PCRs that record state
  of software
- TPM 1.2 spec -> 24 160-bit PCRs.
- "Endorsement Key" for attestation
  - "Endorsement Certificate" signed by vendor
  - QUESTION: X.509?
- "Storage root key" for encryption any other key you want TPM to
  protect

Software components

- TPM driver (libtrousers)
- *TPM Tools*
- *tboot*
- jTSS and jTPM tools for Java
- Other components of interest may be IMA/EVM configured to use TPM
  or a strict SRTM configuration using TrustedGrub.

What TPM is not:

- HSM
- accesserator
- tamper-proof (but has proven to be very tamper resistant for what
  it is)

TPM 2.0:

- New spec is out
- new enc algos
- SHA1 -> SHA256
- ECC
- Three domains: security, privacy, platform

Intel Trusted Execution Technology (TXT)

- creates a "clean slate" shielded execution env
- utilised by tboot during system launch.
- DRTM: Dynamic Root of Trust for Measurement
- LCP: launch control policy

Attestation via Privacy CA:

- Machine to ve verified creates a signed list (with AIK) of PCR
  values paired with nonce and sends to remote verifier on demand
- a grand vision that never panned out...
- each TPM generates RSA keypair called Endorsement Key; privacy
  CA is assumed to know the public key of all (valid) TPMs.
- In case of OpenAttestation, install scripts generate an
  Endorcement Certificate (stores in the TPM NVRAM or a file) with
  the TPM EK and registers this on the OAT server
- when TPM needs to authenticate itself, it generates second
  keypair, called *Attestation Identity Key* (AIK)
- **gah, way too much info on slides**

OpenAttestation Server (OAT) install:

- pretty much the key component of Trusted Compute Pools; an
  implementation of Privacy CA
- be sure to reference OAT by FQDN; have DNS configure propertly
- takeaway: deploying OAT is a pain

OpenStack integration:

- can configure whether a specified image must run on a trusted node

Xen:

- can be configured for Trusted Boot
- also supported by OpenAttestation

Other applications

- TPM as source of randomness
- PKCS #11 module for TPM protected keys
  - Matt Garrett does things with this
- Key Manager integration
- Hadoop encryption (as of 2.3)
- Defending your Death Star against TPM hacks?

TPM attacks

- tamper resistance: some vendors better than others
- reset TPM -> DoS
- compromise OpenAttestation to also return "trusted" or DoS
- system firmware is *assumed to be trusted*
  - mitigation: immutable boot blocks, etc
- HP SureStart and Intel Boot Guard

Further topics:

- *Direct Anonymous Attestation*
