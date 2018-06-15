access control

- half the programmers in the world don't understand mode bits
- in the 80s we already had discretionary access control
- mode bits are a simplification of the general concept of ACLs
- "me, us, them"; this is the way people think

- Bell & LaPadula Model
  - model of US DoD policy
  - levels and categories
  - level = Unclassified | Confidential | Secret | TopSecret
  - category = Footy | Cricket | Netball | Sevens
  - read: Ls > Lo && Co ⊆ Cs
  - write: Ls = Lo && Co = Cs

audit
- specification conflicts with ACL requirements

super user
- tied to user ID
- all privilege, all the time

Privilege and roles
- 3 roles: sys admin, sys op, sec admin
- what does "sys op" even mean? what do they do?

Capabilities
- should have been privileges
  - CAP_FOWNER, CAP_CHOWN, CAP_AUDIT_WRITE
- independent of UID
  - CAP_AUDIT_WRITE
- mapped to policy decisions
  - CAP_SYS_ADMIN, CAP_SYS_MODULE
- most not based on policy
- 320 capabilities; who can keep track of that many?

Capability capabilities:
- drop and raise privilege
- set on a program file
- too complex
- do same job as setuid root

Roles
- bad match to a workstation

Validation
- design testing
- source analysis
- object verification
- pen testing
- process: requirements -> design -> impl -> verify -> maint
  - in 1985, nobody did this

Adversary changed
- Morris worm (1988)
- The adversary was no longer the person beside you,
  but the programs computers were running

Linux
- AppArmor (interpretation of path name audit requirement)
- SELinux (type enforcement and transitions)
- Smack (3rd gen impl of Bell & LaPadula)
- TOMOYO
- granularity differences:
  - AppArmor = bricks, SELinux = sand
  - What would you rather build a wall with?

Enter the rabble.

- porn distrib has two interesting characteristics: payment & privacy
- dot com boom ; dot bomb bust
- rapid development
- Orange Book repudiation ("too slow")
- validation: fuzzing, code review, CI, static analysis
- code reuse
- wrap untrusted code in a fuzzy blanket
  - "here's what we think it should do, and we'll prevent it doing
    other things"
- systemd; set all attributes for the service
- containers: the ultimate fuzzy blanket
  - lie about the environment

Software controlling physical things
- we're at a crossroads
- about to start letting software drive cars
- AI for hacking
  - how much AI defensive software do we have?  not enough
- Asimov's laws

- it's time for computers to look out for themselves
- they need to be careful
- we won't always be in charge
