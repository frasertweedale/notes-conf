Programming in the Large - Mark Hibberd
=======================================

  "simplicity is prerequisite for reliability" - Dijkstra

Legacy systems and organisations:

- hand-me-down situation
- the rush-job
- the rewrite
- the greenfield (post shipping)
- the prototype
- the bandwagon
- *legacy is the default*
  - everyone has to deal with it
  - it exists, and that's a good thing

Taking responsibility:

Programmer myth #1: "it's someone else's fault"

- believe these -> remain hostage to fear of change
- Real systems have autonomy
  - independently deployable
  - stand on their own
- we need rules, not boxes
  - foundational concepts of system / domain arch
  - rules for how systems interact
  - rules for how systems are implemented
    - consistency in deployment etc. (helps avoid chaos)
- Exmaple: code search:
  - indexer app
  - code search
- is system autonomous?
  - can I dpeloy them independently?
  - do they have independent domain models (no overlap)?

- standards for interchange format
- no shared state
- autonomy builds in reliability
  - e.g. indexer crashes, search stays up.
- autonomy gives ability to change
  - easy to replace a component

- "how long does it take to get a one-line change to production"?

Warning signs:

- database replication
- autonomy is far more important for reliability than code
  improvements.
  - bad code isn't always to blame

System for evolution:

- thinking ahead is not about avoiding change
  - it's about changing at different rates for different problems
  - avoid short-term decisions having long-term effects

Programmer myth #3: *we must do something now*
Programmer myth #3: *we should rewrite*

How do we not rewrite?

- architecture is controlled by developers, not architects
  - architecture is an everyday task
  - you have responsibility, but also the power
- version everything.  really, *everything*
  - components, communication processes, APIs
- version is not enough - specify how to handle unrecognised
  versions
- The wedge. Make it easier for people to come along and do the
  right thing, and remove the wrong thing.
  - exmaple: "external <thing> support"
- Partial moves
  - control in-progress moves at a single stop
  - track and cap the number of moves in progress
  - plan for rollback as much as rollforward
- validate as you go

Experimentation and measurement

- change without fear
- confidence stems from knowing that changes work *in production*
  before they will affect customers
  - production-quality data
  - automated tests
  - move development to production
    - I want you to ship your worst, un-tried, experimental code to
      production

Programmer myth #5: *We can't ship that*

- we can always ship the code; we just need different levels of
  safety
- machine-level ACLs
- checkpoints
  - "testing in production"
  - examples: bugs we've hit before, business rules, etc.
  - deep implementation, intra- and intra-process cross-checks
- tandem deployments - old and new running side-by-side with
  checkpoints.
- measure *everything*
  - if you can think of it, measure it
  - traceability back to code and data that produced it
- statistics work
  - measurements over time will find errors

Programmer myth #6: *but we can't do that in our situation*

- these techniques *adapt*

*Change is the default*

- we have to embrace change
- we have the power to do it
  - thinking ahead is important
- experiment for reliability
- measure always
  - and learn some statistics
