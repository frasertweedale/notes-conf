Fixing the Internet of Things
=============================

Alasdair Allan @aallan http://alasdairallan.com/

- desktop is increasingly a niche market
- the world is not full of people like us
- for rest of world, the next big thing is... things
- today's wearables are just the tip of the iceberg
- right now our phones are full of sensors
  - but the black rectangle is going to fade into obscurity
- most people *never wanted* and don't like computers
  - they wanted what computers could do for them

- thermostats, electricty meters
- cars
  - tesla model s
    - has extensive data logging capability
    - rest API
    - a web server with a really really big UPS
  - tesla today and the rest of the manufacturers tomorrow

- we are being flooded with data about even our own bodies
- we are making the world controllable
  - anything that can be actuated or turned on/off
- we are at the "banging rocks together" stage of IoT

- movement away from centrally controlled, top-down systems, to
  individually smart things
- our world is currently filled with sensors but the sensors aren't
  yet connected to anything (except us, who make the decisions)
- we are building an internet of broken things

- most systems today are built around the notion of a remote control
  - with humans are participant in the operation of the system

- the IoT equiv of "pile of remote controls" is an iThing full of
  apps.
- we have become mechanical turks
- the people building these things all have the same (broken) model

- "SOMETHING IS WRONG ON THE INTERNET" (xkcd)


- offering the user choice isn't necessarily the best choice
  - it can be a confusing thing
  - the more buttons and controls you have, the worse your design is
  - every control you have is a design decision you punted on
  - passing design decisions to the user

- if internet-enabling your light bulbs make it harder to use than a
  wall switch, that is not a win
  - except for the person who sold you the light bulb

- the smart washing machine has *one button* to *turn it on*
  - then you put your RFID-enabled clothes in and the *works out
    what to do*
  - *that* is smart
  - IoT should be anticipation based, not reaction based
  - it should *work like magic*, not just "every choice offered"

- we need to build *systems* again

- The IoT has to worry less about the Internet
  - not so much about connecting things to the Internet, but about
    putting interconnectivity *everywhere*

- People building IoT have to stop providing API access only at the
  cloud layer.

- I don't want to control my heating - the whole point of buying a
  "smart thermostat" was to *not have to*.

- We need autodiscovery of things
  - Discovery mechanisms need some "local presence"

- We don't think about the Internet as the "Real World" but that
  will change when your door tells the world you're not at home.

- "The first murder by Internet of Things" will be a thing.

- The smarts can - and probably should - be local.

- Need a three-layer solution
  - Bottom layer: discovery, speaking to things in their own
    protocol
  - Middle: abstract data model; make light bulbs look like light
    bulbs, locks look like locks
  - Top: control logic

- *The Thing System*
  - MIT license
  - proof of concept system
