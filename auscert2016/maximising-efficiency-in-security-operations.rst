Maximising Security in Security Operations
==========================================

Mark Carey-Smith, CITEC

- how does CITEC Security Operations team maximise efficiency?

- emphasis on open source resources where available, as well as
  in-house innovations

- CITEC is Qld gov's primary ICT services provider
  - Deliver whole-of-govt and agency-specific services
  - Agencies are free to choose whether to use CITEC or not
- SecOps supports technical infrastructure with hightened security
  profile in 24/7 environment.

Challenges facing secops:

- limited time, resources, budget
- mixed perceptions about infosec
  - "obstructionist" perception is common
- the key is efficiency

BIND Response Policy Zones (RPZ)

- allow for custom information to be used to override 'normal' DNS
  responses
- in practice, allows blacklists that stop "known bad" DNS responses
  to be returned to clients.
- whitelists can also be used
- a.k.a. DNS firewalls, DNS sinkholes
- data stored in standard DNS zone files
  - existing zone data distribution mechanisms still work
  - familiar to sysadmins
  - same logging mechanisms

Information sources (free)

- your own networks
- OpenPhish
- Virus Total
- Alien Vault Open Threat Exchange
- Threat Crowd
- the Shadowserver Foundation
- Bambanek Consulting
- Spamhaus (DNS blacklist)

Automate the consumption of information sources

- detect vs prevent
- validation/effectiveness monitoring
- RPZ, meet blacklist

Mine your own adventure - "known unknowns"

- firewall logs
- web proxy logs
- DNS client query logs
- email server logs
- can be overwhelming
  - start based on risk assessment?

Leveraging knowns to find unknowns

- spam and phishing campaigns often have common attributes
- leverage these "knowns", sender IP, to and from domains, coupled
  with blocked by SPF or DNSBL events, to find "unknowns"
- some malicious messages still get through
  - use info gained from blocked messages to find them
- summary data of blocked traffic is shared

Domain Generation Algorithm (DGA) malware

- some malware variants use pseudo-randomly generated domain names
  for C&C comms
- after initial infection, malware dropper attempts comms with list
  of C&C servers created via DGA
- Zero Day DGA
- anomaly-based detection
  - random is bad (beyond a point)
  - CITEC SecOps developed a means of detecting suspicious DNS
    queries that were "too random"
  - proved to be an important supplement to other controls based on
    blacklists, resulting in multiple detections including
    cryptolocker variants.

Make incidents and influence people

- manual vs automated processes
  - automate wherever possible and appropriate
- email vs incidents
  - incidents (i.e. put into a job ticketing system) are harder to ignore
  - assign to the relevant person or team
- manage laterally
  - do what you can, within limits
- manage expectations
  - the balance of good netizen vs over-servicing

Team culture

- silos vs symphonies
- teaching moments
- manage up (self-promotion for teams)


Benefits

- CITEC SecOps has resulted in more services being provided to Qld
  govt agencies
  - resulting communication demonstrates capability and value for money
- team design elements have resulted in less stressful, more
  informed, capable and supportive team
