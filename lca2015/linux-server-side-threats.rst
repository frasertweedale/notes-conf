Advanced Linux Server-side Threats
==================================

How they work and what you can do about them.

Olivier Bilodeau, ESET (@obilodeau)

- Malware researcher at ESET
- Infosec lecturer at ETS University in Montreal
- Previous:
  - FLOSS Perl dev
  - sysadmin for large Canadian ISP

- Recent evolution of server-side malware
- Windigo in-depth
- Forensic and incident response

In the past:

- old-school defacement
  - "low hanging fruit" targets
- damage "for the lulz"

Evolution of server-side malware:

- motivated by money
- why crimeware on servers?
  - always up
  - almost always reachable
  - good bandwidth
  - good IP reputation
  - may contain sensitive information

Why care?

- infected servers are put on blacklists
- AV blacklists -> web visitors get warning
- Spamhaus' XBL -> server can't send email
- Google's Safe Browsing API -> web visitors get warning

Crimeware evolution; recent history
-----------------------------------

``.htaccess`` redirection

- done using ``mod_rewrite``
- show advertising, etc
- redirect only when ppl coming from search engine as detection
  countermeasure

Darkleech:

- apache module for redirecting web traffic
- sold for $1000 on underground forums
  - malware-as-a-service for Linux
  - multiple operators and campaigns

Phalanx / Phalanx 2:

- effective and clever rootkit
- hooks kernel syscalls by injecting code inside kernel
- not that portable; kernel ABI change may break it
  - this factor is pushing malware to userland stack


Operation Windigo
-----------------

- Crimeware op consisting of several malware components
  - linux/ebury
  - linux/cdorked
  - perl/calfbot
- infrastructure mostly operated on compromised server

Who?

- CERT Bund
- CERN
- ESET
- SNIC (Sweden)
- law enforcement

Linux/Ebury
-----------

- openssh backdoor
- patch ``libkeyutils.so`` to load ``libns2.so`` and hooks OpenSSH
  addr space
- steals all passwords and keys
  - from both server and client
- provide backdoor root shell to operators
  - doesn't leave traces

How does the shlib work?

- constructor fn executed when loaded
- detect main bin
- hook imported fns such as ``crypt`` and ``syslog``

How is info exfiltrated?

- DNS packet with all required info such as username, target IP addr
  and port
- RSA encrypted
- Keys are kept in memory and are later fetched by ops via ``Xcat``
  command

Backdoor interaction

- embed password in special SSH client version identifier string
  - not encrypted; can be sniffed to detect infection
- five commands:
  - Xver: print version
  - Xcat: print stolen creds
  - Xbnd: choose bound IP addr for SSH tunnel
  - Xpsw: set 4 byte xor key for future backdoor usage
  - none: get shell

Linux/Cdorked
-------------

- httpd/nginx/lighttpd backdoor
  - replace bin on server
- redir reqs on legit site to exploit packs or affiliate links
- use shared mem (POSIX IPC) for state and config
  - no file on disk
  - encrypted with static XOR key, unique per infection

- many conditionals on which redirection is predicated
  - only once per 24 hrs
  - some language strings not redirected
  - presence of admin panel refs in URL

From there:

- reverse engineer the domain generation algo
- buy a domain to get the creds that were sent to DNS
  - witnessed 7000 infected servers
- access to compromised systems through notifications

Perl/Calfbot
------------

- Perl spamming daemon
- deletes itself when it starts; resides only in memory
- hides as ``crond``
- does "test jobs" against operator-owned addresses before receiving
  send jobs.
- 35M msgs / day
  - mostly adult, gambling, dating sites / services

Infections

- 1888 Linux, 61 BSD, 241 unknown, 19 OS X, 2 Windows (under Cygwin)
- Data from user agent strings observed on counter-compromised
  intermediate server

How does it expand?
-------------------

- username distribution of stolen creds
  - 40% "root"


Why advanced?
-------------

- Stealthy and effective


DevOps malware operators?
-------------------------

- interesting monitoring and deployment scripts
- interesting usage (SSH stream redir)


Recon / deployment scripts
--------------------------

- Perl scripts
- eliminate evidence

Recon scripts

- check for ``LD_PRELOAD`` tricks
- various restrictive ssh configs
- BSD jails
- CPanel, BRadmin, Nagios, etc
- Generic SSH honeypot
- Check if Ebury already installed
- Bails out if it sees stuff like this

Deployment:

- use Perl's ``DATA`` to pass files through ssh
- alter package manager manifests
- ``LD_PRELOAD`` trick to fake RPM install date
- signed RPMs

Monitoring:

- Bash
- Grab keys, known hosts, user ssh configs

Other findings:

- modify SELinux policy
  - understood SELinux more than most sysadmins

- various styles of installation
  - precompiled libraries
  - on-site compilation
  - packages

- looks for over 40 backdoors and rootkits
  - some of these were not publically known

Forensics and incident response
-------------------------------

Caution

- running at same priv level -> arms race
- aim for out-of-band (network or mem acquisition)

Evidence gathering

- auditd worked best for us
- eventually needed to MitM

Process analysis

- gcore
- ``proc`` allows you to extract deleted executables
  - ``cp /proc/<pid>/exe``

Caution

- always copy everything from ``/proc/$pid`` before killing
  - e.g. encryption keys in envvar

Process analysis tools

- strace
- ltrace (lib calls)

Reverse-engineering Perl

- ``perltidy``
- rename vars
- ``B::Deparse``


Network evasion

- SSH tuns (spam)
- nginx rev proxy (redir)
- IP in IP tuns (hide all sorts of traffic)
- iptables (NAT to bounce/rdr traffic)
- 3Proxy


OOB forensics

- wireshark
- dd
- LiME (memdump) or virt snapshots


Incident response
-----------------

- do be in denial
- reinstall everything
- do not reuse creds
  - consider password policy
  - use 2FA (FreeIPA!)

Please to system devs

- logs should be harder to tamper with
- same for package manifests
- verify package integ from live CD
- yum: list gpg signer would be good

Closing words
-------------

- spread word on detection and prevention techniques
- help cleaning infected systems
- work on making ecosystem more resistant
- send us anything suspect that you find
  - windigo@eset.sk
