Troublesome Privacy Measures - Using TPM to Protect Users
=========================================================

Matt Garrett, CoreOS


TPMs

Trusted Platform Modules:

- trusted in sense of "trusted computer"
- very few people know what this actually means
- presence of TPM does not mean that your platform is "trustworthy"
- it is not a binary decision
- provides tools that *allow you* to make a decision about who or
  what to trust
- who is making the trust decision
  - Others?  *Treacherous Computing*


TPMs

- 28-pin chip; common form factor
- different manufacturers
- same programming interface if they're the same version
- v1.2 and v2 TPMs have completely different programming model
  - most current free software stacks targets v1.2
  - most shipping TPMs are v1.2
- ranges from microcontroller to ARM
  - some RAM
  - some NVRAM (100s bytes to couple kB)
- some GPIO
- can do some crypto
  - can sign and decrypt *small* data
  - slowly
    - depending on TPM, may take upwards of 20s

Platform Configuration Registers (PCRs)

- 16 to 24 PCRs
- *measurement*
  - measure the boot process
  - come up with a number that "represents" it
- TPM can't watch system booting
  - TPM needs system to tell it things
- PCR "extend" operations
  - PRC_new = SHA1(PCR_old || SHA1(data))
  - TPM 2.0 moves on to newer hashes/algorithms
- to obtain a specific PCR value, either:
  - break SHA 1, or
  - perform exactly the same sequence of writes
- each component of the boot process measures the next
  component of the boot process before executing it, e.g.
  - management engine measures first stage of firmware and extends
    it into PCR
  - first stage of firmware measures second stage and extends it
  - second stage measures drivers for hardware, etc
  - then bootloader measured
  - bootloader measures initramfs, kernel, etc
- with bootloader supports, PCRs hold full boot state
- if anybody tampers with any stage of boot process, measurement
  will differ from expected value

Now what?

- TPM itself cannot halt system or refuse to allow something to run
- you need to ask the TPM what the values are
  - via the kernel
  - "please read PCR value"
- problem: if kernel has been tampered with, how do we trust that
  the kernel is giving us actual values from TPM and not lies
  - seems like an insurmountable problem

Remote attestation:

- third party attests 
- first, make sure you're talking to a TPM
- at manufacture, all TPMs are flashed with a unique *Endorsement Key*
  - private key never leaves TPM (in theory)
  - certificate chaining back to manufacturer
    - not X.509, but not entirely unlike X.509
    - manufacturers are not all on the ball when it comes to hosting
      their root certificates securely
- ask TPM to create an attestation identity key (AIK)
  - returns public key and encrypted private key
  - only TPM can decrypt the private key
  - AIK is signed by the Endorsement Key

TPM quote:

- Copy of PCR values signed by AIK
- Therefore noone can make up PCR values
- Nonce is used to prevent replay attacks
- Kernel is never in a position to give you untrusted data
  - all it can do is refuse to give you any data, which is
    interpreted as "validation failed"
- Ask the remote attestation server if the system is good
  - by... sending packets via the kernel
  - maybe it could send you an SMS?
    - not hard to subvert or obstruct a process like this

Local TPM use:

- "Seal" data to TPM state
  - If the PCRs don't fit, the data won't decrypt
  - Commonly used to add a disk encryption secret
    - Windows supports this OOTB
    - if the system boots, boot process wasn't modified
    - if system doesn't boot, then boot process *might* have been
      modified
      - or you disk might have failed
    - if someone steals your laptop, they can still decrypt the disk
    - so add a passphrase
    - but at this stage, the boot process isn't trustworth!

Anti Evil Maid:

- rather than using TPM to encrypt disk encryption secret, encrypt
  some other kind of secret (phrase, words)
- display the decrypted secret on boot
- problem: if you display it on every boot, someone who wants to
  interfere with your boot process just needs to boot laptop, read
  secret off screen, and embed that secret in their malware
  - i.e. too easy to obtain a copy of the secret
- solution: put it on a USB stick instead
  - requires good opsec

An alternative:

- *non-static* secret that gets displayed at boot time
- seel a TOTP seed to PCR valuesk
- enrol the seed on a second device
- ``sealtotp`` application
- on boot, print the TOTP value
- cross-reference with your OTP app
- don't leave your laptop and phone in the same place!
- problems:
  - if you change bootloader, kernel, initframfs, firmware, etc
    secret will not be decrypted
  - decrypted secret is in RAM
    - DMA attacks
    - need to turn IOMMU on
      - most distros do not turn it on, because it breaks Intel
        graphics
  - Intel Management Engine
    - does the initial measurement of the firmware
    - if ME can execute arbitrary code, it can lie about firmware
      management
    - runs encrypted code and is completely unauditable; we have no
      idea how secure it really is.

What about after boot?

- *Integrity Measurement Architecture* (IMA) (Kernel feature)
- Measure each executable
- Verify via remote attestation that executables have not been
  modified
- PCRs depend on writing values to TPM in the *correct order*
  - you can't guarantee executables will be executed in same order

TPM event log

- record of every measurement event
- whenever you measure a thing, add an entry to log
- record value that was extended into the log
- hand over log for remote attestation
  - remote attestation server can replay the log
  - verify that each individual value in log is correct
  - verify that final result match signed value from TPM
  - log tampered with -> no match

Measuring containers

- when you bring up container you can measure container disk image
  and config into PCR
- logging these in same way allows you to verify which systems which
  launched which containers
  - allows you to identify compromised container hosts
- if verification fails, you can only prove something was tampered
  with, not what was actually run.
- implemented in Rkt v1.0

Code:

- linux-ima.sourceforge.net
- github.com/coreos/rkt
- github.com/mjg/shim
- github.com/mjg/grub
- github.com/mjg/tpmtotp
  - works with dracut / fedora-flavoured distros


Q&A

- do you know of any case where this has actually caught malware in
  the wild?
  - no, but... several attacks we know about (e.g. hacking team)
    *would* have been caught
  - this is a meaningful mitigation technology

- how do you ensure the CPU etc is itself trustworthy
  - very hard problem
  - we need fully open hardware
  - we need ways to confirm that physical hardware actually
    implements an open specification

- what if someone drops to shell in grub?
  - the above grub implementation extends every command executed in
    GRUB into the PCR
  - we are trying hard not to collide with any established PCR usage

- what features would you like to see in new hardware to make this
  easier?
  - currently noone has a method to verify "rest of platform" e.g. SSD
    firmware

- TPMs in firmware
  - there is a full TPM impl running on "management engine" (IPMI?)

- fuzzing TPMs
  - I don't know of anyone talking about fuzzing TPMs
  - maybe there are, but aren't talking about it?
