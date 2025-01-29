# Incident detection and incident response for open source

- Alistair Chapman, Red Hat


Agenda

- Risks
- Open source difference
- What to watch for (detection)
- Handling incidents (response)
- Controlling the risks (protect)

## Planning

Security for open source - what makes it different?

- blurred lines between public and private
- contributions from untrusted parties
- tooling and infra available to everyone


Your one true enemy:

- `git push`
- so many problems come down to someone pushing something they
  shouldn't have, or not pushing something they should have
- treat `git push` like `sudo`

Incident response process:

- prep
- detection and analysis
- contain and recover
- post-incident activity
- (back to the start)

Planning for the worst:

- sometime YAGNI stands for "you ARE going to need it"
- it's not just about the code
  - docs, demo sites, test infra, ...

Open source devs are usually hesitant to devote effort to proper
risk management.

Risks, what should be worried about?

- supply chain attacks
- malicious package substitution
- infrastructure takeover
- lateral movement / privilege escalation

Reconnaissance:

- your adversaries know basically everything you do
- your project will giave away a lot of your surrounding
  infrastructure
  - build systems
  - bug trackers
  - reporting paths
  - developer tooling
- tl;dr everything you know, they also know

You're in a glass house now

- no silent fixes
  - every commit is a convenient summary of any mistake or
    vulnerability
- no "quick enough"
  - once it's on GitHub, it's gone.  There is no "fast enough"
  - there are *so many bots* scraping GitHub APIs looking for
    keys/secrets.
- No boundaries
  - perimeter?  what perimeter?
  - traditional wisdom won't always apply to your project

Working in the open

- watch your outer loop
- public bug trackers are a one-stop shop vuln list
- data classification will only go so far
- how closely do you watch your build pipeline?

Tuning out the white noise

- Every "cyber researcher" is going to report everything you ever
  commit
  - "Did you know your GitHub/bug tracker/config/test data is public?"
- Having a proper disclosure process will filter a lot of noise.

Protect your infrastructure

- You might know your code well, but how well do you know AWS?

Keep an eye on your environment

- Keys and secrets
  - stop it before it happens and catch it soon after
- Runaway bills
  - attackers don't follow your work schedule
  - focus on automation and quick response
- Service accounts
  - convenience comes at a (non-metaphorical) cost
- PRs
  - sure you trust your maintainers...
  - do you trust S1MY, Chlna6666, ...?
  - maybe gate or don't run build pipelines for 1st time contributors?

Secrets management is hard (apparently)

- use the available tools, even for individuals/small teams

## Detection

Detection: you need to know there's a fire before you can put it out.

Watch what matters

- identify key risks
- use what the platform offers
  - find, understand and use the platform security features.
- make it hard to undo
  - if your alerts can be bypassed, they're not going to help
- read/watch the alerts!
  - don't just send all mail from GitHub to archive.
- make sure YOUR accounts are not the weak link
  - use strong passwords, 2FA, etc


Be wary of shadow IT

- even small projects can be affected
  - larger projects/orgs especially at risk
- if the safe process isn't simple, people will do it the unsafe way
- where you can, provide sane defaults and tooling to make it
  easier


## Incident response

- If Optus couldn't revent it, what do you think your chances are?

Visibility is King

- speed is the key
  - you need to know the moment something happens to your project or
    infrastructure
- automate better than the other guy
  - attackers have automation, you need *better* automation
- log everything, including the logs
  - the more you can piece together, the faster you can undo the damage

Don't panic

- respond with haste, not panic
  - panicked decision can do more damage
- preserve where you can
  - if you want to be able to prevent a repeat, you need to know
    what "it" is and how "it" happened
- plan for hostilities
  - attackers might still be in your infrastructure

Learn from your incidents

- How did it happen?
  - ideally you'd understand every step the attacker took, and why
- How can you catch it?
  - What do you need to watch for?  Do you log it, to see it?
- Can you limit impact?
  - No prevention is perfect.  Limiting impact can help.

Prevention is still better

- catch things before the internet
  - if you're waiting until AWS tells you, it's too late.
  - do local intervention wherever possible
- make security part of the process
  - you probably already automate parts of your contributions.  keep
    going!
- some solutions aren't technical
  - teach the fleshy humans better

What have we learned?

- security is more than just code
- planning ahead will pay off
- document everything, automate what you can
- use tooling and processes to reduce risks
- if do all this and never have to execute your IR plan,
  that's a success

- DO NOT PUSH YOUR API KEYS TO GITHUB
- Do not open SSH to the internet, esp if you use password authn.


## Questions

- How can devs practice IR?
  - Tabletop exercises.  It can be hard when there is a lack of
    experience in formulating the scenarios, but there are good
    resources online.
  - Read others' post-mortems.  Learn from them - what and what not
    to do.

- WFH; if not SSH, then what?
  - tunnels, L2 mesh.
  - update rules when IP address changes
  - SSH on alt port, disable root login

- Testing logging/monitoring/alerting
  - Canaries
  - Make sure you get alerts
  - Replay the steps in previous attacks (against you, or others)
    and make sure it's getting picked up.
