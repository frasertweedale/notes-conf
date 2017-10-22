Work by *Threat Intelligence*
threatintelligence.com
https://ruxcon.org.au/speakers/#Ty%20Miller%20&%20Paul%20Kalinin

- fundamental flaw in way that nearly every org deploys AD
- impacts ability to contain sec breaches
- turn AD into internal C2
- post-exploitation attack
- remote control and exfil via AD botnet

- threat actors are winning
- huge financial motivation (ransomware ~$1bn p/a)

top primary attack techniques:

- phishing (email 73% and drive-by download 13%)
- C2 (hacking)
- C2 (malware)

attack capabilities

- attackers are still trying to be stealthy (but for how long?)
- Open Source tech enables sophisticated attacks
- cloud platforms extend internal networks, introduce weaknesses,
  remove visibility of threats
- potential for attacks to turn noisy for Fast Escalation and Large
  Impact attacks
- harder to recover from; increased revenue stream for attackers

current state of play

- highly motivated threat actors
- utilising endpoint exploitation techniques
- that connect to C2
- to launch fast & effective internal attacks
- what is C2 servers can comm with remote attackers using your
  production cloud?
- if the C2 is the core of your network, you can't shut it down.

AD botnet

- AD is central authn and access control point for orgs
- all end-user devices need connectivity to AD
- all or most servers next connectivity to AD
- with Azure (ADFS?) integration, you get remote control

What do you see with AD botnet?

- ADWS (9389/tcp; AD web service), LDAP (389/tcp; Auth, Kerb)

AD standard user attributes

- some user attrs are self-writable, and readable to whole domain

- no login/logout events (leverage existing authenticated session)

- binary and string attributes e.g.
  mSMQSignCertificates, thumbnailPhoto, userCert, etc

- different attributes = different signals / channels
  - e.g. ipPhone = botnet GUID,
    info = control channel,
    hostPostalAddress = command output,
    mSMQSignCertificates = file transfer


Bot registration:

- botnet GUID = identify / find botnet members
- <username>:<hostname>:<bot-state>:<dstuser>:<dsthost>:<command-id>:<cmd>

Running commands:

- regular execution, plus an "interactive shell" loop mode

Remote C2:

- *GraphAPI*: Azure cloud-hosted API into your AD
- interact with AD from internet without any standard user acct!
- Azure AD doesn't sync attrs back to your on-premise AD... does it?
- Your on-prem AD *does* sync attrs out to Azure AD; exfil!

- So AD botnet has *Xfiltrate Data* feature

- Uses *saveuseraccount* feature to do exfil (more attributes can be
  used ; ~800!).  Otherwise you have to wait for sync on only a
  handful of user-writable attributes

Other options:

- AD Botnet Bind TCP Handler
  - tunnel shell through AD to internal bot

- Reverse TCP handler
  - connect out to a system on the internet
  - metasploit!

Live demo!

Mitigation:

- separate domain into different forests
- noticing odd values in fields...
- monitoring regular changes of "personal information" attrs
  - this is not logged!  you can just hammer away
- lock down permissions - most people in most orgs don't need to
  self-write many or any of their attributues

credits/references:

- @harmj0y

Questions:

- is there a command broadcast feature?
- poll or persistent search / syncrepl?
  - A: poll, keeps a list of bots so it does not poll entire DIT
- BiDi comms over GraphAPI?
  - there are "write-back" restrictions with user attrs
  - you can do it with DirSync
  - a bunch of exchange attrs sync back
