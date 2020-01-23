TPM based attestation - how can we use it for good?
===================================================

Matthew Garrett <mjg59@google.com>

TPM = Trusted Platform Module

- small chip, slow, resource constrained
- TPM 1.2: RSA, SHA-1
- TPM 2.0: ECC, SHA-256
- Platform Configuration Registers (PCRs)


- *not* cryptographic offload (slower than CPU by a lot)

- can provide integrity/confidentiality for some data *even when the
  OS has been compromised*
- holds secrets (i.e. keys) that can't be directly accessed by the
  OS
  - e.g. keypairs in TPM can be used for signing etc; cannot be
    exported
- holds data that can't be tampered with by the OS
  - e.g. track measurements abou tthe device that cannot be tampered
    with

What is the identity of a TPM?

- endorsement key (EK)
  - RSA or EC

- EK cert
  - X.509 cert issued by TPM vendor (e.g. Infineon)

- platform cert
  - optionally by the platform vendor (e.g. Dell) after final
    assembly of device

- endorse TPM yourself, e.g. when you first get the computer

- unfortunatley, EK can't be directly used for signing
- so a convoluted process is required to prove the identity of a
  device
  - second key needed; attestation key (AK)
  - complicated challenge/resp protocol required to prove the
    presence of the EK

Why is there an AK?

- TPMs were inteded to support privacy-preserving use cases
  - an AK could be created for each ent that is being attested to
  - such a scheme can be used to preserve privacy using 3rd party
    (i.e. privacy CA)...
  - but nobody has actually implemented that in practice
- oh well.

PCRs

- registers contain a hash
- can't be modified directly; firmware/OS can only ask the TPM to
  *extend* the register
  - if the measurements are altered or reordered, you end up with a
    different value

- PCRs do not reveal intermediate values

  - "measurement log" (which is not trustworthy) contains
    intermediate values and can be verified that they produce the
    final (trustworthy) TPM value.

TPM local use

- disk encryption key in TPM
  - if system not in expected state, TPM will refuse to release
    decryption key

- TPMTOTP

- these all rely on fixed PCR hash values (fragile)
  - trying to use PCR 7 reduces fragility, but also reduces
    guarantees

Remote attestation

- privacy preserving
- offline
- achievable with hardware you probably already have
- entirely free software

Demo!

- BLE server running on laptop
- 2nd laptop connected, full remote attestation handshake performed
- AK verified to match TPM EK
- Quote was verified to match TPM PCR values
- event log verified to match PCR values
- event log was analysed to determine wiether UEFI Secure Boot was
  enabled

- Why BLE?
  - BLE is easy to implement on phones
  - so instead of 2nd laptop, use your phone (which can probably
    connect to internet too!)

- Not ready for prod
  - no authn
  - hardcoded to TPM 1.2
  - look frankly if the demo worked at all it's a miracle!

- github.com/google/go-attestation


Questions
---------

- Put it in the initramfs?
  - sure, it's a go binary, it's big but just copy it in

- Can we trust the TPM vendors?
  - hmm... good question?
  - go see bunnie's talk this afternoon
