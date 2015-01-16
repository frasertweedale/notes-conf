Cooper Lees - Open Source at Facebook
=====================================

- 100s of GitHub repos
- 1000s of contributors from across world
- PRs: ~500 in 2012, ~5000 in 2014

Good citizens:

- we were bad :(
- now focusing on full life cycle

Top projects:

- react
- hhvm
- AsyncDisplayKit
- presto

Upstreaming

- chihaya - BitTorrent tracker
- libtorrent-rasterbar
- GNU coreutils e.g. grep
- Mercurial

More info:

- code.facebook.com
- github.com/facebook


Facebook networking, OCP and FBOSS
----------------------------------

- OCP: *Open Compute Project*
- FBOSS: open source switch software

What do we want from network devices?

- programmability
- openness
- switches == servers

Why Open Compute?

- save $
- more control over infra
- gain benefits from other people using it

OCP top-of-rack switch:

- x86 CPU, RAM disk and NIC
- switch ASIC with open API
- network ports
- runs CentOS
- forwarding software, ARP, DHCP, spanning tree protocol, NTP
- Chef cookbooks

Wedge:

- 16-32 40GE ports
- designed by facebook
- first building block for the Facebook disaggregated switching
  technology
- Facebook's first switch contribution to OCP

FBOSS:

- C++11 daemon
- entire software stack for a switch, including
  - mgmt/monit support (config & counters)
  - control functions (ASIC interface, DHCP, ARP, BGP)
  - platform support


Carol Smith - Google Summer of Code 2015
========================================

From *Open Source Programs Office* in Google.

- many students who participate are new to Open Source

How does it work?

- Early in year (Feb) mentoring organisations submit applications
- Google reviews applications and chooses orgs (March)
- Students submit proj proposals to orgs (March)
- Orgs choose students and pairs them with mentors (announced April)

- The student-mentor relationship is one of the key aspects; gives
  student a great chance for success and establishes relationships

- Work for four months; two milestones
- Two evaluations (midterm and final) for both mentors and students
- At end of term, students submit their project to the program
  website for everyone to see and use

- 2014 was 10th anniversary of the program

Changes for 2014 that will stay in 2015:

- 2015 stipend is $5500
- accepted more students than before (1307), hoping for a similar
  number in 2015
  - 8616 students over past 10 years
- 190 mentoring orgs in 2014

- Most represented countries are US, Germany and India, but trend
  toward developing nations

- 28 students form NZ and 83 from Australia since program started.

- ~50M LOC produced by GSoC students over 10 years (estimate)

- Accepting applications for mentoring orgs from 9..20 Feb.
- Annouce participating orgs on March 2
- Student applications from 16..27 March
- Students announced April 27th
- "community bonding period" from 27/4..25/5
  - not expected to start coding until 25/5
- Miderm evals from 26/6..3/7

Links:

- Melange google-melange.com
- list: bit.ly/gsocdiscuss


Mark McLoughlin - Red Hat
=========================

*Open Source is Shaping The Future of the Data Centre*

- @markmc_


Times are changing'

- IaaS
- PaaS
- microservices
- scale out
- CD
- agile and devops

- Our new reality... is still evolving like crazy.
- Behemoths are leading
  - and building it on open source
- if you can master these -> competitive advantage
- everyone wants a piece of this

Telcos:

- telcos are "special"
- telcos are squeezed and scrambling
- telcos need to be agile and responsive (like everyone else)
- telco datacentre
  - filled with big, expensive, proprietary boxes
  - years to build
  - years to phase out
  - difficult to scale
  - can't respond the change fast enough

Network functions virtualisation

- modern datacentre for modern applications
- still need "carrier grade"
- industry concensus around an open-source stack
- pieces: OpenStack, Open Daylight, Open vSwitch, Linux, KVM, Puppet
- Linux Foundation's OPNFV is building a reference architecture for
  telco datacentre using open source components

Why open source?

- collaboration and competition
- diversity (of contributors and interests) drives innovation
- sustainability
- demand for vendor choice (rejecting lock-in)

"Build a diverse community dedicated to writing and maintaining
software that solves problems users care about."

"Influence a future of IT that empowers and enables free societies."
