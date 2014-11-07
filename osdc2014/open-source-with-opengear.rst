Open Source with Opengear - Ken Wilson
======================================

- software manager at Opengear
- based in Brisbane
- build embedded systems

Topics:

- open source hardware
- OSH business models
- why did didn't work
- pivot to current products
- what we can do better

Open source hardware:

- unrestricted datasheets
- Gerbers & Schematics provided
- no/minimal secret firmware blobs
- licenses:
  - CC-SA
  - TAPR Open Hardware License
  - 3-clause BSD
  - LGPL

Popular open source hardware projects:

- Raspberry Pi
  - nfp foundation
  - goal to advance computing education and understanding
  - outsource manufacture and sales to 3rd parties
  - change royalty per unit shipped
- BeagleBoard
  - similar business model to Raspberry Pi
  - founders are TI employees
- Arduino
- RepRap

Business models:

- Foundations (as above)
- manufacturer outreach
  - exist to encourage interest in particular SoC
- Branding
  - arduino designs and software are free
  - RepRap/MakerBot
    - FDM 3d printer
    - sold kits and components
    - MakerBot bought by Stratasys for $403mil
  - Opengear hoped to be another MakerBot

OKVM - Open Source Hardware in 2005:

- Motivations:
  - KVMs were expensive (1 user 32 port = $4300 RRP in 2014)
  - Proprietary access (java)
  - Vendor lock-in
  - Disrupt: commoditising video capture; "secret sauce"
- open source PCI card for commodity PCs
  - video capture/encoding IC
  - keyboard and mouse emulation
  - schematics/Gerbers/BOM provided
  - bundled source
  - embedded linux distribution - exposed KVM via VNC
  - opengear would sell at cost
  - 3rd party build solution

Failed :(

- hardware vendors wanted NDAs
  - still a problem today, but vendors starting to understand
- limited small volume manufacturers
  - no sparkfun or seeedstudio
- KVM EOL
  - management processors were embedded that functionality
- core issue: wrong types of customers
  - KVMs are for fixing problems
  - reliable, available support, and warranty replacement
  - cost not a compelling argument
  - opengear lucky to get a second chance

Pivot to console servers:

- proprietary hardware; mostly open software
- serial, not KVM

Use of open source at opengear:

- Firmware based on ucLinux-dist
  - considering moving to Yocto in the near future
- console servers - ARM9 and Cortex A8
- mgmt platform is x86-64
- console servers use u-boot as bootloader
- source code dumps are via email
  - only one request in 4 years
- CDK may be the reason

Proprietary software

- core components: config/cgi system, serial port mgmt
- components we want to replace or Open Source
  - cellular modem control ; difficult due to vendor NDA
  - network interface control ; other tools can't model the
    dependencies that we have
- projects for mainline linux don't take embedded into account
  - slowest shipping device has 32M RAM, 16M flash

Changing customers:

- initial customers: unix sysadmins
- scripting and customisability: good
- adding more features - open source projects
  - UPS support
  - serial and network PDU support via Powerman
  - nagios integration

Changing customers - enterprise:

- scripting not acceptable
- rewrite functionality - more complete
- affected earlier customers who had written scripts
- indifferent to open-source software
- feature set not as important as usability
- transition is going well but would have been easier if we'd done
  it right from the beginning.

Changing customers - management:

- large deployment
- single interface for access and configuration
- lighthouse appliance

Transition:

- started off based purely on Open Source
- moved to still using OS, but focusing more on vendor neutrality.

What we can do better:

- pushing changes upstream
  - most changes are to linux kernel
- keeping packages up to date
- move more of stack to being OS software
