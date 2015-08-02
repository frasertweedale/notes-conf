Making sensible security decisions by assuming the worst - Tom Eastman
======================================================================

Tom Eastman; Lead Consultant, SafeStack; tom@safestack.io
(fmrly Catalyst)


Playing to lose.

- What if you found out you were hacked...
  - six months ago?

- People are usually scared because they haven't though about this

What is the correct answer?

- "Well, it depends"

  - It shows that you've though about the problem
  - Indicates awareness that there are "levels" of how bad it could
    be

- Thinking in getting hacked is not an exercise in masochism;
  - it's essential for the security of your application

- Security is not any one thing
  - Your firewall can't save you from everything
  - There's more than one kind of attack, so you have to think about
    more than one kind of defense

Defense in depth

Who are the threats?

- who cares about me, why might they want to hurt me?
- motivations
- skill level
- examples:
  - opportunistic script kiddies
    - out to make a name for themselves; have all the time in the
      world
  - organised crime
    - out for financial gain
      - extortion, credit card numbers, ...
  - disgruntled fmr employees
    - this can happen even if you are a lovely place to work
  - hactivists
  - nation-state actors
    - near-unlimited resources
- These are "security personas"
- Formalise the process of thinking about these different actors

Attack surface

- the sum of different points (attack vectors) where unauthorised
  user (attacker) can try to get into or extract data from an
  environment

Layers of web application

- web server
  - most exposed part of infrastructure
  - bugs found in server will quickly be turned into automated exploits
  - minimise attack surface:
    - *immediate* application of updates
    - disable all unused modules and config
    - ues current best-practice TLS config
  - mitigating breach
    - keep web servers separate from app servers
    - AppArmor / SELinux
    - whitelist egress firewalling

- app server
  - usually written in high(er)-level language
    - ∴ buffer overflow et al *less* easily exploitable
  - mitigating code-execution attack
    - AppArmor
    - make you don't have to give it write access to any files?
    - whilelist egress firewalling
  - mitigating a credential-theft attack
    - IP address resstriction on admin accounts
    - 2FA makes cred theft much less useful

- database
  - SQL injection still #1 in OWASP top ten.
  - lateral movement from other (compromised) server
  - database *backups* could be compromised
    - are they in S3, sitting on dev workstations, etc
  - minimising attack surface:
    - reduce privileges of account used by app
    - never allow code even with *potential* of SQL injection into
      application server
    - to NOT use production data / databases on dev environment
    - always know exactly where your DB dump files are

- front-end interface
  - XSS attacks
  - attack people who visit your site, *including admins*
  - consequences
    - website defacement
    - attack users of site and their data
  - minimising attack surface
    - whitelist input validation on user-generated input
    - escape all data appropriately for display
    - Content Security Policy (CSP)
      - HTTP response header that lets browser know from where it is
        allowed to load / execute JS
        - including from *on the page itself*
        - restrict to loading JS from, e.g. CDN and nowhere else
      - only a mitigation, not a cure
        - only helps people using modern browsers
      - CSP violation reports
        - make customers' web browsers your ally
        - early warning system

- What if someone got your IaaS keys?
  - Commit to public repos, pastebins, stack exchange etc
    - These are constantly being scanned
