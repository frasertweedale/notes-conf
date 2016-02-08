FreeIPA workshop
================

- ~40 attendees (full house)
- Most actively participating
- Most participants had done the prep

Platform / prep issues
----------------------

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


Issues during workshop
----------------------

- Lots of people needed to flush SSS cache at one stage
  - A participant submitted pull request for curriculum before end
    of workshop.  Yay collaboration! \o/

- One person complained VM(s) were very slow.  He did not have
  hardware-assisted virtualisation enabled in BIOS.
  - I will update preparation steps to make a note of this


Requested content
-----------------

- Several people asked about sudorules.
  - I will write a module (or maybe Thorsten will!)

- Someone wanted OTP content
  - Assumption is that user has an OTP app, so the module would have
    to be optional
  - but it is a good opportunity to promote FreeOTP!

- Hostgroup dynamic rules
  - Someone wanted to know how these work


Questions that arose during workshop
------------------------------------

- Q: what is the hbacrule evaluation order
  - A: good question.  I didn't know and need to learn about this.

- Q: can you install freeipa-client on IPA server?
  - A: yes, it already is installed as part of server install, and
    IPA server machine is a client of itself.
  - The pupil seemed pleased by this consistent.
  - They were thinking about installing FreeIPA server on their
    laptop just to have OTP for login to machine, etc.

- Group Policy enforcement?
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


Feedback
--------

A form was provided for participants to give feedback about the
workshop.  Response rate was ~50%.  Response summary (PDF) is
attached.  Highlights:

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


Conclusion
----------

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
