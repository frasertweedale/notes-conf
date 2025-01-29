# Trip Report - Everything Open 2025

[Everything Open 2025](https://2025.everythingopen.au/) was held in
Adelaide on 20–22 January.  It is something of a successor to
linux.conf.au.  Talks ranged from deeply technical to higher-level
topics like the free software movement, community management, open
data and open culture.  As in previous years Red Hat was a major
sponsor, with a number of associates presenting or attending.

Talks should be published in time on the [Everything Open YouTube
channel](https://www.youtube.com/@EverythingOpen) and the [Linux
Australia
mirror](https://mirror.linux.org.au/pub/everythingopen/2025/), but I
do not know the precise timeframe.


## Red Hatters at EO2025

There were 8 presentations by Red Hatters.  Due to schedule
conflicts I didn't make it to *any* of these sessions (apart from my
own) but I look forward to watching the recordings.

- [*The Storage Shift*][talk-storage] by Steve Ellis.  A
  comprehensive overview of storage technologies including for cloud
  native applications.

- [*Facilitation - How Might We "make it easier" for people to
  collaborate onsite and online?*][talk-facilitation] by Donna
  Benjamin.  A workshop about *"How Might We?"*, an open practice
  from the world of design thinking that invites people to reframe
  and open up their problem statements.

- [*Beyond TAP: polyglot testing with exeter*][talk-exeter] by David
  Gibson.  Exeter is a tool for defining and running tests for
  solutions involving multiple programming languages.

- [*All the wonderful things you can do with
  StackRox*][talk-stackrox] by Shane Boulden.  Shane explains and
  demos StackRox, which is the upstream project for Red Hat Advanced
  Cluster Security for Kubernetes.

- [*Open source your code, not your keys and secrets*][talk-secrets]
  by Alistair Chapman.  How to keep your secrets safe when building
  in the open, what are risks that lead to secret compromises, and
  the tools needed to prevent them.

- [*Avast ye, we be getting the clouds shipshape! Cloud networking
  with Skupper*][talk-skupper] by Apollo Bradshaw.  Skupper is a
  networking technology that lets you stretch applications across
  multiple Kubernetes clusters.

- [*tips to build and repair empathy with other teams*][talk-teams]
  by Cait Macleod.  Useful tips, resources and mindsets for
  repairing team relationships.

- [*Partly Cloudy IPA - joining cloud VMs to FreeIPA*][talk-ipa] by
  Fraser Tweedale.  My presentation on the Podengo project, which
  enables the Insights "Domain Join" feature which recently went
  into production.  Further comments below.

Additionally, Paul Wayper, Alistair Chapman and I gave lightning
talks on various topics.  Donna Benjamin, Cait Macleod, Steve Ellis
and Mandi Buswell served on the session selection committee, and
Neill Cox and Charelle Collett volunteered during the event.  It is
possible I missed some folks; apologies if so!  Thanks to all the
Red Hat associates who contributed to the success of the conference!

[talk-storage]: https://2025.everythingopen.au/schedule/presentation/98/
[talk-facilitation]: https://2025.everythingopen.au/schedule/presentation/78/
[talk-exeter]: https://2025.everythingopen.au/schedule/presentation/96/
[talk-stackrox]: https://2025.everythingopen.au/schedule/presentation/97/
[talk-secrets]: https://2025.everythingopen.au/schedule/presentation/73/
[talk-skupper]: https://2025.everythingopen.au/schedule/presentation/110/
[talk-teams]: https://2025.everythingopen.au/schedule/presentation/109/
[talk-ipa]: https://2025.everythingopen.au/schedule/presentation/68/


## Session highlights

- [*Sandboxing untrusted code with WebAssembly*][talk-wasm] by Katie
  Bell.  Very interesting session deep diving into WebAssembly
  runtimes and the WebAssembly System Interface (WASI) spec.  Katie
  explained how these tools can be used to run untrusted code, and
  the limitations.  There was an interactive demo, where the
  audience were invited to submit bot implementations for a
  rock-paper-scissors tournament!

- [*Why 99% Of WordPress Vulnerabilities Are Utterly
  Irrelevant*][talk-vulns] by Cameron Jones.  This presentation
  really pressed home the point that CVSS scores usually say very
  little about a real-world user's risk of exploitability or
  exploitation.  The biggest risk is usually the user themselves,
  through poor security hygiene or insider threats.  The [*Exploit
  Preduction Scoring System (EPSS)*](https://www.first.org/epss/)
  was brought up during the Q&A.

- [*Towards Editing Programs via Abstract Syntax Trees*][talk-ast]
  by Clinton Roy.  Presents a vision of new ways of reading and
  writing programs, departing from the "programming = editing
  strings" paradigm.  Practical examples using
  [tree-sitter](https://tree-sitter.github.io/tree-sitter/).

- [*Cargo Cult Standards Compliance: Proprietary Standards in an
  Open Source World*][talk-standards] by Sam Bishop.  The problem of
  paywalled standards (ISO, IEC, etc) and how it affects the free
  software movement.

- [*A Big Live Demo of Kanidm*][talk-kanidm] by William Brown.
  William used to work at Red Hat in the IdM group and is nowadays
  at SUSE doing similar work.  He gave a huge live demo of
  [Kanidm](https://kanidm.com/), an identity management solution
  written in Rust.  It does LDAP with replication, OAuth2.0/OpenID
  Connect IdP, machine log-in (PAM, NSS, etc), homedir automount,
  passkeys (supporting attestation!), TPM credential binding, and
  more.  One of the most amazing features is support for on-the-fly
  user renaming.  A Kerberos KDC is in the works.  (No PKI yet,
  though).

  It also supports a kind of "migration" or integration with
  FreeIPA—as far as I could tell FreeIPA remains the authentication
  endpoint but Kanidm can override specific attributes, similar to
  AD user overrides in IPA.  I will catch up with William in a
  couple of weeks to drill into it a bit more.

  Kanidm seems an amazing achievement for a project that is only a
  few years old.  It is one for Red Hat IdM engineering to keep an
  eye on.  In the spirit of free software there is always
  opportunity for our projects to learn from one another.

- [*Capture the Flag*][workshop-ctf] hosted by Rob Kenefeck.  A Kubernetes
  capture-the-flag contest was held on the final day of the
  conference, alongside other tracks.  The objective is to exploit
  security weaknesses and misconfigurations in an environment set up
  for the CTF scenario, and capture the hidden "flags".  I
  participated to refresh my Kubernetes and containers muscles,
  because it was a couple of years since I worked directly with
  OpenShift.  It was a lot of fun and took up most of the day, and I
  managed to capture all but the final flag (I got it a few
  minutes after the deadline).

[talk-wasm]: https://2025.everythingopen.au/schedule/presentation/100/
[talk-vulns]: https://2025.everythingopen.au/schedule/presentation/86/
[talk-ast]: https://2025.everythingopen.au/schedule/presentation/74/
[talk-standards]: https://2025.everythingopen.au/schedule/presentation/105/
[talk-kanidm]: https://2025.everythingopen.au/schedule/presentation/57/
[workshop-ctf]: https://2025.everythingopen.au/schedule/presentation/113/


## My session: Insights Domain Join

I presented [*Partly Cloudy IPA - joining cloud VMs to
FreeIPA*][talk-ipa], introducing and demonstrating the Insights
*Domain Join* feature on Red Hat Hybrid Cloud Console, and
explaining the implementation.  In summary, customers can register
their RHEL Identity Management (IDM; upstream = FreeIPA) deployments
with our service, and when they launch VMs that include the client
RPM, our service will introduce the VM to the IDM deployment to
facilitate a secure, automatic join.  The talk included a live demo.
The upstream project is called
[Podengo](https://github.com/podengo-project).

Regrettably I packed a bit too much content into the talk, getting
through it all but leaving no time for Q&A.  However, several people
gave feedback in the "hallway track".  All were impressed with the
demo.  There were several comments in support of adding Active
Directory support (one of the "future ideas" mentioned in the talk).
Also, William Brown suggested a possible security enhancement:
blocklist the RHSM certificate after a successful join to prevent
reuse.

Some people wondered about using Podengo in internal (single tenant)
environments.  We would be happy to see community engagement on
this, but it would mean a modest developer effort to generalise or
abstract some parts of the implementation (which I was transparent
about in the presentation).

Overall the topic was well received.  Furthermore, variations of
this talk will be presented by my colleages André at FOSDEM, and
Akshay at DevConf.in.  So we are getting a great "return on
investment" for the [slide deck] and demos.

[slide deck]: https://speakerdeck.com/frasertweedale/partly-cloudy-ipa-joining-cloud-vms-to-freeipa
