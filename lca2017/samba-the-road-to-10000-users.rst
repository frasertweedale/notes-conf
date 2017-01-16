Samba and the road to 100,000 users
==================================

Andrew Bartlett (Catalyst)

- five engineers @ catalyst working on Samba

- samba now doing strict 6mthly releases
  - freeze 3mths before
- supported for 18mths (then throw to distros)
- 4.6 about to be released

Getting faster:

- two hour benchmark adding users and adding to four groups:
  - 4.4: 26k
  - 4.5: 48k
  - 4.6: 55k
  - tip: 85k

Scale is important do us:

- every user (with a workstation) implies a computer account also
- samba3 deployed widely using OpenLDAP for the hard work
  - ideally we need to match that scale
- we want to remove barriers, real and perceived, to Samba's use
  - not reasonable to ask that samba be deployed at edge of its
    capability

Rebuilding Samba for performance

- Performance issues - not bugs - are now biggest area of work
  - customers deploying at scale
  - customers using for 802.1x authn
  - customers growing and very keen to keep samba
  - very glad to be backbone of some multi-national corp networks

Replication as a performance bottleneck:

- so what if it takes time to add 10k users?
  - companies can't hire that fast anyway
  - even students don't enrol that fast

- biggest bottleneck is adding new DCs to Samba domains
  - e.g. opening a new office
  - sysadmins want to add new DC and go home at end of day

Improvements in 4.5

- 3 of 4 O(n²) loops removed
  - critical because these were under transaction lock
- turned on graph (rather than all to all) replication by default
  - avoid replication storms

Improvements in 4.6

- avoid over-replication of links
- faster parsing of links also improved performance ~20% for some
  tasks
  - avoid sscanf and malloc

Metrics!

- pretty graphs of add/modify/delete performance during benchmark

Finding a bottleneck: number of group members

- flame graphs
  - linux-perf
  - clone brendangregg/FlameGraph
  - only includes function names, no sensitive data
  - catalyst.co.nz/blog/burning-samba-perf-and-flamegraph

Future:

- remove other O(n) and O(n²) operations
  - multi-valued attr handling
- better index handling
- reaching the limits for current DB
  - memcpy() and memmove() from ldb_tdb transactions are 20% of time
  - LMDB looks interesting

Maintaining performance and scale:

- benchmarks need to be part of autobuild
- project to develop a new performance metric for Samba domains
  - currently awaiting client approval
- ongoing graphing of performance measurements
  - try to spot regressions before they get too old

Help wanted!

- need to calibrate metric tool
- need volunteers running AD willing to run a tshark script
  - windows or samba AD welcome
  - what does your "busy hour" look like?
  - what is the pattern of requests?
- email abartlet@samba.org if you can help

Beyond performance

- working on inter-forest trusts
  - initial support in 4.3 but more work needed
- inter-domain trusts
  - to allow migration from per-department Samba domains
  - still pending further development
  - most orgs moving to one domain, one forest

Beyond just pure AD

- what would make Samba compelling for your networks?
- can we integrate better with POSIX systems?
  - become the natural directory for Linux networks, too?
  - can Mac OS X be better supported?
- Samba 4.5 includes a Samba-specific password sync extension

MIT Kerberos

- blocking Samba being part of SLES and RHEL
- still in progress
- very important as Heimdal Upstream only just restarted releases
- I'm hoping to update the Heimdal copy as well
  - 5yo security code is not a great thing

OpenLDAP backend

- little progress other than presentations in 2015
- no public code
- I'm hesitant about another lift-and-shift like MIT K5
- prefer to fix one identified, isolated issue at a time
- incremental progress can pay off now
- we are focusing on own DS because it's what we can deliver to
  customers on time

Questions:

- Does Samba4 support custom schema?
  - Yes.  Off by default.  (but it doesn't help that there are no
    docs)
