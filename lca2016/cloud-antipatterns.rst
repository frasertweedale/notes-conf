Cloud Anti-patterns
===================

Casey West, Pivotal (Cloud Foundary)


Deinal: We don't neeed to automate continuous delivery

- automation is not about making things perfect, but *consistent*
- don't wait until things are perfect before automating
  - automate broken things; once they break *consistently* you can
    fix it once!

Anger: works on my machine

Anger: dev is just YOLO-ing shit to production

Bargaining: we crammed this monolith into a container and called it
a microservice

- you can't just cram your old busted shit in a container and call
  it good
- critical thinking is still required
- one of the biggest anti-patterns because it makes you believe
  you're getting somewhere

Bargaining: "Bi-modal IT"

- due to "Gartner" (researchers/analysts)
- "fast lane" and legacy "slow lane"
- a false dichotomy; there is a whole spectrum of speeds that
  different projects need to go at.  How quickly do you need to
  change the application?
- Casey's definition of "legacy software" is anything you can't
  iterate on quickly enough

Bargaining: What if we create "microservices" that all talk to the
same data source?

Depression: We made 200 microservices and forgot to set up Jenkins

- take one problem, and make it 200 problems
- a strong case for CD; green builds

Depression: we have an automated biuld pipeline but release twice a
year

- you did agile, it was magic, singing kumbayah, but the business is
  too slow
- anything product you have built up and not released is *risk*

Acceptance: all software sucks

Acceptance: CAP theorem

- consistency, availability, partition tolerance

Acceptance: respect Conway's Law

- architcture of software will reflect architecture of organisation

Acceptance: small batch size works for replatforming, too

Summary
-------

Operability is:

1. microservices architecture
2. devops culture
3. continuous delivery

PICK THREE

Some architectures resist the ability to deliver quickly - but we
should change that!
