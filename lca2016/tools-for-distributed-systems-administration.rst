Open Source tools for distributed systems administration
========================================================

Elizabeth K. Joseph, HPE, OpenStack Infrastructure team

How most open source projects do infrastructure:

- team (or company) manages it... or just use code hosting
- requests are submitted via mailing list, bug report or ticket
  system
- request priority is determined by core team
- this may be similar to your organisation
- is there a better way?

OpenStack Infrastructure Team

- our job is to make sure OpenStack devs can do their job
- all system configs are open source and tracked in Git
- anyone in the world can propose patches for direct inclusion in
  our infra
- we all work remotely, worldwide: US, Russia, Australia, Spain

What we run:

- CI
- ELK
- etherpad
- git
- ... LOTS more

OpenStack CI system:

- >800 individual projects
- all projects must work together
- gated trunk
- code should be syntactically clean
- testing must be completely automated
- multiple changes every minute

Tools for CI:

- launchpad (for authn; some day: openstackid)
- Git
- Gerrit
- Zuul*
- Gearman
- Jenkins (with jenkins-job-builder*, devstack-gate*)
- Nodepool*
- * = started by OpenStack Infra team

Automated tests for infra:

- flake8 (linter)
- puppet parser validate
- puppet lint
- puppet application tests
- XML checkers
- alphabetized files
- IRC channel perms

Peer review means...

- multiple eyes on changes prior to merging
- good infra for developing new solutions
- no special process to go through for commit access
- trains us to be collaborative by default
- because anyone can contribute, anyone can devote resources to it

Automated deployment

- change gets approved, tested and merged
- puppet master gets updated and applies changes

Can you really manage infra via git commits?

- Cacti (cacti.openstack.org) to keep an eye on server usage
- PuppetBoard (puppetboard.openstack.org) so you can watch your
  changes get applied
- Thorough, specific documentation at
  http://docs.openstack.org/infra/system-config

Well, not *everything*:

- automation is not perfect, sometimes you just need to log into a
  server
- complicated migrations and upgrades need manual components (but we
  automate more every time!)
- initial persistent server deployment still has manual components
- passwords need to be privately managed (but we use git!)

Maintenance collaboration on Etherpad

- problem statement
- steps written out in words
- exactly commands we're going to run
- allows for peer review of what we're going to do
- can easily annotate with who has done what when the maintenance is
  underway

Human collaboratoin

- IRC: #openstack-infra, #openstack-infra-incident,
  #openstack-sprint, #openstack-meeting
- All logged
- Pastebin: http://paste.openstack.org
- in-person collaboration at the OpenStack Design Summit every 6mths

And then there are time zones :(

- increased coverage is great, but...
  - the first core/root member in a particular region struggles to
    feel cohesion
  - increased reluctance to land changes in prod
  - only solved by increasing coverage in that time zone so they're
    not alone

Mostly it's pretty great!

- Lots of OpenStack companies are hiring!
- always looking for new contributors
