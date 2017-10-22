two problems assoc to app sec

- develop secure code (this is difficult)
- implement defensive layer

developing secure code
----------------------

- default behaviour of apps that deal with insecure reqs is to just
  drop them

- when apps do not detect attacks, they start
  - leaking information
  - doing bad things

- DARPA / CERT secure coding example ; full of fail

- Don't just drop / die ; LOG THE ERROR

  - You MUST decide what to do when someone is attacking you

- Define application minimum security level

- do you want to know if someone is triggering expected/unexpected
  exceptions?

implementing a defense layer
----------------------------

- previous work (e.g. OWASP Java AppSensor) has been
  language-specific and doesn't consider all attack vectors

- problems with detached defense
  - post facto, with incomplete info (logs don't show what was
    happening inside the app)
  - may introduce new vulns

- vulns are valuable
  - they point out where you need defense

- proposed soln
  - comprehensive open source spec of defensive measures

- three places to integrate defense
  - pre-exec; before functionality is executed
  - during execution: while functionality is executed
  - post-exec

- pre-exec controls:

  - tool disclosure:
    - UA
    - Host contains IP instead of hostname
    - forced browsing?
    - retrieve backup files
    - retrieve robots.txt
      - On an internal network???  Not normal
    - URL disclosure
    - HTTP verb
    - ignored hidden form fields being twiddled?
      - red herring that reveals attacker

  - what can you do?
    - diversion/trap

- execution controls

  - check if client is trying passwords in a loop
  - check if trying to bypass anti-CSRF
  - check if trying to perform MITM
  - check if they are trying to exploit path traversal
  - check if they are triggering uncaught exceptions
  - check if triggering expected exceptions
  - and so on... (this list is not exhaustive)

- post-execution controls
  - sensitive information: what info attackers may want to exfil

  - check if a special username being exfil'd

take action
-----------

- log and share your knowledge
- react to the attacker; do not ignore it
  - distract, slow down, stop (disconnect the application)
  - bait (give something to attacker that may help prosecute them)
- legal actions
- counterattack? (illegal?)
- defense cannot be static
- implement measures discussed in the language(s) you use

- https://github.com/IOActive/Embedded-Defense

conclusions
-----------

- don't ignore attacks; do something
