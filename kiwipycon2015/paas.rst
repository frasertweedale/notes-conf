My own PaaS in Python, which you can do too
===========================================

- Lee Begg
- @llnz2
- http://begg.digital/

What is Paas?

- Platform as a service
- app hosting in language environment
- normally built on IaaS

PaaS for Python

- WSGI / HTTP socket environment
- database provided
- generally in a virtualenv

My approach

- Nginx (static / media / proxy)
- circusd (process and socket manager)
- chaussette (WSGI server)
- Git
- Fabric + Cuisine (deployment)
- Django-environ (config mgmt)

- (chaussette and circus are Mozilla projects)

App configuration

- e.g. SECRET_KEY, db conn params, etc
- don't commit in code
- managed by PaaS thru files (c.f. env vars a la heroku)

Alternatives

- OpenShift
- Herkou
- Google App Engine
- Gondor
- Cloud Foundry

The future

- systemd
  - process control features (like circus)
  - socket activation (like circus)
  - cgroups (isolation)
- containers?
