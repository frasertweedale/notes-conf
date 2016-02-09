linux.conf.au was held from 1 - 5 January in Geelong, a regional
city near Melbourne.  It is the largest free software conference in
AU/NZ.  This year the conference was sold out with ~600 delegates
(less than other years; numbers were limited by venue).

I'll outline some of the relevant talks below and provide links to
my notes and the talk video.

I also ran the FreeIPA tutorial so let me start with that.  See also
the attached response summary (pdf) for the feedback form.


FreeIPA tutorial
----------------

- ~40 attendees (full house)
- Most actively participating
- Most participants had done the prep

Platform / prep issues:

- One fellow had an i386 kernel
  - luckily he told me the night before
  - so I built an i386 Vagrant box just for him
  - but it seems my USB stick's FS was corrupt so I couldn't get it
    to him :(
  - but soon after, he buddied up with the fellow adjacent :)

- One chap was using Windows 10; something didn't work with Vagrant
  or VirtualBox
  - I had only verified on Windows Vista
  - Couldn't do much for him, just suggested to search the error
    message and try and find help on web

- I had failed to update a version number in URL vagrant box online.
  If downloading the box directly, resulted in 404.  This tripped
  some people up.  Was quickly fixed during workshop.


Issues during workshop:

- Lots of people needed to flush SSS cache at one stage
  - A participant submitted pull request for curriculum before end
    of workshop.  Yay collaboration! \o/

- One person complained VM(s) were very slow.  He did not have
  hardware-assisted virtualisation enabled in BIOS.
  - I will update preparation steps to make a note of this


Requested content:

- Several people asked about sudorules.
  - I will write a module (unless Thorsten does it first)

- Someone wanted OTP to be covered
  - Assumption is that user has an OTP app, so the module would have
    to be optional
  - but it is a good opportunity to promote FreeOTP!

- Hostgroup dynamic rules
  - Someone wanted to know how these work


Questions that arose during workshop:

- Q: what is the hbacrule evaluation order
  - A: good question.  I didn't know and need to learn about this.

- Q: can you install freeipa-client on IPA server?
  - A: yes, it already is installed as part of server install, and
    IPA server machine is a client of itself.
  - The pupil seemed pleased by this consistent.
  - They were thinking about installing FreeIPA server on their
    laptop just to have OTP for login to machine, etc.

- Q: Does SSSD do Group Policy enforcement?
  - Nope.  Does Samba do (some of) that?  Maybe that's what they
    want.

- Q: Containers as IPA clients?
  - I pointed them to Jan's work and explained that the IPA client
    is installed in initial ``docker run``, with environment
    variables used to convey required info

- Q: Can IPA server be "joined" to AD domain?
  - A: I wasn't sure precisely what was meant but explained that
    trusts are the supported means of integrating with AD.  We
    discussed further and it became clear that they just wanted AD
    users to be able to access IPA-enrolled services.  They were
    satisfied to know that trusts enable this.

- Q: One person asked about "connectors"
  - This terminology comes from another identity management system;
    I forget the name (it wasn't AD).
  - It is basically a mechanism to sync data from IdM to other
    identity silos, push-based.
  - A: I explained that we do not do that or encourage it, the
    preference being to configure applications to use Kerberos,
    pam_sss or even plain LDAP to FreeIPA's DS instead of
    duplicating identity data.


Feedback:

A form was provided for participants to give feedback about the
workshop.  Response rate was ~50%.  Response summary (PDF) is
ATTACHED.  Highlights:

- Almost all attendees had sysadmin in background, a bit under half
  had dev in background and under 1/3 had tech support in
  background.

- Most attendees had moderate or strong level of familiarity with
  identity management before workshop.

- There was a strong upward shift in reported likelihood of
  recommending FreeIPA before/after the workshop, from 35% likely or
  highly likely to 85% likely or highly likely.

- Aspects that participants felt were poorly executed or not useful
  - One repspondent felt that manual enrolment (cf OTP) was a bad
    practice to show
  - Various specific technical issues about the curriculum

- Topics NOT covered that they wanted covered
  - sudorule module was a big request
  - AD trust (even just as a demo)
  - A few requests for 2FA (worth doing as an optional module IMO,
    and an opportunity to promote FreeOTP)
  - Requests for more detailed overview of Kerberos and X.509


Conclusion:

The workshop well attended and IMO was a success.  Most people who
were actively participating and did not have major issues were able
to complete or get most of the way through the curriculum.  I
encouraged people who had issues or didn't finish the workshop to
return to the curriculum in future, if they are able.  Many people
expressed their thanks and that they have learned a lot.

Someone suggested that I propose a Kerberos talk for a future
linux.conf.au, explaining the use cases and protocol in detail.
Many people seem mystified by it and some expressed surprise that it
is still actively used, actively developed and indeed highly
regarded by some folk :)  OSDC 2016 and linux.conf.au 2017 are still
far off and I'm sure I can take a lot from Alexander's talks to
construct a compelling abstract.

As for future workshops, given it was accepted for both OSDC and
linux.conf.au, I do not think a sequel is likely to be accepted in
the next couple of years.  I always have my eye on conferences in
APAC so hopefully more opportunities will arise.  I feel like there
has already been a good return on the effort put into developing the
curriculum but more we can run the better, if they are well
attended.  Curriculum development will now be incremental - a big
shout out to Thorsten who ran the DevConf FreeIPA workshop(s) for
already contributing a new module!


Secretd: storing and distributing secrets (Tollef Fog Heen)
-----------------------------------------------------------

Basically a key escrow service for use in cloud (similar space to
Tang).  Audit (which clients asked for and received which keys) was
considered a key requirement.  Keys were stored in a heirarchical
structure (AFAICT clients could ask for a "subtree" of keys).
Uses SSH for authentication and encrypting channel.

