Linux: the first second
=======================

Alison Chaiken
alison@she-devel.com

- NHTSA requirement to show rear video within 2s of starting
  car in reverse.

Slow boot: linux boot on 8-bit AVR
  - uARM - ARM emulator on AVR.
  - 2hrs to boot to sh prompt


How linux starts.  Overview:

- what does "off" mean?
- ACPI vs DTB
- kernel as PID 0
- how does PID 1 start?
- what *is* an initrd?

x64_64: never genuinely off

- WOL
- IPMI: run from Baseboard Management Controller
- AMT: run from Platform Controller Hub

What kind of program is the kernel?

- ELF binary
