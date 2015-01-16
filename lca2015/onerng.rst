OneRNG
======

http://onerng.info/

<MISSED THE START>

- keep topping up the pool

- 7.5k pool fo data that is kept full

- LED tells you when the pool is full

- 305kbit/s quoted, but unofficially, ~500kbit's atm.


``/dev/urandom``:

- Not inteded to be consumed directly

- if you read manual, you'll read from /dev/urandom all of the time
  on Linux, but not necessarily other OSes

There's more:

- can enable RF monitor to get another source of entropy data with
  higher quality - but at the cost of a little paranoia

Flow of data:

- avalanche diode circuit -> whitening -> data pool -> results
- untuned RF receiver also feeds whitening

Why random numbers?
-------------------

- cryptography has huge appetite for random data

- your only privacy and security depend on crypto

- the more you encrypt, the more entropy you need to consume

- altcoin also requires high quality random data
  - coins have been stolen because of weak RNGs

Non-crypto random numbers?
--------------------------

- Statistics

- Gambling

- Simulation (Monte Carlo, Markov chain, etc)

- Could use PRNG but need to be careful about periodicity / repeats

PRNG
----

- don't ever use a default ``random()`` function
- don't use PID, or microsecs since epoch, or wall time
- don't do your own crypto
- don't do your own RNG

True RNG vs PRNG
----------------

- even if your PRNG is a CSPRNG, you'll feel the need for seed

- get your seed values from a true RNG

- only generate your long-lived private keys when there is
  sufficient entropy


How much random can a ``/dev/random`` random?
---------------------------------------------

- never mind the quantity; feel the quality
  - unless you needed quantity, that is

- if your true RNG returned sufficient data, why use PRNGs at all?
  - (because you must mix multiple entropy sources together)


Just because you're paranoid
----------------------------

- Even if they aren't out to get you, they'll get you

Known badness
-------------

- RSA SecurID token breach
- Entire CA system
- Dual EC_DRBG, $10M NSA/RSA deal
- RdRand and the Sigint Enabling Project
- supply-chain interdiction
- BIOS chronomancy: will persist across reflashing
- Supercookies, evercookies
- NSA vs Vodafone (aka "we are the 43%")
- Everything "Snowden"

How are issues fixed?
---------------------

- when risks are "addressed" they are usually pushed up or down stack
- they end up *requiring* trust in end-user behaviour

How does OneRNG help?
---------------------

- Being open hardware and open source solution is a start
- OneRNG is also designed to be *verifiable*
- You *do not need* to trust the device
- You should verify that what you physically hold is what you need
  to have

How to verify the hardware?
---------------------------

1. plug the device in
2. dump firmware
3. confirm data is full 256k
4. validate the GPG sig
5. confirm the data is incompressible

What does verification tell you?
--------------------------------

- hardware received confirms with description
- firmware matches source
- tampering would be detectable

What will OneRNG provide on Linux?
----------------------------------

- more entropy
  - ``/dev/random`` less likely to block

- earlier entropy
  - we haven't done much work on this

Using OneRNG with Linux
-----------------------

- software to make OneRNG a source of entropy

Just plug it in
---------------

- UDEV detects the device (using ID assigned form OpenMoko's range)
- on insertion we validate firmware, then start ``rngd``
- on removal we remember to stop ``rngd`` for you
  - UDEV implementations make life more complex
  - noone seems to bother with UDEV removal scripts
  - so the mechanism is probably not very well tested

Serial over USB?
----------------

- USB CDC drivers are the generic USB serial interface
- ModemManager stomps on every one unless you remember to disable it
  in UDEV
- OneRNG doesn't support Hayes AT commands
- UDEV: ``ENV{ID_MM_DEVICE_IGNORE}="1"``
- Wireshark will capture USB traffic if you need it
  - ``modprobe usbmon`` first

Why does ``/dev/random`` block?
-------------------------------

- The NSA-designed SHA-1 was not fully "trusted"
- The blocking behaviour is a defense against this untrusted DRBG
- Use ``/dev/urandom`` (they behave identically with sufficient
  entropy)

How does OneRNG help ``/dev/random``?
-------------------------------------

- even though you should use ``/dev/urandom``
- OneRNG helps to avoid the need to block
- therefore systems that use ``/dev/random`` *run faster*?
  - Catalyst are simulating workloads to quantify this assertion

The more the merrier
--------------------

- a small quantity of good entropy is enough to improve everything
- the more sources of entropy you have, the better off you are

Complements to OneRNG
---------------------

- rtl-entropy
- USBtrng (Bdale Garbee/Keith Packard, DebConf 2014)
- Turbid from audio I/O
- NeuG (DebConf 2014)

Kickstarter
-----------

- who does product announcements at KiwiCon?
- funded in six days
- $10k in 45 days
- funding is still open
- expect to hit ~$32k
- current progress: >$27k, >250 backers, 13 days to go
- future
  - 1-up version that will be installed internally in the works

Manufacturing
-------------

- First run on team's pick & place machine
- After that, probably China

What's next?
------------

- One of the kickstarter rewards is a OneRNG programmer device
  - cannot be (re)programmed over USB
  - *we want you to hack on it*

Q&A
---

Why whiten?

- Remove DC bias from entropy so ``rngd`` doesn't freak out and the
  cost can still be kept down.

China manufacturing: fake chips?

- Verify what you get.
- Improve chances: sit down, spend some personal time with them.

OneHSM after OneRNG?

- Nope, other projects.

Bulk pricing?

- We hope for <$20 once in bulk manufacturing.
