The Psychology of Authentication
================================

William Brown


(Some) doors have documentation - WHY?
The interface has a flaw.

When interfaces do not match human interaction principles, problems
occur.

The cause of Three Mile Island nuclear incident was confusion about
the control interface.

Princples
---------

- visibility
  - see what's possible (button, extrusion, noise, colour)
    
- affordance
  - physical attributes of control that give hint to how it works.
  - the wrong type of control *affords* the incorrect action

- consistency
  - taps - hot SHOULD BE on the left!

- mapping
  - relationship between controls and effects
  - e.g. stove burner control layout should match burner layout

- feedback
  - press button -> light changes
  - ped X crossing with no feedback = keep pressing!

- constraints
  - car gearbox select reverse
  - gas fitting threading


Human Interface Guidelines
--------------------------

- all the big tech companies have them
- they encompass the above principles and many others
- accessibility tools rely on adherance


Example: computer UI

- keyboard shortcuts listed next to actions
- invalid actions greyed out (constraint)


Counterexample: https://userinyerface.com/


Authentication UI
-----------------

- Passwords invented at MIT in 1966
- http://joelcalifa.com/blog/patronizing-passwords/

Phishing

- homograph / similar domain attacks
- consistency of browser address bar UI is a *disadvantage* here

Credential stuffing

- people reuse passwords (because they were made so hard to
  remember)

Passwords have poor UX but we keep using them

- easy to implement

- secure passwords are easy for *us* (*I* can use a password
  manager, so why not everyone else)

- physical tokens / smart cards have high cost

- implementing 2FA in legacy protocols (e.g. IMAP)?  Will all the
  myriad client programs implement it?  No.

OTP
---

- User, password, OTP (either on same page or separately on next page)

- Shoehorned into legacy protocols e.g. by append OTP to password.
  But this is:
  - inconsistent
  - lack affordance that OTP is required

- OTP still vuln to phish

- OTP out of sync / TOTP races

- Token for every site!
  - too many sites; hard to manage
  - some browser extensions can help
    - but token is now in the device; if device compromised you have
      big problems!


U2F / webauthn
---------------

is good


For now
-------

We still have passwords, so...

- Libraries
  - e.g. zxcvbn (constructive, actionable pw quality feedback)

- gradual move towards devices as authentication

- NIST 800-63B Digital Identity Guidelines; Authentication and
  Lifecycle management

  - SSO is good to rely on


Closing thoughts
-----------------

- humans are always part of our systems.  to make systems secure we
  have to think about human behaviour

- if people are always making mistakes, the problem could be the
  interface, not the people

- read "The Truth about Unix" by Don Norman
