Type-driven configuration management with Propellor
===================================================

Joey Hess

Best practices for system configuration:

- simple file format (e.g. INI)
- declarative (e.g. systemd)
- compositional

Config mgmt systems in common use:

- Ansible
  - "simple" YAML
  - vars
  - conditionals
  - loops
  - turing complete
- Puppet
  - INI
  - variables
  - hashes
  - separate lang for conditionals and loops
  - might be turing complete

- Turing tar-pit
  - "everything is possible but not everything of interest is easy"
    - Alan Perlis
  - Not declarative
- Embedded DSLs
  - make common cases easy
  - type checker to avoid bad configurations

Propellor

- types
  - DebianSuite, Architecture, Package, User, Group, SshKeyType, etc
  - over 150 such types

- Want bad combinations of properties to be a type error
- Type errors are not bad, they just mean that the compiler saved
  you from shooting yourself in the foot

- ``RevertableProperty`` (property that can be reverted)
  - Q: is ``RevertableProperty`` a type synonym

- Properties just model the system.
  - you're not pinning down everything about the system with types
    - you can't, but they do help

- Containers
  - systemd, docker, FreeBSD jails, etc

- can introspect the ``Info`` of a ``Host``

- ``Info`` propagation problem
  - ``Property a`` where a can be ``HasInfo`` or ``NoInfo``

  - type operator ``+`` (type-level lists)
  - type errors for, e.g., FreeBSD properties in Debian hosts.

- type-level port conflict detection
  - implemented but not merged yet
  - joey runs a lot of tor bridges
