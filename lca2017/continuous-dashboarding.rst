Continuous Dashboarding - Your DevOps Airbag
============================================

**Christopher Biggs**

Strategy:

- Who your dashboards are for
- What to monitor
- When to build dashboards

Tactics:

- How to get there
- Where to do them?
- With (what tools)

Who:

- traditionally, ops
- monitoring focused on services and servers, rather than software
  domain
- little feedback goes upstream
- devs are not being well served
  - they don't care about disk IOPS
  - is the app working?  did I make it better?  did I help the
    business?
  - regression test your bottom line!
- learn it while you're still in business!
- if you let your customers tell you about faults, it's too late
  - 1% call, 99% go somewhere else

- management want reassurance (and to know you deliver)
  - shipped a great new feature?  you have have **hotline** to CEO's
    eyeballs

- data lets you sooth your customers
  - give customers a weather report
  - give sympathy along with realistic expectations
  - people appreciate honesty
  - head off customer calls before they happen

  - turn act of supporting customer from palm reading to brain
    scanning.  observe what affected the customer

  - customers are not (often) reliable witnesses
    - record as much as you can.

What?

- traditionallys:
  - system load
  - rates of key events (pageloads, signups, checkouts etc)
  - service status
  - error alerts ("pages")

- now: patterns and trends
  - business goals: rates plus trends
  - unavoidable errors: look for out-of-range levels
    - eg: CC processing errors; can't find template
  - location and traffic patterns; browser/platform demographics
    - if you notice sudden shift in demographics, watch out!
  - UX metrics.  Record sessions.  Make developers watch them.  They
    will learn something.
  - health of 3rd party interfaces
  - app and product reviews (iTunes, Google, Yelp, Amazon), socmed
    - most customers don't bother calling support: they give you 1
      star and say your app's crap
    - there are tools to ingest and analyse customer feedback

When?

- early and often!
- make sure *whole team* knows lay of the land
- concentrate effort where value is
- testing before release tests against a fixed set of conditions.
  once you ship, conditions are constantly changing.  so you need to
  keep testing!
- look for trends in performance and responsiveness over time
- the act of having to explain yourself leads to more thorough
  thought.  dashboard = "how do I know if feature is working?"

How?

- Love your data as much as your code
- Bring silos together
- Visual dataflow tools like Node-RED (nodered.org)
- You can build a dashboard to answer a particular question in
  ~30min
- Orchestratoin
- Devs must have access to BI stack
  - And should ship dashboards alongside their features
  - make them think about what data they should emit
- Make dashboards an artifact
- Be proud of them, demonstrate in product showcases
- in QA: dashboards are first class products

Where?

- In development pipeline
- code-review your dashboards
- pretty-print JSON or XML data
- use Visual diffing tools to highlight changes
- performance testing - daily/sprintly test
- introduce failures to confirm that monitoring works
- break things to see if *anyone* notices (if not, write a
  dashboard!)
- automate dashboard deployment

With? (tools)

- ELK, splunks, etc
  - turn logfiles into easy/fast-to-query data lake
  - start now and look at what patterns emerge
  - target your logging to facilitate analysis
- ELK
  - build dashboard, export config and commit it
- Node-RED
  - visual system for Wiring the Internet
  - pull data from just about anywhere (plugins)
  - originated at IBM; now under JS foundation
  - create flows using drag+drop
- Blynk (for mobile apps)
- Socmed integration
  - Node-RED has good support
  - receive feeds from app stores
  - post to slack; monitor channels
  - Watson for better seniment analysis
  - Send poor reviews / feedback to support for follow-up

Summary

- Who is data for?  Make it palatable
- Think of data beyond graphs, and inform software design
- Teach dashboards like code, review, version, demo ship
- Focus on business goals: the big picture!
