Securing firmware: secure and trusted boot in OpenBMC
=====================================================

Joel Stanley, IBM OzLabs

OpenBMC

- BMC = baseband management controller
- ASPEED and Nuvoton make silicon
- in past, BIOS bendors provided firmware
- LF proj to collab on impl
  - Yocto layers
  - BMC specific userspace dev
- I work on upstream kern for ASPEED SoC

History

- IBM and Rackspace; 2015: P8 Barreleye prototype
- IBM and Google; 2017: P9 Zaius, Witherspoon
- Linux Foundation; 2018: Intel, Facebook, Google, Microsoft, IBM
- Current status: @ABOVE + ARM, Nuvoton, Dell, Lenovo, Quanta,
  Yandex, AMI, Inspur, Mellanox, Yadro, ...

Firmware like OpenBMC and uBMC are good first steps to trusting it - @qrs

linux kernel on BMC:

- BMC manufacturers
  - industry uses ASPEED or Nuvoton
  - No upstream Linux kernel or u-boot bootloader support
  - companies buy firmware from e.g. AMI
- OpenPower uses ASPEED
  - P8: AMI firmware
  - P9: OpenBMC firmware
  - future: more shiny
- My focus is kernel, blotloader and qemu
  - Linux kernel code is in dev

Linux kernel goals

- big reduction in out-of-kernel patches being carried (i.e. they
  were upstreamed)
- get the code upstream so I can retire

Qemu:

- AST2400, AST2500, AST2600 supported upstream
- used by OpenBMC for testing
- other projects use it tool
- Qemu model can boot images

Booting from NOR flash:

- NOR flash: can execute directly from the NOR
- u-boot executed from SPI flash
- u-boot trains (initialises) SDRAM
- relocates itself to SDRAM
- loads FIT (device tree containing boot payload)
- jumps to Linux

Booting from eMMC

- bootfrom loads u-boot SPL
- SPL executes from SRAM
- SPL trains SDRAM
- SPL lodas u-boot into SDRAM
- u-boot executes from SDRAM
- loads FIT
- jumps to Linux

Hardware root of trust

- prev ASPEED BMCs lacked hardware root of trust
- existing solutions:
  - Google Titan, Microsoft Cerberus, Nuvoton's secure boot
- ASPEED's new hardware has hardware root of trust
- keys can be programmed in OTP memory of ast2600
- bootrom loads 64K region from eMMC or SPI
- if secure boot enabled this is verified using OTP keys

Secure boot design

- put keys in BMC
- u-boot SPL is verified
- u-boot SPL verifies u-boot
- u-boot verifies FIT
- Linux uses e.g. dm-verity
- downside: only key owner can replace code

Trusted boot design:

- add TPM (to BMC, separate from the host TPM)
- put keys in BMC for SPL
- put keys in EEPROM for u-boot
- u-boot SPL is verified
- u-boot SPL loads but not verifies u-boot
  - allows end user to replace u-boot onwards
- u-boot verifies FIT keys in EEPROM
- downside
  - not secure boot
  - requires further step to verify system is secure

Trusted boot from eMMC

- bootrom loads u-boot SPL; verifies SPL against sig in OTP
- SPL executes from SRAM, trains SDRAM, loads, measures and verifies
  u-boot using sigs from EEPROM
- u-boot execs from SDRAM; loads, measures and verifies FIT using
  sigs from EEPROM, jumps to linux

This year's job:

- upstream u-boot
  - then we can build on existing work: FIT, kernel lockdown,
    dm-verity
- impl the scheme described
- finalise filesystem design so userspace can be verified
- future: remote attestation
- contribution encouraged: review design, write code, doc, test
