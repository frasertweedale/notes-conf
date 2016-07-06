What Breaches Teach - education the hard way
============================================

Bill Duane, Distinguished Engineer, RSA

- An intrusion took over my life for 1mth
  - Full redmediation took >1yr


Many hackers are "professionals".

- Close their laptops at 5 and go home to family
- Take holidays
  - also take off US holidays (avoid detection)
- Have respectable skills
- Operate with military precision
  - Structured command and control
  - Extensive reconnaissance
    - May be resorting to recon/extortion of current/former employees
  - Multiple attack fronts
    - attacks may *seem* disjoint
    - teams achieving independent objectives, but information
      travels
  - Mix of weapon types
  - Adaptive tactics
  - Decoy maneuvers
  - Backdoors for later use / ongoing surveillance
    - plan your network assuming you're already compromised

Targeted assets:

- keys
  - code signing (esp. driver signing) keys
    - kernel mode can't make OCSP calls to check revocation
  - encryption keys protecting your information
- your IP
  - code / crypto / core libs / defect lists
    - malware / backdoor injection
    - clone your product
    - create malware specifically for your product
  - patents, product architectures, roadmaps, M&A data
- customer information
  - PII, PCI, customer lists
  - connections to customer/partner networks

Common tools and techniques

- account compromise
  - from admin acct, grant required privs to other acct,
    get out of admin account, use other acct
    - most monitoring is geared to admin accts
- memory cached password hash attacks
  - *pass the hash* attacks
  - cached credentials
  - extract them from memory
- SSO
- nesting
  - rather than hopping from system to system, reaching out and
    brining data back to "the nest"
- exfiltration
  - encrypted RAR segments
  - slow exfiltration
  - capture and save, and when possible analyse, exfiltrations
- your own infrastructure
  - SQL
- velocity
  - low and slow
  - if they think they've been detected, it's smash & grab

You vs Attacker

- time is their friend, your enemy
- everything you see in a breach is *in the past*
- they have focus, you have chaos
  - you don't know what they're after
- the attacker can hide in plain sight
- we make it easy for them
  - loose lips sink ships
  - we make things easy for partners, remotees, etc
  - principle of least privilege?
- you are all alone
  - bound by employee NDAs
  - bound by contracts
  - you can't say what's happening, ask for help, etc.

All is not lost!

- make time your ally
  - eagerly shut down access to other systems
  - graphical visualisations of whats going on
    - helps at 3am in the morning!
- know thyself
  - ask CIRC/IT staff if they know which business critical data
    resides on which IP addrs in your network
  - overlay business data on IP map
    - correlate attack to potential targets
- pretend to be your enemy
  - do proper threat model
  - red team
  - recurse!
  - threat model is a highly sensitive data!
- sacrifice some convenience
  - do away with local admin passwords
  - heterogeneity in system types and business processes
  - limit scope of SSOs and domain size/access
- upgrade crypto key defenses
  - reduce number of copies of keys
  - HSMs and smart cards
  - don't forget about your backups, archives etc
    - attackers can target them there, too!
  - get code/device signing keys out of automated build
- consider isolationism (data diodes, air gaps)
  - flash hardware from known good images
- make new friends for sharing information about attacks
  - it's awkward to do this with competitors
  - we have to make the sharing work well for us

Lessons from the RSA breach are pretty sobering:

- attacker spent lots of time, money, cyber weapons, and people to
  attack RSA
  - possibly as a backup strat against a very limited number of
    defense contractors
- attacker is focused, motivated, funded
- attacker doesn't care who the collateral damage is
- you probably are a target for an attack
  - are you a supplier to mil, gov, defense contractor?
  - are you a large commercially successful company?

Sorry, they're gonna get in!

- they're gonna get in
- they're in, so what now?

Time for a change in our defense

- evolve from "protecting permeters" to "rapid detection and
  response"
- treat corp network as dirty
- talk to each other about attacks

Personal thoughts:

- I never worked so hard, under so much stress, and with so much at
  risk.
- strongest driving factor was protecting people (co-workers)
- attackers came after me personally (notebook malware injection)
  - and I have a low internet profile!
- repair and remediation lasted >1yr.
  - jumped up from excellent commercial security levels, closer to
    defense contractor levels
- an education in the hardest possible way.
- it's OK to be attacked, but it's not OK to be a victim
  - Dave Martin, EMC Global Security
