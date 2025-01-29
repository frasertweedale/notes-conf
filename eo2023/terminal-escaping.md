# Houdini of the Terminal: the need for escpaing

- David Leadbeater
  @dgl@infosec.exchange

- History of teletype and terminals.

- ANSI standard, 1979

- DEC VT100, one of the first to implement the ANSI standard

- "CSI" = Control Sequence Introducer

- 2003 memo by H D Moore, "Terminal Emulator Security Issues"

- RCE.  "report title sequence" can be used to read the window title
  (which might already have been set to attacker-controlled value)

- CWE-150: Improper Neutralization of Escape, Meta, or Control Sequences
  - https://cwe.mitre.org/data/definitions/150.html

- CVE-2001-1556
  - Apache httpd log file contained info directly supplied by
    clients, did not filter/escape escape chars.  Cat file, get
    owned.

- Python HTTP server was vulnerable in a similar way, with default
  log behavoiur.

- CVE-2008-2383, xterm DECRQSS handling (another for iTerm)
  - DECRQSS = "DEC Request Selection or Setting"

- xterm CVE-2022-45063

- OpenSSH - if you exploit the server, can you exploit term vulns to
  exploit clients with open SSH session?
  - Various techniques are possible.

- Mitigations
  - Escaping
  - Ignore esc sequences in input
    - ZSH and ZLE (a la readline)
  - Implement DEC "compatible" parsing

- Similar classes of attack
  - Trojan Source
    - https://en.wikipedia.org/wiki/Trojan_Source

- Summary
  - history repeats itself because nobody listens
  - term vulns are rare, but can be dangerous
  - escpaing needed to protect us
    - ever time your print, think about where the data came from and
      what kind of escaping or sanitisation to perform

## Questions

- Are term bugs/vulns actually rare?
  - They're fairly small, it's easy to look for them and when we do,
    you either find problem or can be confident that there aren't
    some.
  - Other kinds of problems like buffer overflows are harder to look
    for
