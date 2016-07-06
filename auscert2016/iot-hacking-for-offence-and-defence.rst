Hardware Hacking Chronicles - IoT Hacking For Offence and Defence
=================================================================

Fatih Ozavci, Context Information Security

- Author of Viproy and VoIP Wars
- Blackhat, Defcon, AusCERT, Troopers, etc.

Outline:

- Subscriber services and IoT
- Hardware hacking chronicles
- Hacking broadband, broadcasting and office equipment devices
  - Medical and consumer devices are another talk
- Improving defense and offence

Why care?

- Routers are getting attacked all the time
- Some (many) devices ship with built-in backdoors
  - Even from big / well known vendors

Traditional testing is not sufficient

- design reviews do not show business logic issues
- tech must be tested for various perspectives
- traditional tests do not cover
  - device firmware and hardware
  - management in a protected network
- testing methodology must be flexible
  - various devices (ARM vs MIPS; phone vs modem)
  - various OSes
- testing must always focus on devices' *roles*

Easy ways of compromising a device:

- configuration edit and re-upload
  - enable functionality not exposed in web interface
- secret handshake to enable telnet
  - many widely known exploits
  - vendors will often simply tell you how to do it
- admin password leak

Physical interfaces:

- UART, JTAG, SPI
  - bootloader, console access, real-time debugging, access to flash
  - only ever found one device that encrypts the comms
    (embedded certificate for authentication and encryption)
- sometimes they have pins, othertimes they have pin pads
  (get out the soldering iron)
- CFE - common firmware environment
  - re-flash for OpenWRT
  - get info out (credentials?)
  - dump firmware

Extract keys, credentials to attack service providers:

- VoIP, broadcast devices (e.g. set top box)
- Decomissioned 3G station hardware
  - Extract creds, access main 3G network

- Backdoors on devices are common
- Hardware implants are expensive (takes engineers and time)
- Human factor pen-testing (send backdoored devices)

Other attacks:

- Wireless keyboard attacks
  - Keysweeper, Mousejack

Defense and offense:

- force vendors to
  - disable physical interfaces and/or use encryption
  - follow a security standard
- network isolation for subscribers
- tailored research for
  - vendor product vulns
  - CPE management services
  - backdoor analysis
  - focus on all components
  - focus on exploitable issues
  - combines various disciplines
  - closes gap between offense and defense
- improve testing
  - devices are IN SCOPE
  - extract information from devices
  - fuzzing

Q&A

- What devices / vendors are most secure?
  - Some ship products with different (more secure) firmwares
  - There is no major difference between vendors or service providers
