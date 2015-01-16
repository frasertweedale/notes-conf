============================================================
Incorporating the Security Development Lifecycle and Static
Code Analysis into Our Everyday Development Lives:
An Overview of THeory and Techniques
============================================================

Goal: give quick overview of secure development and available SCA
tools and ohpefully get you thinking about ways to improve security
on your project

Resources:

- OWASP
- *Secure Programming with Static Analysis* - Chess & West
- SDL: A process for Developing Demonstrably More Secure Software -
  Howard and Lipner
- Coverity Open Source Scan

Traditional assumption about open source are not working:

- "many eyes make all bugs shallow"
- everyone can review, but is anyone?
- old code is stable?
- firewall/IDS will protect us
- component based software dev is more secure
- using frameworks is more secure

Who is to blame and who can help us fix it?

Failure to educate.

How can we fight back?

- **Security Oriented Developer Education**
- use Security Development Lifecycle
- Organise strong proj sec teams and be ready for incident
  response
- Make software updates as easy as possible
- encourage security in everything we do

Who is examining the security of open source code?

- bad guys, security researchers, state actors, academia

Who else should?

- companies looking to leverage Open Source need to do their own
  code audits
- companies that are making $ from leveraging Open source have an
  obligation to devoce resources to security (IMO)
- companies that offshore internal development (May be a
  requirement to sell to certain governments)

Difficulties in controlling security in open source projects:

- inherent, obvious difficulties in working with projects that
  anyone can contribute to
- different size projects mean different things
- [more: check slides]

Trends helping and hurting

- frameworks/APIs help to remove common security bugs through
  abstraction
- security researches working with community to get things fixed
- tools like static code analysis

Other trends:

- identiy issues by performing security assessment of applications
  after they are developed and then fix these issues
- race condition: need to get software out door quickly. Security
  features might be overlooked, "later release"

Advanced Persistent Threat:

- APT successfully compromises any target it desires.  Conventional
  defenses do not work.
- Patient and methodical

Interesting finding from Coverity report on open source:

- defect density is .59 (.72 for proprietary)

Thread modelling:

- STRIKE and DREAD (Thread ranking) from Microsoft are good.

Testing:

- a golden opportunity to catch security bugs
- maybe the only opportunity
- don't just validate functionaliity with unit tests; look at
  security too

What about agile?

- makes good security-oriented design harder
- key is to have developers using good tools like SCA and for models
  to be updated periodically as the application evolves

Static code analysis

- uses heuristics, coding principles, pattern matching and other
  tricks to try to identity security bugs in code
- process of assessing code without executing it
- augments good defensive programming techniques
- When?  IDE integration, build report, CI, pre-release check.
- Limitations
  - Can only find problems in code, not architecture
  - Some things are not obvious
