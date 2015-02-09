How to write SELinux policy for your project painlessly
=======================================================

- Lukáŝ Zapletal @lzap
- Not SELinux administration
- Not a step-by-step tutorial on creating policies

What is SELinux
---------------

- makes sure that **subject** (process) plays by the rules
- selinux tags can be assigned to processes or resources
- policy = {allow,deny} + tag + resource_tag + action
- MSC (multi-category security)
- MLC (multi-level security)

What can SELinux do for you
---------------------------

- increase security
  - prevent attacks
  - restrict actor after successful attack
  - warns during attacks (denials)
- find software bugs
  - unchecked file open retvals
  - leaked descriptors
- prevent undesirable behaviour in proprietary software

SELinux policy in Fedora
------------------------

- ``selinux-policy``
- ``selinux-policy-targeted``
- ``selinux-policy-devel``

Custom policy - hello world
---------------------------

- mypolicy.te (type enforcement)
- mypolicy.if (interfaces and docs; public interface)
- mypolicy.fc (file context)

Makefile:

- targets: all, load/reload, refresh (reload all policies), clean
- important vars: NAME, TYPE, ...

Need a domain entry point:

- standalone binary: ``domain_entry_file``
- daemon

File context:

- regular expressions to apply tags to files
- ``gen_context``

Important interface files:

- application.if
- corenetwork.if
- files.if
- miscfiles.if
- devices.if
- terminal.if
- ...
- apache.if
- abrt.if

Tools
-----

- Book: *SELinux Cookbook* Sven Vermeulen
- https://github.com/lzap/vim-selinux


Anatomy of SELinux denial
-------------------------

- ``grep AVC /var/log/audit/audit.log``
- ``ausearch -m AVC``
- ``audit2allow`` turns audit denials into allow policy
  - to activate, execute ``semodule -i qiuckfix.pp``
- permissive + ``audit2allow`` = ⚡
  - file contexts
  - domain transitions
  - software bugs are hidden
- policy is not the only artifact
  - the process is just as important; find design issues,
    misconfigurations, bugs
- writing a policy is like audit; it is making your software better

Take small steps:

- modify, compile, load, commit, repeat
- one issue -> one commit


Who should write policies?
--------------------------

- developers
- review code; must not be one-man-show
- when unsure, ask SELinux team

Multiple distributions:

- ``distro`` flag from Makefile
- must be turned on manually
- ``ifdef(`distro_rhel6') ...``


Deployment
----------

- send patch to selinux-core-policy if subject is in official repos
- if in 3rd-party repos, ship (and enable) a policy and relabel
  installed files


Final words
-----------

- You will not be famous
- SELinux is usually not a product feature

How to file a SELinux bug:

- Processes: ``ps axuZ``
- Files: ``restorecon -rvn /``
- Denials: ``ausearch -m AVC``
