On the Importance of Upstreaming
================================

Robin Sheat, Catalyst IT

- Koha developer
  - library mgmt sytesm
  - used in 1000s of libraries in the world
  - free software
  - dozens of companies
  - hundreds of developers

- Catalyst
  - open source focused company
  - offices in NZ, AU, UK
  - all work with Moodle, Mahara, Drupal, free databases

At a Koha workshop in Malaysia a few years ago:

- people were stuck on very old versions
- they'd made changes to suit their needs
- they no longer had an upgrade path
- only options are:
  - throw away changes and start again
  - port them
  - both are expensive options

Upstreaming:

- getting fixes/improvements into original product
- no just throwing them over the wall
  - never works
- shepherding them through the process
  - might be as simple as writing a patch and submitting PR
  - for larger projects: discussion, signoff, QA, test coverage

Why should you upstream?

- Don't have to maintain own patches anymore
- Losing patches is easy to do
- Let upstream take care of it, if you can
- Contribute to the betterness of the world!
  - Give people something they find useful
  - Maybe they'll improve on it.

Become upstream:

- If it's a core product or integral to what you do...
- You can become part of the core developers
- Help guide the future of the project
  - or make sure decisions that are made do not *adversely impact*
    your concerns
- Become a core developer, release maintainer, etc.

Vanity:

- It's nice to see people using things you made
- Get your name in the project
- CV material
- A chance you're becoming an expert in some area.
- Get your company's name out there
  - Attract staff
  - Attract clients
- Get your clients' names out there
  - "sponsored by Bob's Widgets"

Why shouldn't you upstream?

- It is a lot of work.
  - esp. on big projects
- Matching coding guidelines, test coverage, make it not impact
  other use cases, etc.

Fear:

- Is there some legal liability involved?
- Competitive advantage.
  - It's probably not your core business.
  - Even if it is, you can probably separate the bits that are/are
    not.
  - Trade secrets.
    - Internal rules should be housed internally.
- Are these claims valid?
- Could it be solved with education?

Totally local customisation:

- Can they be generalised?
- Keep the minimum amount of customisation you can get away with.
- Sometimes you can't.

How can you make it easy for contributors?

- Use a public tracker
  - Github, Gitorious, etc
- Keep an eye on issues and pull requests
- Large projects:
  - Have a documented process
    - Guides for developer setup
    - Guides for patch submission/review/acceptance
    - Coding guidelines
  - Help newcomers.
    - Let them bypass parts of the process, e.g. style violations
    - Handhold where you can.
  - Recognise people for their contribution
  - Entice people back by giving them a good experience
