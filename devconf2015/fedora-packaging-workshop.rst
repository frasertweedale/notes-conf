Fedora Packaging Workshop
=========================

- Matthias Runge <mrunge@fedoraproject.org>
- OpenStack developer, Red Hat


Software packages
-----------------

- a piece of software
- often compiled code
- metadata
  - description
  - license
  - manifest
  - dependencies
- installation / removal of packages


RPM
---

- cpio archive + metadata
- scripts are executed at
  - install time (before / after)
    - run daemon, add user, etc.
  - removal (before / after)
    - stop daemon, remove user, etc.
  - update
    - upgrade scripts
- packages are signed
- package install and updates are applied unattended
  - no user interaction is desired


Tools
-----

- ``rpm -qf /etc/nova/nova.conf``  "where does this file come from?"
- ``yum search openstack``
- ``yum install openstack-dashboard``
- ``repoquery --whatprovides "*/bin/bash"``
- ``dnf install openstack-dashboard``
- ``dnf builddep <packagename>``
- ``dnf repoquery "*/bin/bash"``
- ``dnf info python-flake8``


packaging environment
---------------------

- Fedora or RHEL clone
- ``yum install @development-tools``
- ``yum install @fedora-packager``
- ``yum install rpmlint``
- Don't even think to work as root
- mock: ``usermod -a G mock <username>``


Spec file
---------

- Header
  - name, version, release, license
  - sources tarball, (+patches)
- %description
- %prep, %build, %check
- %files
  - list of files
- %changelog


RPM macros
----------

- RPM comes with huge set of macros
- %sysconfdir /etc
- %prefix /usr
- %exec <shellcmd>


Get started
-----------

- rpmdev-newspec


Package build process
---------------------

- ``rpmbuild``
  - ``-bs`` -> SRPM
  - ``-bb`` -> binary RPM
  - ``-ba``
  - ``-bp``, ``-bc``

- Packages are built on a dedicated build host

- Mock

- koji

- fedpkg

- Afterwards: bodhi request, sign, push to testing


Build in Fedora build env
-------------------------

- ``koji build --scratch <dist> <srpm>``
  - dist is one of rawhide, f20, f21, epel7, dist-6E-epel
- ``fedpkg``
  - builds in dist-git


Package review
--------------

Submitter:

- upload spec and SRPM
- file request for review in Bugzilla
- sponsor?
- wait for reviews, respond, review+
- SCM request, import, build
- submit update

Reviewer:

- package complies to packaging rules
- License ok, SPEC readable, builds on rawhide, %files, name ok,
  deps ok
- no copied code from other projects
- common rules and e.g. python review guidelines
- ``fedora-review``, ``rpmlint``

Become sponsored:

- you need sponsorshop for first package
- convince someone to sponsor you
  - do non-official reviews
  - submit new packages for review
- become a co-maintainer
- once you're in fedora's packager group, you can/should review
  others' packages


More tools
----------

- rpmdev-setuptree
- pyp2rpm -n Pint > python-pint.spec


More ideas
----------

- fedoraproject.org/wiki/Package_maintainers_wishlist
- fedoraproject.org/PackageReviewStatus/


Bugs
----

- fedpkg should clone anonymously or fail over to anonymous when
  clone fails
