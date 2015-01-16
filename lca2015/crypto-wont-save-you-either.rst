Crypto Won't Save You Either
============================

Peter Gutmann, University of Auckland


"If you see something, say something"

Snowden saw something, and said something about it.

BULLRUN
-------

- funded to tune of $250-300M/yr
- capabilities against TLS/SSL and more
- "aggressive effort to defeat network security and privacy"
- "Do not ask about or speculate on sources or methods underpinning
  BULLRUN successes"

"I know, bigger keys!"

Crypto won't save you.

Shamir: "Crypto is bypassed, not penetrated", or, "attack the user"

Example: games consoles
-----------------------

measures included

- signed exes
- encrypted storage
- full media encryption and signing
- memory encryption and integrity protection
- on-die key storage and/or sec co-processors

All of them have been hacked!

In none of the cases was it necessary to break the cryptography.

Kindle 2
--------

- all bins signed with a 1024-bit RSA key
- jailbreaker replaced it with their own
- later versions were similarly jailbroken

Motorola cellphones
-------------------

- hash chaining, MACs, digital sigs
- ignore crypto and target ARM TrustZone hardware-enforced security
  system
- "it's secure, because we say it is"
- exploit kernel and attack untrusted code from inside trusted
  kernel


Samsung galaxy
--------------

- firmware signed with 2048bit RSA
- modify firmware metadata to load it over top of sig-checking code

Nikon cameras
-------------

- sign images (RSA 1024)
- sig encoded in EXIF
- dump firmware and rewrite

Canon
-----

- HMAC
- shared key across *all the cameras*

Chromecast
----------

- carefully verified signed image on loading
- ignore verification result

Google TV
---------

- exploit inadvertently-enabled debug modes
- improper path validation to run unapproved bins
- remap NAND controller registers to allow kernel mem overwrite
- desolder encrypted SSD and replace with unencrypted one

Android code signing
--------------------

- APK = JAR = Zip
- signed using specially named files included in Zip archive
- use custom archive tool to create Zip file with dup filenames
- verification done using Java hashmap
  - dupes are overwritten
- installation is done via C code

iPhone/iPad/iOS
----------------

lots of security measures; bypasses include:

- inject executable code as data pages
- exploit debugging facilities
- use ROP to synthesise exploits from existing signed code fragments

Windows UEFI
------------

- exploit priv esc vuln in RT kernel to bypass signing

CCC 2011 badge
--------------

- use corrected block TEA/XXTEA block cipher with 128-bit key
- various exploits that all bypassed need to deal with XXTEA
- eventually, loaded custom code to extract the key

probably some sign of end times when conf badge has rootkit!

Xbox
----

- data moving over high-speed internal buses was deemed to be secure
- LVDS signally looks a lot like HT signalling
- used LVDS transeiver to decode HT signalling
- BFPG aren't fast enough to process data
  - hand-optimise paths through FPGA switching fabric
  - clock data onto four phases of a quarter-speed clock
  - overclock the FPGA

Later attacks:

- force CPU to boot off external ROM
- expoit arch quirk in CPU
- exploit backwards-compatibility support in CPU for bugs dating
  back to 80286
- exploit fact that font files (TTF) were never verified

PS3
---

- variant of first xbox attack
- glitch data on the bus to create inconsistency between cache and
  memory

Metrics
-------

How unnecessary is it to attack crypto?

Geer's law: "any security technology whose effectiveness can't be
empirically determined is indistinguishable from black magic"

- in 2012 researchers noticed people were using toy keys for DKIM
  - 12k orgs, 4k using keys so weak an individual attacker could
    have broken the key
  - noone did because there were still easy ways around it

- insecure encryption modes e.g. ECB
  - bigger keys don't help you there

HSM
---

- for verification, supposedly only emits "yes" or "no"
- oracle attacks for bank PINs

Crypto summary
--------------

- number of attacks that broke the crypto: 0

- NSA slides
  - interdict to get IPSec keys from config files
  - many similar stories
  - US govt is getting good value out of NSA!

National Security Letters
-------------------------

- requirement to hand over what they want
- and it's a gag order
- FBI overused them while under-reporting their use
- Several service providers shut down in face of NSLs

Dual_EC_DRBG
------------

- It is broken
- Noone would use it
- Except for a pile of US companies
- Mandated by FIPS 140-2
- FIPS 140 doesn't allow you to fix things
- NSA paid RSA $10M to use it and make it a default

NIST ECC curves
---------------

- Where did the seed come from?
- Jerry Solinas at NSA
- NSA generates billions of seeds, from which they generate curves
  until they find one that's vulnerable to attack
  - Get it adopted as NIST standard
- Brainpool curves defined in response
  - Immediately adopted in a number of open source projects
  - TLS WG never moved so quickly on an issue before.

IPSec
-----

Schneier: "It can't have been made this bad by accident"

- design by committee
- sabotage manual may be hard to distinguish from SOP in many orgs.

Routers
-------

- Does Huawei represent an unambiguous NS threat?
- Surely not bigger than NSA!
- Besides, TAO interception

NSA-proof crypto
----------------

- Any well designed crypto is NSA-proof; we have it today
- Sometimes we don't need crypto at all... maybe don't use the cloud?
- "If you don't hold it, maybe the NSA does"
- Predicate access to data on physical access to the
  location/machine.
  - or, "Don't put your data where the NSA can get it"
  - Now only your local spooks can get it.

Conclusion
----------

- "I love crypto, it tells me which part of the system to look
  *near* for problems*

- Crypto can be very *strong* but very *unsafe*

Q&A
---

- "Don't be a target" can be a useful strategy.
