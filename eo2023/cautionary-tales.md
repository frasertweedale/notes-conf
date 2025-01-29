- Paul McKenney - Linux Kernel engineer

- "Be careful what you wish for.  You might get it."

- Anecodes of solving the wrong problem
- Happens when you fail to *live among your users*

- 1980s: 8-bit machine (to save money cf 16-bit processors)
  - wrote a CRM, to spec
  - customer hated it, and went out of business
  - the engineer developing it had no access to the users
  - failed to understand that I was competing with a filing cabinet.
    The filing cabinet won.

- 1970s: Student Housing System
  - figure out which student goes to which dorm, and how much to
    charge them
  - punch cards and Fortran (later COBOL)
  - pro-rata if students arrive part way through term
  - mistake: months have different lengths
  - solution: "jdate" algorithm
  - good idea implemented poorly

- 1990s: clustered database systems
  - shared disks for availability win
  - failover required when a system goes down
  - have to test it; but customers didn't like that (testing in
    prod)
  - no such thing as chaos monkey yet
  - took a long time to reproduce the issue (between 5 to 27 hours)
  - cause: memory was unaligned (intentionally, to save memory), but
    the compiler assumed otherwise
  - `volatile` is your friend

- 1970s: 
  - computer dating program for National Honor Society fundraiser
  - Students' paper questionnaires transcribed to paper tape, then
    read into program.  Hamming distance matching.
  - One dissatisfied customer: senior girl matched only with frehmen
    boys.
  - Program looked correct.  Was correct!  It was a data entry
    error.
  - Good idea, implemented properly.  Correct code is not enough.
    The environment and processes matter.
  - It was a mistake to entrust the data entry to the freshman boys.
  - *"A lot of success in life and business comes from knowing what
    you want to avoid*" - Charlie Munger

- 2004: Real-Time Linux
  - IBM got a lot a requests for "enterprise-grade real-time Linux"
  - No such thing existed.  No bid.
  - But multicore was dawning.  Can dedicate CPUs to real-time code.
  - Migrate processes to non-RT-dedicated CPUs when they make a
    syscall.
  - Tested patch, works great.  No more need for no-bid.
  - Customer rejected the idea :(  Never found out why (lack of
    security clearance).
  - Nevertheless, a career highlight.
  - When a nice idea collides with reality, **reality wins**

- Formal Verification
  - A million-year bug?  If you have 20 billion installations,
    that's several times per hour.
  - What about when the risks are very high (e.g. people die).  What
    risk is acceptable?
  - Full state-space search is easy and attractive, when feasible
  - Verification still valid after bug fixes?
  - FV is *expensive*.
  - How to verify the verifiers?  De-risk via one-way bet.

- Natural Selection
  - Darwin didn't realise it, but he was studying natural selection
    in software.
  - But... we can get bugs adapted to the verification.
  - Don't just fix bugs in software; fix bugs in validation
  - Validate only intended use cases... new use cases → new bugs
    - Free/open source software can help
  - If you only ever fix the bugs that affect customers, you can end
    up in an evolutionary dead end.

- "Natural selection" is a euphamism
  - what happens to the organism that is not "selected"?
  - If your tests are not failing, they are not improving your
    software
 
- Why would users fail to complain?
  - Don't actually use the software (common)
  - Don't know who to complain to
  - the last N times they complained nothing happened
  - Your software is successful (→ faded into woodwork)

- "Customers don't know what they want until we've shown them" -
  Steve Jobs
  - You must live among your users
  - You must complain on their behalf

- Summary
  - Users don't know what they want
  - For sofwtare developers, that's no excuse
  - You have only failed if you have given up; until then it's
    called learning.
  - You are not a failure until you start blaming others for your
    mistakes - John Wooden
