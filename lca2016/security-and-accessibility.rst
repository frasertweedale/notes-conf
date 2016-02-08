Security and Accessibility
==========================

Nicolas Steenhout, Simply Accessible
@vavroom


- Lots of people talk about accessibility and security, but they are
  rarely discussed together.

*Nic first asks if there is anyone with a vision impairment who
needs the side content described.  There are.*

- The double arm amputee who went to the back
  - Bank had no-exception policy that anyone cashing a cheque who
    did not have an account must give a thumbprint
  - The solution the bank gave was "open an account with us"
  - To open an account a thumbprint is a required

- Easy to use != easy to break
- Falsehood: to make an app secure it will be difficult to use

- *Without reflection, we go blindly on our way, creating more
  unintended consequences and failing to achieve anything useful.*
  - Margaret J. Wheatley

- White House website petition
  - Petition to improve acessibility
  - Signing petition required solving CAPTCHA
  - Alternative audio CAPTCHA was totally incomprehensible
  - Try to contact White House: also required CAPTCHA

- Guide Dogs vs allergies
  - Is giving access to someone who uses guide dogs more important
    than keeping dogs away from people with severe dog allergy?
  - There *are* dilemmas of conflicting accessibility requirements
  - There is rarely one single solution that will work for everyone
    at all times

- Challenge for coders: start thinking of "complying with
  Accessibility guidelines / regulations" as a *fun challenge*,
  not a burden.

Legal requirements:

- Australia Commonwealth People with Disabilities Discrimination Act
  (1992)
- Of course, in 1992 we weren't talking about websites, so we are
  extrapolating technical requirements from the law

Commercial incentives:

- Worldwide, 20% of people have some disability
  - We want to be accessible to as many as people as possible
  - Because people spend money on the web

Right thing to do:

- Corporation have ethical responsibilities


Security and accessibility
--------------------------

CAPTCHAs are evil:

- Completely Automated Turing Test to Tell Computers and Humans

- W3C, 2005: CAPTCHAs are useless
  - people who break captchas are a step ahead of those that have to
    solve them

- Avoid relying on user input to make the determination of
  legitimacy

- User testing is really important

- Purpose of CAPTCHA is to reduce spam
  - maybe some say purpose is to be annoying
  - validate *content* of what is submitted first!
    - maybe it... looks like spam?!
  - honeypot
    - create a field
    - label it clearly
    - hide it with CSS
      - respected by browsers, accessibility tech
      - usually ignored by spambots
    - if stuff gets submitted there, it is almost certainly spam

Two-step verification:

- Don't submit content immediately
- Display it on second page
  - This is what you put in?  Submit to confirm
  - Defeats many spam bots
- Really important for
  - banking applications
  - anything where personally-identifying info is submitted
    - meets a common requirement here


Time limits:

- Spam bots have two indicators:
  - Fill out form as fast as they can
  - Harvest a whole bunch of form, get ready to fill it up, and
    submit in batches
    - One hour is a good maximum
    - But it does depend on how complex the form is
    - Think about people who use screen readers or for whom
      (written) English is not first language
  - These behaviours can be observed


CAPTCHAs with simple logic questions:

- e.g. "identify photos with street signs", etc
- it is somewhat better
- there are a limited number of questions to ask
- spammers have been building capabilities to solve
- example: "what colour is the sky?"
  - sunset?  wet days?
  - filling out the form at night?
  - it is not a simple answer!
  - some people are quite literal in the way they think
    - some people have disabilities that cause them to think this
      way?
- example: "which is larger: elephant or shrew?"
  - what about the "elephant shrew" (actual animal)
  - what is user has mouse phobia and you scare them away?
  - unintended consequences!
- simple math puzzles:
  - is the answer "2", or "2.0", or "two"
  - people with *dyscalculia*
    - https://en.wikipedia.org/wiki/Dyscalculia
    - up to 6% of U.S. population


"I am not a robot?" / "Are you human?"

- Torture survivors are dehumanised, may have been asked this
  question.


Google CAPTCHA:

- When it came out, accessibility people were skeptical
- But it's actually not that bad
- Works with screen readers
  - but audio CAPTCHA is ONLY accessible via a screen reader
  - if you are a sighted keyboard user who needs audio CAPTCHA, no dice
- Doesn't work well if you navigate with voice control, etc
- Can only access second level of verification with a mouse
- Overall less evil than other options, but not ideal


Session timeout

- Some users need more time
  - especially with complex forms e.g. banking, buying airline
    ticket etc
- Accessibility guidelines:
  - Allow 10x what you think
  - 20 hours is long enough
  - Options for short sessions:
    - extend session (actual button)
    - turn off timeout altogether


Re-authentication

- If user has gone away and form has timed out
- Or user was not able to use mobile version to fill out form
- Want to catch data and re-present the data
- Normally requires some sort of user login
  - Save, encrypt data, and present form as user left it


Data validation

- Often validation is often triggered upon click
- Sometimes users activate button using keyboard
  - don't forget to catch these events!
- Some people do not have scripting enabled
  - so have an useful way of presenting problems *after* form
    submission was validated by server, if something is not right


WCAG

- Love it and hate it
- Too many people say "I met the guidelines ∴ my solution is
  accessible"
- You can be accessible but not necessarily meet all WCAG technical
  guidelines
- It is a useful guideline


Resources

- incl.ca


Questions
---------

CAPTCHAs with useful purposes, e.g. translation, identifying signs /
street numbers?

- problem with anything that's imaged based, it is difficult if not
  impossible for screen readers and people who rely on them.
- sub-optimal no more accessible than other CAPTCHAs


Where do you see the intersection of good user experience and
designing for accessibility?

- UX and accessibility should work together; this concept is 


Accessibility comparison of 2FA tokens, passwords, federated sign
on, SSO, etc?

- It completely depends on the implementation you are using
- Passwords can be fine, or terrible if things are not labelled
  properly for screen reader
- 2FA can work brilliantly if the fields are labelled properly
- Physical tokens: haven't had any experience thus can't comment,
  but happy to follow up.


Important of coding to standards?

- It *is* important but not the be all and end all
- If you code to standards you will meet 80 - 85% of accessibliity
  needs
- There are libraries.  Bootstrap and jQuery do or make it possible
  to do a fair bit already.
