When Your Codebase Is Nearly Old Enough To Vote
===============================================

- Denise Paolucci
- Dreamwidth Studios - www.dreamwidth.org
- Slides: www.slideshare.net/dreamwidth

Dreamwidth:

- fork of LiveJournal
- Perl 5.6
- Apache 1.3 (released in 1998, still using in 2008!)
- BML (custom template language from 1996)
- Various Perl modules (usually outdated)

Here be dragons
----------------

- I could have made this whole talk, "weird things we found in our
  codebase so you can laugh at us."

- comments that are totally unhelpful
  - "read more <url>" to long-deceased, non-archived sites

- workarounds for oudated browsers
  - "fixed a bug in opera 8; caused a bug in ie6"

- really old HTML
  - upper-case tags
  - 1999 called, they want their table-based layout back.

- out of date special-casing
  - catch misspellings of "hotmail.com" and "aol.com"
  - meanwhile, 80-90% of new signups *today* are from gmail.

- old bad decisions (that were right at the time)
  - workarounds for unicode bugs in old versions of Perl
  - even today, LiveJournal is the free press in Russia

- and more!
  - outdated modules
  - ancient JavaScript; in 1999 there was no jQuery or shlibs
  - massive duplication
  - bugs that had turned into features

Should you rewrite?
-------------------

- There is no algo for deciding "yes" or "no"
- There are a lot of pros and cons

Decision examples:

- Should we upgrade apache?
  - Benefits:
    - move away from software at EOL
    - security fixes
    - not having to downgrade new installs
    - easier to explain
    - no longer horribly ashamed of ourselves
  - Cost:
    - so much time (~6mths to working prototype)
    - new and exciting bugs
  - Decision: a no-brainer

- Switch to *Foundation* framework?
  - Benefits:
    - modern framework
    - responsive design (better mobile)
    - better cross-browser
    - easier to make accessible
    - well documented
    - kill lurking old HTML
  - Cost:
    - so many pages
    - change to look/feel (our users are picky)
      - they will notice a 5px difference and complain
      - a lot of advance warning of coming changes so users don't
        freak out
    - new and exciting bugs
    - less individual
    - no, seriously, SO MUCH WORK
  - Decision: went ahead, but we're still not done
    - One FT employee, 9 months in, about half-way there


Rewriting pros; be rewriting will you...

- become more compatible with standards / best practices?

- make your code easier to install?

- eliminate project-specific systems?
  - concerns that are not the core concern of your project?
  - things that were written before a viable alternative emerged?

- reduce reliance on institutional memory?
  - it's a great thing to have, but a bad thing to rely on

- benefit your users or your team?
  - get you to (or closer to) a feature your users want
  - remove things that people hate touching


Rewriting cons; be rewriting will you...

- lose a long history of bug or security fixes?
  - second system syndrome
  - you *will* (re)introduce bugs

- tie up too many of your people, or involve too much retraining?

- be tied to something that might not last?

- need to fork / adapt a standard / module?

- not benefit yoru users or your team?


The number one question: **are you going to finish it?**


Future proofing
---------------

- comment everything, in the code itself

- be grep friendly ("FIXME" etc; be consistent)

- write task lists for your future self
  - software development is a series of compromises
  - so put reminders of the compromises you made on your calendar,
    and actually address them

- regularly install / compile from scratch, on a clean system
