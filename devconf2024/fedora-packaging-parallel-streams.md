# The new era of packaging: parallel streams in Fedora

- Filip Janus and Zuzana Miklankova, Red Hat

Multiple streams rationale

- competing goals
  - users want stable and unchanging system for the application
  - users want latest version with latest features
- can we satisfy both goals?
- how?
  - software collections
  - modularity
  - post-modular...

Definitions

- parallel availability: multiple streams available in repo
- parallel installability: multiple streams can be installed
  simultaneously

pre-software collections

- environment modules
- alternatives

software collections

- provide parallel installability
- wrapper around environment modules
- complicated spec file changes
- non-standard installation and data paths
- intsallable from separate repos from softwarecollections.org
- no changes in infra needed

Modularity (2017)

- many complex changes needed
  - in build system
  - in git
  - in maintainer workflows
  - in dnf
- many features provided
  - simple UI
  - handy dnf subcommands
  - chain builds
  - standard paths (cf SCLs)
  - parallel availability

RETIRE MODULARITY change agreed.


## Post-modularity

Goals

1. default stream definition
2. use existing infra
3. transparent for users
4. parallel availability
5. support version switch
6. standard install paths, service names

Definitions

- compat package
  - multiple versions of same package, different naming
  - e.g. zlib, zlib-ng

- virtual provide
  - allows to define provided symbols regardless of package name

Our solution

1. compat packages
2. virtual provides
3. versioned package names
4. default version definition

- explicit provides and conflicts e.g. `postgresql-any`

Challenges

- dnf behaviour
  - different between dnf builddep and dnf install

- srpm vs rpm names for default stream
  - koji and git use srpm name
  - dnf uses rpm name

- file requirements
  - e.g. `Requires: /usr/bin/postgres`
  - which stream should be picked
  - limitation of our concept

- if you stick to fedora guidelines you should be ok

## Demo

## Post-modular evaluation

Pros

- no infra changes
- lightweight solution for maintainers and users

Cons

- inconsistent approach through teams, components