I asked about how SSH host private key is protected.  Answer was
basically "if your host is compromised to that extent you have
bigger problems".

It seems like the system was written around requirements like
PCI-DSS.  If it has not been done already, people who understand
PCI-DSS should review Tang to work out if there are gaps and whether
or how they should be addressed.

Notes: https://github.com/frasertweedale/notes-conf/blob/master/lca2016/secretd.rst
Video: http://mirror.linux.org.au/linux.conf.au/2016/05_Friday/Costa_Hall/secretd_another_take_on_securely_storing_credentials.webm


Using TPM to Protect Users (Matt Garrett, CoreOS)
-------------------------------------------------

mjg explained TPM and discussed remote attestation before proposing
some non-remote-party methods to use TPM to ensure platform
integrity.  I think he did a similar talk at 32C3.

The interesting proposal is to "seal" a TOTP seed to TPM state.  It
can only be decrypted if TPM PCRs have not changed.  Having
decrypted seed, software displays current TOTP value, which can be
checked against another device e.g. phone in which seed is also
enrolled.  TOTP is used because static value need only be observed
once by attacker, then included in malware.  Requires good op-sec
(e.g. don't leave both phone and laptop unattended).

Very interesting talk, highly recommended.

Notes: https://github.com/frasertweedale/notes-conf/blob/master/lca2016/using-tpm-to-protect-users.rst
Video: http://mirror.linux.org.au/linux.conf.au/2016/05_Friday/Costa_Hall/Troublesome_Privacy_Measures_using_TPMs_to_protect_users.webm


Security and Accessibility (Nicolas Steenhout)
----------------------------------------------

Nick talked about motivations for good accessibility before
discussing accessibility issues with common web security measures
such as CAPTCHA, session timeouts, data validation.

I asked Nick about accessibility of 2FA (physical token, soft token
apps) solutions, federated sign-on (e.g. "login in with
{Google,FB,GitHub,...}) and true SSO such as Kerberos.  He did not
know but I have followed up with him and he is going to look into it
and get back to me.

Notes: https://github.com/frasertweedale/notes-conf/blob/master/lca2016/security-and-accessibility.rst
Video: http://mirror.linux.org.au/linux.conf.au/2016/05_Friday/D2.193_Percy_Baxter/Accessibility_and_Security.webm


The Machine: Hardware and Software (Keith Packard, HP)
------------------------------------------------------

The Machine is HP's prototype distributed memory-driven compute
architecture.  The talk is interesting but naturally the security
parts are the most interesting  :)   They plan to use TLS to secure
comms between nodes, with private keys in ROMs on the node hardware.
Accordingly, an X.509 PKI is needed to issue certs.  In later
discussions with Keith it does not appear that the needs are
anything special.  They are also looking at including a TPM on the
next version of the hardware (which will still be prototype).

Notes: https://github.com/frasertweedale/notes-conf/blob/master/lca2016/architecture-of-the-machine.rst
Video: http://mirror.linux.org.au/linux.conf.au/2016/04_Thursday/Costa_Hall/Hardware_and_Software_Architecture_of_The_Machine.webm


Assorted security topics in open cloud (Jason Cohen, HP)
--------------------------------------------------------

This talk was basically about TPM and remote attestation to build
*Trusted Compute Pools*.  Goes into some detail on attestation
chains of trust and *OpenAttestation* (OAT) server, which is
allegedly a pain to deploy.

At the end he also mentioned *Direct Anonymous Attestation* which is
a privacy-preserving attestation protocol, but there was no time for
detail.

I am interested to know: what is Red Hat's offering in terms of
trusted compute with OpenStack?

Notes: https://github.com/frasertweedale/notes-conf/blob/master/lca2016/assorted-security-topics-in-open-cloud.rst
Video: http://mirror.linux.org.au/linux.conf.au/2016/01_Monday/Costa_Hall/Security_Topics_in_Open_Cloud_Advanced_Threats_2015s_Vulnerabilities_Advancements_in_OpenStack_Trusted_Computing_and_Hadoop_Encryption.webm


MediaGoblin (Ben Sturmfels)
---------------------------

GNU MediaGoblin is a web publishing platform than handles a variety
of media and is intended as a free replacement for, e.g., YouTube,
Flickr, SlideShare, etc.  It is a "young but ambitious" project.

Current practice is that each GMG instance is an identity silo, but
federation is planned in future (no detail given).  Default user
store is a table in GMG's SQL database.  Common deployment practice
is to run behind Apache/Nginx reverse proxy.

I discussed with Ben and he did not know whether GMG observes
external authentication (``REMOTE_USER``) but noted that it would be
simple change if it did not.

Notes: https://github.com/frasertweedale/notes-conf/blob/master/lca2016/mediagoblin.rst
Video: http://mirror.linux.org.au/linux.conf.au/2016/05_Friday/D4.303_Costa_Theatre/Preventing_Catastrophes_with_GNU_MediaGoblin.webm


Functional Programming Miniconf
-------------------------------

You did not think you would get away without me spruiking FP, did
you?  I ran the FP Miniconf on the Tuesday.  It was well attended
and I think a success.  I particularly commend *The Essential Tools
of Open Source: Functional Programming, Parametricity, Types* by
Tony Morris (data61) and *Haskell is Not For Production and Other
Tales* by Katie Miller (Facebook).

Talks: http://mirror.linux.org.au/linux.conf.au/2016/02_Tuesday/Wool_Museum/


Final notes
-----------

I took notes for most of the talks I attended[1]; many more than
those mentioned above.

All videos from the conf are now available on the Linux Australia
mirror[2].

[1] https://github.com/frasertweedale/notes-conf/tree/master/lca2016
[2] http://mirror.linux.org.au/linux.conf.au/2016/
