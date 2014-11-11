The state of full disk encryption
=================================

Hugh, Catalyst IT

- @haquaman

Overview

- encryption
- types of full disk encryption
- problems with full disk encryption
- possible attacks on full disk encryption

Disk encryption:

- encryption applied to a disk
  - can be whole (full) disk, just a partition or just a file

Types of full disk encryption:

- BitLocker (Windows)
- TrueCrypt (Broken)
- FileVault (Mac)
- LUKS (Linux)
- Hardware encryption

Problems with full disk encryption:

- Unencrypted boot partition
- Key stored in memory
- "Looks" suspicious
- Slower? (moot point with modern processors)

Attacks on full disk encryption:

- Cold boot attack
- Evil Maid attack
- Vulnerable code
- Disk corruption

Common theme on attacks:

- Physical access wins all

Cold boot attack:

- As key is stored in memory while system is running, that key could
  be retrieved if attacker has access to the memory
- How to do it:
  - cut power
  - "Freeze" RAM
  - Load custom kernel to dump memory
  - Find key
  - $$$
- protection:
  - turn device off when unattended

Evil maid attack:

- Now that you turned off laptop, Mallory comes in to clean your
  room.
- Remember that unencrypted boot partition?
- How to do it?
  - load custom kernel
  - change encryptio bootstrap script to read key and send to
    mallory
  - dump encrypted disk (you'll get key later)
  - turn off laptop and leave; wait for victim to enter in key (will
    look normal to owner)
  - $$$
- Countermeasure:
  - Don't leave unattended
  - Separate boot partition on USB key
  - Hardware level (TPM et al) protection
    - not really any point; mallory could have tampered with it

Vulnerable code:

- update regularly
- roll keys every year to two
- don't use TrueCrypt
- Use open source, that has also been "verified"?
  - Can we really trust that there aren't backdoors?  Look at
    SSL/Bash/etc security.

Disk corruption:

- if device is unattended, attacker could easily corrupt disk


Follow-up
---------

Asked Hugh about IdM support for disk encryption:

  Quick question in follow-up to your OSDC talk.  Are you aware of
  any disk encryption schemes that support centralised key
  management of some kind?  e.g., one workflow (of perhaps many)
  might be: keys escrowed by an identity management system and
  supplied to a client upon successful login to decrypt homedir?

  Let me know if you are aware of anything along these or related
  lines.

In reply, Hugh mentioned that he did look into key escrow systems a
while back; would have to dig up notes, but the gist was an n-of-m
secret splitting method.
