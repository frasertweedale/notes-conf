CoreOS
======


cgroups
-------

- count and limit resources
- cAdvisor (from Google); API for time series metrics
  - useful for making scheduling decisions
  - useful for feedback to developers about broken code


docker engine
-------------

- container runtime
- named images you can fetch over the internet
- we previous had these things, but docker provided a transport for
  them


Rocket
------

- a standard and an application runtime engine built at CoreOS
- still at prototype stage
- striving for multiple implementations of the *container spec*
  - Rocket is one such implementation

Other container runtimes
------------------------

- google lmctfy
- cloud foundry garden
- mesos
- lxc


Container creation
------------------

- Commit -> Push -> CI -> Registry -> Pull to container host


Container superpowers
---------------------

- isolate application from underlying OS
- talk about app and run it in identical way across multiple hosts
- obviates configuration mgmt a la chef, puppet


OS
--

- With OS decoupled from application, we can update OS underneath
  applications
- CoreOS updates are atomic with rollback
- Opportunity for automatic updates
- Consistent set of software across hosts


Clustering
----------

- etcd: ``/etc`` distributed
  - implement as KV store accessed over HTTP
  - sequentially consistent across hosts
  - share config data across hosts
  - high availability; resilient to host failures


Scheduling
----------

- You -> Scheduler API -> Scheduler -> Machines
- fleet
- Other job schedulers: mesos (Apache / used at Twitter), kubernetes
  (Google), swarm (Docker)


Coordination
------------

- Locksmith


Service discovery
-----------------

- skydns
- discoverd
- confd
