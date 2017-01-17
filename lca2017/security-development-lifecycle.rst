Make more secure code - SDL and static analysis
===============================================

Jason Cohen (HPE)

Resources

- OWASP
- Microsoft Security Development Lifecycle (SDL)

- FTC starting to go against companies with insecure products
  (e.g. recent D-Link router lawsuit)

- Traditional assumptions about free software are not working
  - "enough eyeballs -> all bugs shallow"
  - everyone *can* review, but not enough qualified people *are*
  - old code is stable?
  - using frameworks is more secure?

- Who's examining security of open source software?
  - bad actors
  - security researchers (bounties, etc)
  - vendors
  - academics
  - gov (defensive and offensive)

- demand for exploits drawing big $$$
  - as exploits become more difficult to find, they pay better!

- systemic problem
  - devs being pushed to be experts in areas outside traditional
    training

- balancing act
  - ship it!  make $$
  - secure architecture takes time

- on positive note
  - better tools, more of them
  - free Coverity tool for open source projects
  - code is improving

- have to consider security throughout entire development/deployment
  process; not just at one stage

- Security Development Lifecycle
  - add security to each stage of development/deployment/incident
    response
  - can be adapted to agile dev processes
  - a lot of devs understand need for security but don't know how to
    implement properly
  - allocate time for study (a key part of SDL)
  - research past vulns
  - training budget and standards
  - get to know major vuln classes
    - recognise -> avoid
  - CWE (common weakness enumeration) top 25
  - learn from similar applications
  - find blogs etc that describe particular vulns in detail

  - defense in depth
    - explore new (and old) techniques
    - detailed security event logging

  - users will be users
    - if they're allowed to leave things in an insecure state, they
      will

  - requirements
    - threat modelling
      - the **foundational engineering effort** in terms of security
      - data flow models and user interaction models
    - security impact statements
    - attack surface analysis
      - infil and exfil
    - **rank threats** and define countermeasures

  - design
    - even a well-though-out design can fail
      - e.g. TCP global rate limit

  - impl
    - coding standards
    - regular peer reviews
    - regular static code analysis
      - report and explain findings
      - there are many tools, some free, some proprietary
