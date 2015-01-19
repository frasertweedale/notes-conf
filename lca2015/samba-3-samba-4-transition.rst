Pushing users into the pit of success: Stories from the Samba 3 -> Samba 4 transision
=====================================================================================

Andrew Bartlett

- Catalyst, Wellington NZ
- Samba team member since 2001

Samba AD DC features:

- LDAP
- Kerberos (Heimdal)
- Windows DC
- Centralised IdM
  - Authentication
  - Authorisation
- SMB / SMB2 / CIFS
- Windows machines join AD natively

Pushing users into the pit of success means initial install is a
success even if:

- software is complex
- protocols are complex
- needs of every site are different

Success means security:

- password expiration and complexity requirement by default
- admin shouldn't choose machine keys
- replication should be authenticated and encrypted

Success means managed complexity:

- don't shy away from complex protocols like Kerberos
- hiding details by making things "just work"
- making complex software simple to operate
- not expecting administrator to be an expert

Sales pitch:

- Samba AD DC has solved *some* of these problems well
  - At the expense of performance and some flexibility
- High praise also for FreeIPA
  - Many of the same patterns are there also
  - Different products, close communities
  - Can't handle windows clients directly

What we have done:

- Changed Samba DC from "choose your own wiki adventure" to
  consistent, reproducible pattern

- Added sensible and strictly-defined constraints.
  - e.g. "you can't have same the same username twice"

- Changed security from optional and after-the-fact to on by default

- Made replication simpler to configure and secure by default

Integration:

- Somebody else's problem?
- OpenLDAP is "just" a data store.
- Samba uses externally managed LDAP store
- Lots of tools and module you can use, but none installed or
  running by default.
- We can do better.

How bad is it really?

- Can't smart administrators install software, follow wiki,
  customise the software for their own org?

- Can't they create a secure, reliable, fully-featured IdM?
  - Without great stress and inconvience?

- NO, they can't.  The smartest folks have trouble getting this
  all to come together well.


Migration success for users:

- Strongly encouraged testing on independent network
- Many sites migrated; some large
- Glad to be able to use modern Windows (7, 8) OOTB

Lessons:

- attitude change was key
- from "kit of parts" to product
- admins still pushed off the cliff at the edge of support


Beyond Samba, Beyond Windows?

- FreeIPA (based on 389 DS)
  - Installed in a trial at Catalyst recently
  - Asks a lot of the same questions as Samba AD DC
  - Kerberos, LDAP, web GUI
  - Was impressed

- OpenLDAP could still do the same
  - The parts are available if you want to build a non-AD Active
    Directory
  - Samba has already got some of the code


Status report (AD DC)

- Bad password lockout

- Now uses common ``winbindd``

- Finished ``smb.conf`` merge (no longer conflicting ``loadparm``
  tools)

TODO:

- inter-forst trusts
  - recent work on trusts to FreeIPA quite successful

- subdomain support

- experimental effort to (again) use OpenLDAP, this time
  autoconfigured

- POSIX integration
