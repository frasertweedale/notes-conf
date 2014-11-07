Before All Else Be Graphed; Katie McLachlan, Anchor Systems
===========================================================

- making graphs manually doesn't scale

- streams of data coming in all the time?  don't have the resources
  to graph that manually.

- machiavelli, a graphing and visualisation tool for metrics
  github:anchor/machiavelli


Data sources and stores:

- Stores hold data, e.g. json feed, flat file, db
- Sources: logical contextual representation of data
- separate entities in machiavelli
- configuration: a single yml file for defining sources and stores.

Visualisations:

- a separate javascript lib for graphing data visualisations
  - Clizia
- supports various graph representations:
  - standard time-series graph
  - stacked graphs.
  - tealeaves
  - horizon graph

Source logic:

- can test and add more attributes, e.g. colourisation

Stateless dashboard:

- dashboards of favourite servers, most vulnerable processes, etc.
- search
