Stock Hacks
===========

Nik Cubrilovic @nikcub

Applying open source intelligence (OSINT) and information leakage in
an event-based trading strategy.

Event-based trading:

- apple report drop in sales revenue = event
  - caused 8.3% drop in price
- we know when announcements are going to drop
- trade based on what we think the stock is going to do around an
  event
- also: earnings/revenue (guidance) hit/miss/exceed

- margin trading: e.g. CMC provides it
  - 8.3% can become 166%

- what if you *knew* iPhone sales were down?
  - a lot of analysts and hedge funds trade on predictions
  - you can't move 55m iPhones without trucks, ships, etc
  - everything that happens leaves a "wake"

- meta-research
  - "overall IT spending up/down X%"

- dedicated research
  - send people into stores all over the place, mark down serial
    numbers, work out how many being manufactured
  - talk to staff: "how are sales going?"

- high-tech event-based trading
  - WSJ *Trades Seek an Edge With High-Tech Snooping*
  - a lot of funds are very secretive

- Genscape
  - You buy their reports for six-figures per year.
  - Track commodities
  - 80-90% of oil passes thru Oklahoma
  - Fly helicopters with IR cameras over site to work out how much
    oil is there.
  - Between July and Nov, Genscape reports predicted the direction
    of every change in EIA report, accurate to 0.1%
  - Customers trading on this info made a killing
- UBS
  - Walmart
  - *UBS proprietary satellite parking lot fill rate analysis points
    to an intersting cadence in intra-quarter an dpotential upside*
  - purchased satellite imagery for all walmart stores
  - count the cars every day, monitor reduction or spike
  - doing it for many retailers now
- Second Measure
  - Partnered with bank (name not disclosed)
  - Anonymised Credit Card xn data
  - report on retail traffic based on CC xns at all US retailers
  - Chipotle E. Coli outbreak... Second Measure nailed the report.
    - And they knew where customers were going instead!
- Foursquare checkin data
  - use for retail traffic
  - originally was being scraped; now 4sq sell the data
  - see also Facebook et al
- Sino-Forest Corporation
  - IPO on Canada stock exchange
  - prospectus claimed they were largest forestry company in CN
    - specific figures of land managed
  - investors were falling over each other to buy in
  - Muddy Waters Research shorted them
  - figured out that at claimed rate of logging, they would have
    logged out their land in a couple years
    - and there weren't enough trucks to move the wood
  - paid people to get research on the ground, bought satellite imagery
  - found out they owned *nothing*!

Open source intelligence

- any intel you can get on target from public sources

Apply these techniques to internet companies?

- First step in most pen testing standards
- passive / active
- passive example:
  - finding users of a service?
  - find customers of Bank W?
  - observe effects of phishing campaign on SocMed support services
    - extrapolate to active users
    - compare to competitors
  - statistical estimates (not advanced math)
- active example:
  - how to find hard numbers?
  - german tank problem
  - ``/user/99``
  - autoincrement is harmful
  - recommendation is to obfuscate or high internal structure
  - UUIDs?
    - not necessarily random!

Information leakage

- disclosure if information that describes a system
- meta-metadata
- example: server headers reveal vendors
- example: leak existance of username
  - during signup, or failed login
  - enumerate usernames
  - sample known usernames e.g. linkedin usernames
  - see how many hits you get and correlate
- determine how popular service is based on whether "preferred
  usernames" are taken


Putting it all together; examples:

- passive OSINT can give you plenty of info to trade on
- DigitalOcean
  - knew from ping sweeps and net sweeps that they were growing fast
- User number enumeration
  - Adobe: transition from desktop software to Creative Cloud
  - watch one number: no. of new Creative Cloud customers
  - see how well it tracks targets

Paper will be out soon.

Ideas:

- work out how often / how eagerly an org deploys updates?
