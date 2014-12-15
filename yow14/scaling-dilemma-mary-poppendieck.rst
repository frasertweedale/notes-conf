The Scaling Dilemma - Mary Poppendieck
======================================

How do we scale agile?

- how do we convince execs to invest in agile?
- how do we ensure teams work on the right stuff?
- how does agile work with a big legacy code base?
- how do waterfall and agile teams work together?
- what is the role of managers?
- These are not the right questions!
  - doing agile/lean does not have any value in itself.

The right questions:

- What if we used the same chip for all printers?
- How can we integrate early and often?
- How can we create code that is *always releasable*?
  - That's the only way we know we got anything done.
- How can we re-engineer our planning process to do it with minimal
  investment?
- These led to:
  - simplified architecture
  - rapid feedback to developers
  - increased marketing flexibility


Theory of constraints:

- every system has a bottleneck; if you want to go faster you *have*
  to attack the bottleneck.  Nothing else will work.
- In software development:
  1. The integration problem.  % of release cycle spent "hardening"
       (typically ~30%; up to 50%).  Top companies today: "what's a
       release cycle?"
  2. Deciding what to build. ~64% of software built is rarely/never
       used, but adds complexity.
  3. Getting teams to work together.
  4. The time, energy and initiative of bring, creative people.
- Solve for (these are the scaling problems):
  1. System complexity
  2. Organisation mindset
  3. Multi-team cooperation
  4. challenging work.

For complex systems:

- Big bang does not work.  "Poke, poke, poke" is the only thing that
  works.
- CI/CD *on trunk*
  - keep large code bases stable
  - make experiments easy to run
  - give rapid feedback to developers
  - reduce cost of finding and fixing problems
  - increase business flexibility and responsiveness
- Branches are inventory.  Inventory (sitting around) is evil.
- Team that release together must work together.
- staging pipeline requires new thinking and tools
- keep trunk production ready as top priority (above new features)

Organisational mindset:

- How do we decide what to do?
- scaling is an organisational problem.
- two different org mindsets:
  - IT mindset
    - "the business"
    - order-taking dev team
    - success = cost, schedule, scope.
    - tough tradeoffs made during planning process
  - product mindset
    - entrepeneurial leader
    - responsible engineering team
    - success = delighted customers
    - touch tradeoffs made based on market realities
  - in a competitive environment, which mindset will win?
    - product mindset will.
- shared code base? -> one team
- teams too large? -> break dependencies
- shared system test? -> one group
- groups too large? -> change the architecture
- use capacity allocation for portfolio management
  - divide capacity and get things done now
  - parallel pipelines cf one large, linear pipeline.
- replace governance with product management

Multi-team cooperation:

- cooperation requires accommodation
- Accommodation has a cost
- If cost of accom. is borne by one party, resentment arises and
  cooperation fails.
- Monopolies (by definition) don't have to accommodate:
  - Monopolies destroy cooperation
  - Examples:
    - Departments everyone loves to hate
    - "Autonomous" teams
- Book: *Six Simple Rules: How to Manage Complexity without Getting
  Complicated* - Yves Morieux & Peter Tollman
- Cooperating teams:
  - Getting teams to work together has a much higher impact than
    just getting them to work.
  - People on a team must have a shared goal.
  - *Groups of teams* must have a shared goal.
  - Everybody must be invested in helping everyone else turn out
    their best work.
- Shared responsibility:
  - Who is responsible for delivering value?
    - Business? Product owner? Other teams?
    - *We work together*
    - *Nobody suceeds unless everyone succeeds*
  - The military model.
    - Squad leaders need to know command intent two-levels up.
    - Maintain situational awareness one level up.
    - Command intent = a concise expression of the purpose of the
      campaign, the desired results, and the expected team
      progress toward achieving the desired end state.
    - Collaborative planning.
    - Situational awareness of the progress of other
      squads/platoons

Challenging work:

- change "delivery teams" to "problem-solving teams".
- impact-driven development.  start from impact and work backwards.
- start with *why* - purpose, problem.  understand desired impact:
  - who ares about the impact of potential solutions
  - how will these people measure the impact of outcomes?
  - ...

- Gov.UK case study.
