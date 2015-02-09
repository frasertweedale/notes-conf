System Management with CIM and OpenLMI
======================================

- Tomáš Smetana, Server Experience, Red Hat


About OpenLMI
-------------

- Remote API and tools for Linux systems management
- Standards based: WBEM/CIM
  - Web-based Enterprise Management
  - Common Information Model
    - Prevalent outside Linux
  - DMTF (Distribution Management Taskforce)
- Modular
  - Anyone who wants to make their software managable can do so.
- Oriented towards scripting and CLI interfaces


OpenLMI components
------------------

- CIM: Common Information Model
  - object model defining the management information for the system
- CIMOM: CIM Object manager
  - runs on every managed system
- CIM providers
  - plugins (DSOs) for the CIMOM
- OpenLMI Shell
  - Python-based interactive and scripting environment
- LMI metacommand
  - extensible set of scripts based on OpenLMI shell


Constraints
-----------

- builds on top of system APIs
  - interacts well with other components
  - doesn't interfere with manual changes
- avoid direct modifications of configuration files


HW and System Information
-------------------------

- hardware and firmware information
  - CPU, Memory, BIOS motherboard
- operating system info
  - kernel version, release name
- read only

Demo
^^^^

``lmishell``

- Python shell
- tab completion and paging
- all classes have documentation; ``<class>.doc()``
- connect to and inspect managed machines


Service management
------------------

- uses systemd
- start, stop, enable, disable, status
- dbus

Demo
^^^^

::

  s = ns.LMI_Service.first_instance({"Name": "smartd.service"}
  s.print_properties()
  s.StopService()
  s.refresh()  # refresh local objects


Storage
-------

- Can list and manage block devices
  - create FS, manage LVM, MD RAID, LUKS, mount FS
- based on Blivet library from Anaconda


Networking
----------

- Basic networking configuration
  - VLANs, bridging, bonding, teaming
- uses NetworkManager
- model based on SMASH DMTF profile


Package management
------------------

- List, install, remove, verify, delete
- based on PackageKit

User management
---------------

- based on libuser
- management of the *local* user/group accounts onlly
- eventually SSSD will provide D-Bus API for configuring users and
  groups


Power management
----------------

- shutdown, restart, etc.
- based on upower


Other providers
---------------

- fan
- realmd
  - enroll in AD, Krb5 domain
- PCP
- SELinux
- SSSD
- locale
- journal
- logical file
  - prepare mount points


What's next
-----------

- APIs are incomplete, inconsistent, undocumented
  - want NetworkManager for storage

- Services rarely provide API for their configuration
  - "Can I configure Apache with this?" ; "No"


Future possibilities
--------------------

- Make management component invisible on the managed system
- Local APIs remoting (remote D-Bus?)
- There is no configuration without API
- There is no monitoring without events
- Cntainers management / monitoring


Links
-----

- openlmi.org
- fedorahosted.org/openlmi
