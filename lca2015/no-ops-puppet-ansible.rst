NoOps with Ansible and Puppet
=============================

Monty Taylor

- Distinguished Technology at HP
- OpenStack Infra core member
- Formerly MySQL consultant
- Drizzle core dev

NoOps
-----

- developers can code and let a service deploy, manage and scale
  their code
- if I'm "doing ops", I've probably done something else wrong
- I want to change the system by landing commits
- If I have to use root to do a task, it's a bug

Cloud Native

- ephemeral compute
- data services
- design applications to be resilient and scale out

Cloud Scale Out

- forget HA of one system
- forget long-lived systems
- shared-nothing for EVERYTHING

What about existing applications?

OpenStack Infra

Gated Commits

- every commit is fully integration tested (twice) before landing
- 1.7M test jobs in last 6 months
- each test runs on a single cloud test node
- we have no servers
  - it all runs across HP and Rackspace public clouds


Puppet
------

- open source config mgmt system
- written in ruby
- models intended state
- wants to own entire system

Config management is great:

- repeatable and consistent machines
- code review
- collab from non-root users
- less repetition for me
- open source infra
- github:openstack-infra/system-config
- http://puppetdb.openstack.org/

Three ways to run:

- puppet apply
- puppetmaster + puppet agent daemons
- puppetmaster + puppet agent non-daemon

What about passwords and keys?

- Do not put any of those things in public git repo
- Hiera

Hiera:

- simple YAML database; sits on puppetmaster
- use for secret data
- puppet code is still complete
- breaks ability to use simple ``puppet apply``


Ansible for Orchestration
-------------------------

- open source system management tool
- written in python
- sequence of steps to perform
- works over SSH
- incremental adoption

Playbooks

- YAML syntax
- can encompass multiple systems
- variable interpolation
- executed via ``ansible-playbook``

Ansible organisation:

- plays
- playbooks
- roles

Ansible inventory:

- list of servers
- groups of servers
