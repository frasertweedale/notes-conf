Lightweight Inventory Management with Pallet Jack
=================================================

Karl-Johan Karlsson

Pallet Jack:

- inventory mgmt for sysadmins
- store system info
  - IP, location, serial number, etc
- generate e.g. salt configs from info in db

Goals:

- SPOT for sysadmin
- version control <> config mgmt
- easily interoperable
- reasonably scalable
- decomposable database (both vertically and horizontally)

Data store:

- "warehouse" with "pallets" (objects)
- objects are dirs
- keys and vals stored in YAML files
- inheritance by symlinks and dir hier
- can store in git
  - can use existing code review workflow to review changes

transformations:

- synthesize new vals form existing vals
- value subst and regex
- e.g. store hostname and domain name separately; compute FQDN

db tools (cli):

- create_system ; dump_pallet ; etc

data export tools:

- e.g.: palletjack2kea (for kea DHCP server)
- can run on each change to database

current integrations:

- config mgmt: salt
- installation: PXELINUX and kickstart
- DHCP: kea
- canon dns: knot
- recusrive dns: unbound
- IdM: FreeIPA integration !

API:

- ruby

Try it:

- gem install palletjack-tools
- saab-simc-admin/palletjack ("forklift" was overloaded)
- by "Saab Simulation Centre"

Questions
---------

- This looks like LDAP?
  - There is no schema/validation (until export time)
  - Users can put any data; data interpreted by "exporters" who can
    skip/fail/report/whatever according to their needs
