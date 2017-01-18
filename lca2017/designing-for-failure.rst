Designing for Failure
=====================

Dan Callahan (Mozilla)


- Persona was designed to solve authn on web

- designed to be fully decentralised
  - started centralised, but the scaffolds were supposed to fall
    away

Authentication matters

- trying to solve this problem was a worthy investment of Mozilla's
  resources

- social authentication started to take over
  - password field was going away
  - then, so was the email field
  - imposes a 3rd party
    - subjects users to that 3rd party's Terms of Use
      - real name policy, etc
    - privacy issues
      - authn provider can see everywhere you're logging in
  - no ability to choose how we want to authenticate in different ctxts

Persona failed

- we showed it was possible but didn't succeed in changing the web

Five rules of cave-diving accident analysis:

- never exceed training
- maintain continuous guideline to open water
- reserve 2/3 of gas for exit
- never exceed max operational depth of gas
- carry 3 lights

We can perform critical analysis on Persona failure.

If your project failed, share what you've learned so people don't
repeat the mistakes!

1. Free license is not enough
  - does not mean that users are meaningfully empowered to continue
    your project
  - persona inadvertently built in an intractable point of
    centralisation
    - browser was "eventually" supposed to build it in,
    - but because the JS iframe bootstrap was used in the beginning,
      noone was motivated to do the native work
  - if we hadn't been at mozilla, we wouldn't have predicated
    Persona on browsers implementing native support
    - we were blinded by our context
  - to fork the project, you'd have to fork the whole community

2. Bits rot more quickly online

3. Complexity limits agency
  - if it's complex to deploy / run / maintain, people are not
    empowered!

Free licenses don't further my freedom if I can't run the software

- i don't know how you fix that.  maybe you do.


Prolong your project's life
---------------------------

Friction:

- used a popup for the UI... but we quickly learned that people
  *reflexivly* close popups
  - we built a system that didn't mesh with user heuristics
- built an API that behaved differently when arg was false, null,
  undefined (yikes)

1. measure the right thing
  - is persona a product, or infrastructure?
  - branding.  does it have one?  who owns it?  how does it relate
    to other brands e.g. mozilla?
  - if it's product, the questions are: how many users, user growth
    metrics, etc
  - if it's infra, the right questions is: are we solving the
    problem?

2. explicitly define and communicate your scope
  - persona verifies email addresses
  - persona solves authentication
  - but... because Firefox OS, we expanded scope to run in a
    centralised way that doesn't verify email addresses.
    - it wasn't what it was for, or what it does best

3. ruthlessly oppose complexity
  - the explosion of use cases made software hard to test
  - patches are harder.  moving project forward is harder.
  - harder for other contributors to come in, contribute or take baton
    - there were only 1 or 2 significant outside contributes.
    - when Moz pulled plug, that was the end.
  - how hard is for you to say confidently to say the system behaves
    the way you believe it behaves.
  - simplify.  focus.
  - know what you are.  know what success means.

Plan for your project's failure
-------------------------------

- "Bus factor"
  - it's a commuter bus, that picks them up and takes them to
    another job.

1. if you know you're dead, say so.
  - regret taking so long to shut persona down
  - fallacy: "as long as we don't say anything, there's a chance it
    might come back"
  - sucked the oxygen out of the room.  we lost 3 years of
    innovation.
  - the sooner you admit it, the sooner people can perpare,
    transition, refocus

2. ensure users can recover without your involvement

3. use standard data formats

How can you minimise the harm when your project goes away?

In review
---------

1. authn is important open problem
2. free licenses don't guarantee functional freedom
3. define your scope and solve a specific problem
4. consciously consider your projects death; minimise harm
5. talk about failures
