Managing Performance Parameters Through systemd
===============================================

Sander van Vugt (technical instructor)

- talk focuses on CPUShares (just one performance parameter)
  - user, system, and machine slices
- but generalises to other performance parameters
- hints on how to find available parameters

- demo VM is CentOS (6.7?)

- create unit files in ``/usr/lib/systemd/system/``

- unit file with ``ExecStart=/usr/bin/dd if=/dev/zero of=/dev/null``
  to make computer busy

- ``CPUShares=1024``
  - if only one task has ``CPUShares``, it means *nothing*
  - if > 1, then it is linear ratio between them
    - i.e. {512, 1024, 2048} means same as {2, 4, 8}
- ``CPUAccounting=on`` (on by default in CentOS)

- slices
  - user slice is env that is reserved for user-related job
  - system slice is env that is reserved for OS-related job
  - machine slice is optional, can be used for VMs
  - all get 1024 CPU shares by default

- switch off some CPUs
  - ``echo 0 > /sys/devices/system/cpu/cpu<N>/online``

- Configure usr/system/machine CPUShares in
  ``/usr/lib/systemd/system/{user,system,machine}.slice``

- users have slices (within the ``user`` slice)
- user slices have *scopes*; they are like a slice within a slice

- what parameters are available
  - ``systemctl show foo.service``
  - will consider the type of unit file, and show available
    parameters
  - examples: memory limits, OOMScoreAdjust, default Nice,
    BlockIOWeight, etc
  - see also systemd.resource-control(5).
