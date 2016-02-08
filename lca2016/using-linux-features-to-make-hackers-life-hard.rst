Using Linux features to make a hacker's life hard
=================================================

Kayne Naughton, Cosive
@kxyne


- I want to make it *difficult* for bad actors; hopefully they will
  go attack someone else.

- Threat actors are not a force of nature, they are people.  They
  respond to incentives and frustrations.


Phases on an attack
-------------------

1. Reconnaissance

- LinkedIn is good resource for social engineering
- Think about putting fake people on LinkedIn (canaries)

2. Weaponisation

3. Delivery

4. Exploitation

5. Installation

- Getting persistance in environment

6. Command and control

7. Actions on objectives


Six 'D's from US Army information operations doctrine:

- Detect
  - Has no visibility to attacker
- Deny
- Disrupt
- Degrade
  - What can you do to make the information less valuable to the
    attacker?
  - e.g. maybe you don't need to log all the things?
  - Don't keep stuff you don't need
  - Use proper password hashing
- Deceive
- Destroy
  - Remove the opponent from the equation
  - Law enforcement is one vector

Action matrix:

- Grid of "phase" against "(D) action"


A word on "Destroy":

- Need to know whether you're being tactical or stragetic when
  disrupting / destroying
- What do you gain by getting them shut down?
- Attackers outsource (which reduces their risk)


Disrupting reconnaissance:

- Put fake people on LinkedIn with important sounding titles
- Bogus traceroutes can confuse the hell out of attackers
- BCP 38
- Don't let stacktraces etc. through on HTTP 500/40x errors.


Meterpreter:

- A bunch of exploits that are very easy to run
- Look for port 4444 (default port) in your network
- People use the default because few people look for and disrupt it


Mounts and premissions:

- Use unionfs


Containers:

- not intended as security control
- can be an effect measure
- make containers read-only where possible
- use iptables in your containers to avoid people getting out of
  container


Command and control:

- we still see a lot of VNC out there, unfortunately
- ssh, rdp, et al
- look for them in your traffic, slow them down, block them
- this is already a "panic and alert" stage


Actions on objectives:

- Always human factors: fame, cash, revenge


Don't reveal your mechanism:

- SQLmap


Counterintelligence:

- cowie honeypot
- record ssh pubkey from attackers
  - crossreference with e.g. github keys
- give attacker cookies to track / crossreference
- ipaddr "reputation" services are worse than useless
