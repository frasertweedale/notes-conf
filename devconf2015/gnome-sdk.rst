Gnome SDK - a better way to ship apps
=====================================

- Alexander Larsson


Applications
------------

- what is an application?  Too broad.

- more interesting qn: what is an OS?

- is a distro an OS?
  - I would say not
  - *contains* OS, but also apps

- There is no way for the "Distribution" model to scale in the way
  the Apple App Store has.
  - There has to be a way for a developer of an app to do their own
    release without distros having to "pick it up"

- Solution: bundling?

- Idea: runtimes

- Idea: sandboxing
  - Important, but not everything


Gnome SDK
---------

- the core things an app needs
  - client X libs
  - glibc
  - mesa
  - crypto
  - gtk et al
- and does not have:
  - x server
  - udev

New complexities:

- Runtime prereqs: kernel, services
  - e.g. if you're using some new syscall, it better be in the
    kernel
- IPC protocol compatibility between host and app
  - e.g. dbus
  - we will need to be more careful about forward compatibility


Tooling
-------

- choose a base runtime
- it comes with a corresponding devel runtime
- build using xdg-app
- alternatively, Gnome SDK contains rpm/rpmbuild
- configured to build RPMs into /self

Future
------

Sandboxing:

- wayland
- kdbus
- selinux   \_ abstracting over confinement technology?
- cgroups   /
- use more namespaces
- portal DBus APIs for sandboxed apps
