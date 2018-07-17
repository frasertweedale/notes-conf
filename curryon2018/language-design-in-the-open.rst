Language Design in the Open
===========================

Yulia Startsev (Mozilla) @ioctaptceb

original JS requirements:

- scripting lang for web APIs
- developed in 10 days
- originally planned as a Scheme-like language
- "it should look like Java"
  - deceptive in intent and effect

- Netscape saw MS getting in the came and preemptively moved
  to standardise through ECMA

TC39 and its structure

- Ecma Intl technical ctte 39
- takes care of JS, JSON and their own process document
- a policy of 100% consensus; *everyone* must agree on a decision
  - 89 people; this is hard!

What happened to ES4?

- ES3 (1999) to ES5 (2009)
- ES4 was debated for 10 years abandoned in 2008 (*Oslo meeting*)
- proposals were added to the spec w/o impl
- ES3.1 (incremental update) came into conflict with ES4

Moving forward with "Harmony"

- addresses issue of having working impls for proposals,
  called "Harmony Process"
- structure of "champions", more discussion

Process

0. Allow input into spec; anyone can talk to TC39
   - should have clearly defined problem space
   - should have API / interface sketch
1. Champion takes it to ctte
   - make the case for addition
   - describe shape of soln
   - identify particular challenges
   - illustrative usage examples
2. Precisely describe syntax and semantics (spec prose)
   - TODOs and placeholder are still allowed / expected
   - ctte will decide here whether it should/not be included
3. Got a polyfill or impl somewhere
   - spec text is finished; all ES editors have signed off on it
   - only minor changes expected
   - not necessarily a linear path forward
4. Ready for inclusion in formal ECMAScript standard
   - Test262 acceptance tests have been written for mainline use cases
   - two compatible impls which pass acceptance tests
   - significant in-field experience with shipping impls

How is JS designed today?

- meet 6 times a year; 89 delegates (usual attendance ~60)
- everything done in public
- ask not "what problem you are trying to solve" but "what's at
  stake?"
  - stakeholders: users, developers, vendors
  - also: other standards bodies (W3C, WHATWG)
- if some feature that's great for developers breaks Facebook,
  users will switch vendors


*Extensible Web Manifesto*

- how to design web APIs and lang features so that we can build upon
  them and not repeat mistakes of the past
- focus on low-level APIs to support userland solutions

Backwards compat

- *Pure CSS Francine* by Diana Smith

Communication

- we often forget that there is a receiver in the communication
  - it should be a *conversation*
- check that the receiver has understood

What is important?

- the extensible web manifesto
- Complexity budget
- does it stand on its own weight?  Does it pay for itself?
- the principle of least surprise
- ...others?

Communication issues

- quality of discussion on esdiscuss has degraded (trolling etc)
- IP restrictions make it impossible for us to be completely open

Upcoming goals

- having even more meetings (maybe)
- better documentation
  - how to read the spec, how to create or give feedback to
    proposals
- reworking the website
- how to move away from esdiscuss and create space for newcomers to
  bring their ideas

Ctte issues

- info is siloed, we depend on who comes to mtg
- no reference impl
- we lack access to experts who can help us

Formal verification and impls

- KJS
- JaVert (JavaScript Verification Toolchain)
- impl of JS in JVM by André Bargull
- from JSCert to JWExplain and beyond
- *Formal Methods meets JavaScript* workshop
- *How can academics get involved in a useful way?*
  - How can TC39 help *you* get involved?
