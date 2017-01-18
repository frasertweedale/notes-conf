Drink from the Firehose - release-monitoring.org
================================================

- OWASP top 10 2013 A9: Using components with known vulns

Three approaches to security

- hardened bunkers
  - backport security patches
- moving targets
  - update dependencies eagerly
- sitting ducks

- Most languages have a specific place packages are published
  e.g. PyPI, RubyGems, etc

- libraries.io
  - developer-focused upstream monitoring
    - help app devs to be a moving target
    - help app devs find libs of potential interest
  - already monitoring over 2M projects
    - just be monitoring ~33 publishing platforms

- there are two orders of magnitude difference between what a distro
  can provide, and what's out there

- there will always be "hardened bunker" use cases

- a lot of software moving to "deploy on demand" models

- need to being systematically alerting restributors to new upstream
  releases

- fedora-infra/anitya

- data model
  - upstream projects
  - monitoring backends
    - lowest common denominator: URL + regex
  - upstream ecosystems
    - assumes that there cannot be same name twice in e.g. PyPI,
      Gems, etc.
  - downstream distributions
    - typically linux, but we're neutral about what a downstream
      actually is
  - upstream/downstream mapping

- fedmsg
  - ØMQ based
  - dns-based service discovery
  - reverse-dns based topic namespaces
  - message source authentication
    - GPG, X.509

- future enhancements
  - OpenID Connect support
  - add ecosystem details to public data set
    - and de-duplicate
  - libraries.io backend?
    - monitoring the upstream stuff is the heavy lifting...
    - someone needs to do it, but everyone doing their own doesn't
      make a lot of sense
  - downstream version mapping?

Questions

- metadata sources e.g. CVE assignments
  - e.g. annotate release version with security info
  - it's not supported in anitya itself.  maybe libraries.io?
