IPMI
====

Matthew Garrett <matthew.garrett@nebula.com>

Four letter words:

- ACPI
- UEFI
- IPMI

Data centres are awful

- noisy, nowhere to sit, noisy, cold noisy
- any technology to spend less time in data centre is probably good

Sysadmins are lazy:

- [citation not needed]

IPMI:

- turn things off and turn them on again
  - this is what people are mainly using it for
- monitoring sensors
- serial-over-LAN

Vendor extensions (outside spec):

- firmware updates
- virutal keyboard/monitor
- fake CD drive
  - upload ISO to IMPI controller
- discoverability

Vendor value add:

- Significant complexity
- Magic GPUs
- Web UI and web services
- Misc
- Other

BMC:

- Baseboard management controller
- Complete small embedded computer
  - 600-800MHz CPU, big RAM, at least a DVD's worth of flash
  - usually running Linux (HP isn't)
- Soldered to the mobo

IMPI isn't very good:

- authentication mechanism involves it giving you the password hash
- spec describes "cipher zero" that contains no encryption, no
  integrity protection and no authentication
- friendly advice: update your BMCs
- make sure cipher zero is *very, very disabled*

How is functionality implemented?

- Two main vendors: AMI, Avocent
- Build on variety of SOCs
- Lots of commonality
- Lots of vendor-specific code on top

Exposed interfaces:

- Web UI
- SSH
- Non-UI web services
  - Some vendors use PHP
  - Some vendors use an extension of PHP that allows you to embed C
    in your PHP
- IPMI extensions

Firmware configuration:

- GitHub:nebula/firmware_config

Let's talk about security:

- if the code you're writing links against libc, it's software
- software is miserable; firmware is even worse
- it is difficult to prove that your BMC is trustworthy
- when it core dumps it gzips the core dump... slowly
- SSL certificates leak ID
- SSL certificates leak server serial number
  - More than enough info for high probability of social engineering HP
    to tell you default passwords or send you new CPUs or something

Moral

- put BMCs on separate network
- firewall out all incoming IPMI anyway
- make sure they're up to date
- make sure they have cables plugged in
