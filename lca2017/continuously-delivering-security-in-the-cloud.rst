Continuously Delivering Security in the Cloud
=============================================

Casey West @caseywest (Pivotal)

- principal technologist, Cloud Foundry

- Systems get attacked because they exist
- sometimes they get attacked on purpose

- resisting change to mitigate risk
  - a trap: the longer its up, the more chance attackers have
  - lots of places still believe system uptime matters

- successful attacks have:
  - time
  - leaked or misused credentials
  - misconfigured or unpatched software

- let's go faster
  - moving target is harder to hit
  - "cloud-native" operability lets us deliver security faster

- "cloud-native"
  - composable architecture
    - microservices
  - automated processes ; automate path to production
    - CD
  - collaborative culture
    - devops
  - production environment (that's actually not a tire fire)
    - structured platform

- the 3 'R's
  - rotate
    - rotate creds every few minutes / hours
    - challenge: reject all human-generated passwords!
  - repave
    - repave every server and application every few minutes or hours
    - a server that doesn't exist isn't being compromised
    - automate delivery to prod
    - repave ≠ patch
    - not necessarily about rapid change of software; just turning
      of and on again all the time
    - uptime <= 3600 ; don't worry about uptime!
  - repair
    - repair vulnerable runtime envs every few mins/hours
    - repair apps, runtimes, servers, OSes, etc
    - "cascading builds"
      - fix in OS?  rebuild, triggering rebuild of next layer up,
        and so on.
    - automation is essential
    - the reason hotfix processes are quick & agile is size of
      typical change

- container lifecycle:
  - build, deploy, run, stop
  - immutable artifacts; don't have luxury of changing in-place
  - reproducible builds

- the future of security is build pipelines
  - try to bake in credential rotation into build pipelines

- embrace change to mitigate risk
  - less of a trap! (in the cloud)
  - starve the attacker of the things they need

Questions
----------

- if you're going to repave every hour, do you care about
  intrusions?
  - yes we care.  monitor, mitigate.
