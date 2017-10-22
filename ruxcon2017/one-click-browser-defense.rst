web browsers

- every system has one (or something like one)
- used by allaudiences from least to most technical
- becoming more and more powerful
  - 3d, web assembly, websocket, etc

- primary vehicle for most modern-day attack

weaknesses

- core tech and plugins
- limited means to control where users go
- vehicle for other attacks
  - offer up downloads; auto-exploits
- difficult to understand what's actually loaded when we browse
- ingrained in our everyday lives

attack success

- impact relative to subj of compromise
- personal: financial loss, randomware
- corporate: pivot, exfil / access assets
- civil society: reveal sensitive contacts, could result in
  detainment or worse

details to consider for civil society:

- business might say "eh, we'll eat the cost"
- compromises have real-world impact (arrests, physical attack, etc)
- total attack surface could be one individual, not an org
- general lack of funding, tech resources, time, subject expertise
  - education becomes a critical resource for defense

initial problem case:

- citizen lab
- high rate of phishing against Tibet activist groups
- stolen data reused to target and exploit close contacts
- awareness needed to be raised across multiple non-profits without
  central tech contacts

- requirements for success
  - cross-platform as far as possible
  - require little-to-no change in user behaviour
  - scale with little money or tech
  - allow for collab
  - block specific malicious resources
  - get data back to central location

- blockade.io: suite for browser defense
  - **browser extension** with automatic updates, capable of blocking
    threats
  - **cloud node** that can run on free tier; can run "serverless"
  - **analyst tool bench** that can publish and interact with cloud nodes

- browser extension:
  - ``webRequest.onBeforeRequest`` intercept all network requests
    (includes DNS prefetch and async requests)
- request resources are parsed, hashed, checked against db

- details of blocking event are recorded and sent back to cloud
- context menu for analysts to submit resources directly

isn't this just Google Safe Browsing?

- similar, but..
- blockade is free software
- not backed by a product company
  - you control indicators, users, mgmt etc
- targeted to only what's in the db
- can feed data back to operators
- requires nearly no change to user behaviour

state of proj:

- beta; used by a few targeted groups
  - citizen lab, security without borders
- looking for more analysts to contribute indicators
- looking for analysts or orgs to host their own cloud node
- looking for activists, journos and other volunteers for testing
- looking for developers to assist
  - ff extension
  - admin capabilities
