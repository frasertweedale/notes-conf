************************************
The trade-off between UX vs Security
************************************
it is not easy to get the balance right
=======================================

Petdra Hayati, elttam


- Too much security turns users away
- Too much convenience can come with high price of data breach

- Began journey into literature about software design for usable
  security (there isn't much)

Topics:

1. Why is it important to consider UX?
   And what will happen if you don't do so.

2. Tradeoff between security and UX.

3. One approach we can use to address this problem.


What is UX?

- Simple defn: the quality of experience a person has when
  interacting with a specific design.

What do we need UX?

What is no.1 reason for data breaches in past five years?

- Employee negligence.

What are causes of employee negligence?

- Insufficient security awareness
  - unfamiliar iwth recent security attacks targeting employees
  - poor security hygiene
- **Insecure decisions caused by bad UX**
  - e.g. password policies; it is hard to choose a strong password
  - we haven't considered the human in our design!

UX vs security:

- Aspects of UX: look, feel, usability
- Aspects of security: confidentiality, integrity, availability (CIA)

Case study: "Security question"
-------------------------------

- Type of UX design for authentication purposes
- Extra verification step that checks authenticity of incoming
  request prior to authorisation of an action
- two types: generic, application specific
  - generic: secret question, email/SMS verification, 2FA
- Why do we need a security challenge?
  - Password is "single point of failure"
  - Difficult to manage
  - Password does not verify an identity; all it proves is that
    requestor knows the user's password
- UX: users don't want additional step; wasted productivity

What can we do?

- There is no single answer
- Security questions
  - **Secure** security questions
    - easy to remember
    - doesn't change over time
    - applicable to wide rsange of users and backgrounds
    - not easily guessed *or researched*
    - **do not allow users to create their own security question**
  - Time limit to provide answer (10min)
    - If exceeded, restart whole process
    - Rotate questions
  - If no correct answer given
    - temporarily lock down account, notify admin and user (email)

Pros and cons:

- Simple to implement
- No dependency on other platform
- Large number of users unable to recall answers
- Or choose fake or easy to guess answer
- Poor security questoins provide no additional security
- More websites use secq, user likely to reuse Q&As

Verdict?  Bin the security questions.


Email verification
------------------

- gen "random" token and email user (put token in URL)
- verify user uses same session as generated the token
  - otherwise, restart process
- limit number of verifications to 3..5 attempts per 30min.
  - if threshold exceeded, temporarily account lockout.

Five characteristics of token:

- unique
- one-time use
- time-limited
- random
- strong (min 128 bits)

Pros and cons:

- email can be intercepted
- regularly sending email verification msg to users, we create use
  expectation of receiving emails for authentication.  Increases
  threat of spear-phishing.
- User may have lost access to email or forgot which email she used.

IVR or SMS verification
-----------------------

- IVR = *interactive voice recognition*
- 5min window
  - restart process if exceeded

Pros and cons

- repurpose familiar technology
