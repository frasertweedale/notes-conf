Securely backing up GPG keys to the cloud?
==========================================

Joey Hess.

key backup solutions:

- paperkey(1)
  - hard to restore
  - not going to help new people use GPG
- backup $HOME
  - not really secure
- obnam(1), attic(1)
  - encrypted using what key?
- shard and store on USB/whatever, scattered here and there
  - not automated
- DIY
  - you're going to screw up
- don't backup at all (in a way you can restore)
  - common approach!
- straw poll: 20% of users have keys backed up
- 2600 magazine published their key in mag
  - promptly lost it
  - and hadn't made a revocation cert

Keysafe

- GPG key backup to "cloud" servers
  - securely, easily
- make key recovery expensive (takes a long time, even when you know
  the required secrets)
  - 25mins to 1hr
  - this is where most of the security actually comes from

Building blocks

- argon2, SSS, AES, Cloud, Tor, zxcvbn (pwd strength est. lib)

argon2

- 2015 pwd hashing competition winner
- memory intensive (GPU/ASIC resistant)
- tunable difficulty (iterations; memory)

SSS

- boring 70s tech; beautiful piece of math
- n of m required to reconstruct

aes key generation:

- inputs: password, name, other name, 1 random byte
- salt = name <> other name <> 1 random byte
- argon2(pwd <> salt)
  - tuned to take 12s
- run argon2 up to 256 to find the right key
  - average 128 tries
- even a bad password takes 25 CPU-years

defenses

a. passwords
b. object IDs
c. keysafe servers

keysafe servers (in the cloud)

- simple KV stores
- store only fixed size objects (no large data)
  - 256KB
- no object enumeration
- self-tuning proof of work system
  - DoS protection
- only accessible over Tor
  - anonymity, transport security
- as long as 2 of 3 keysafe servers are uncompromised, no mass
  password cracking
- best hosted by well-known, broadly trusted organisations

Object ID gen:

- combined name <- name <> other name
- salt <- keyid
- base id <- argon2(salt, combined name); tuned for 10mins
- split ID
- two colluding servers can perform a correlation attack to find
  related object IDs
- servers don't record timestamps, or keep logs, to prevent
  correlation attacks after the fact

is keysafe safe enough?

- probably for "lower value" GPG keys
- if your threat model includes nation-state: ???

current status:

- Haskell, 3600LOC
- in debian (experimental)
- needs more design, impl, security review
- three servers
  - Purism
  - Faelix
  - Digital Ocean (run by Joey)
- more servers needed

options for the more paranoid:

- keysafe with non-default options (e.g. 6 shares)
- store some shares on keysafe, others elsewhere
- 64kb share can be stored locally in a variety of hard to detect
  ways
  - end of partition
  - stenography

thanks to Purism and patreon supporters for supporting the work


questions (lots!)

- why don't I just put up my password protected GPG key?
  - the hash is very weak (maybe 1 week to crack)
  - you can tune the hash, but not enough to be secure

- have you considered a high-latency network for
  storing/retrieving objects?
  - it's a possibility

- what's the risk of TOS violation or other issues at keysafe server
  on cloud provider?
  - I'd like warrant canary

- my questions (not asked):

  - what AES cipher mode is used?
    - AEAD?

  - servers on tor hidden services?
