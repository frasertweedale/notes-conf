Securing the Linux Boot Process
===============================

- securing boot is important
- if you don't have secure boot, you don't have secure anything
- boot process can tamper with everything else
- it can lie to you about whether it tampered with anything

- mbr attacks
- bootloader attacks
- malicious initrd attacks

How do we fix these? (focusing on PCs)

- the only thing we have is UEFI Secure Boot
- software is hard

- initrds contain local configuration, so we can't just sign them.
  - e.g. which graphic to display.  users have opinions

TPM

- slow, relatively inexpensive, not particulary good at anything
- not under the control of the system processor
- Platform Configuration Registers (PCRs) contains hashes
  - "measure" (hash) boot components into the TPM PCRs
- bootloader measures initrd and only if measurements match will
  TPM release the disk encryption key.

- BUT firmware updates and all other kinds of legit changes will
  completely changes the hashes and make your system unbootable

MS to the recsue

- MS realised that this fragility was a problem
- don't measure files; measure the signing keys
  - result: "it will only boot something you signed"
- but... initrd is not signed
- embed initrd and kernel into a single image
  - single image that can be signed; a valid EFI object
  - it can also contain the kernel command line
- but initrds contain local configuration
  - and we cannot expect end users to sign their stuff all the time
- kernel can be handed multiple initramfs images
  - each is unpacked in turn
- unpack config first, code second

- kernel command line is also security sensitive
  - it lets you turn security features off

- proof of device state: tpmtotp
- other devices (e.g. tomu.im) ; tpm can have a conversation with
  the tomu to blink or receive GO/NO-GO.
- remote attestation
- secure provisioning of secrets

- /sys/kernel/security/tpm0
  - contains logs of everything the TPM measured during boot

Question:

- what are cloud providers doing in this space?
  - Azure has Secure Boot and TPM support
  - can't comment on other cloud providers

- Android verified boot process is quite different
  - on some devices you can change the trusted signing key

- A: nobody has yet deployed TPMs in a user-hostile way
