SubPos - A "Dataless" Wi-Fi Positioning System

Blair Wyatt (@BleckyWhat)
http://www.subpos.org/

What is SubPos?

- SubPos is an open source Wi-Fi positioning system usable where you
  don't have GPS.

- initial concept came about during caving expedition

- was created to fill gap in market for open source indoor
  positioning

- no longer have to send your position data to $MEGACORP

- How can you enable GPS like positioning in subterranean env?

- SubPos based on Microsoft research paper

- Don't have to associate to send position (locations encoded in
  beacon-frame based)

- Beacon frames must be sent at min rate of 1mbps and 802.11b

- SubPos is standard for xmit pos info via Wifi for use in client side
  trilateration.

What SubPos isn't:

- Not used for client tracking (would need client-side app to send
  that info)

- Isn't a calibrated system for use where there are tight
  positioning constraints


Problems / limitations

- some embedded platforms only allow 31 octets of data in SSID
  (which is not standards compliant)

- Android API returns SSID as unicode string
  - it is supposed to be interpreted as octet string
  - there is a workaround

- Some access points don't allow certain characters
  - e.g. OpenWRT doesn't allow null bytes or space

- Coding masks
  - why not just base64/other encoding?
    - too much overhead
    - some resulting octets may still cause problems, e.g. '+'


Android API

- easy to integrate into existing positioning apps
- trilateration takes about 20ms


SubPos Node

- Dedicated AP device for wifi positioning
- Accuracy: +/- 0.5m
  - hope to improve with time-of-flight factored into calculations
- Frequency hopping
- Low-cost; you could retail them for > AUD 20


What doesn't work?

- iOS Wifi scanning API is private framework and apps that use it
  are not allwed in Apple App Store
  - Enterprise distribution can use private APIs though


Future work


- shift away from ESP8266 modules
- close ESP8266 source limits tight beacon control
- pure RF solution will genreate raw beacon frames
- much lower power
- time of flight a possibility

Config mode

- remotely configure SubPos nodes
- nodes scan for config frame

SubPos receiver

- implement time of flight
- highly accurate
- low cost
- connect to drones or robots with NMEA data

Room zoning

- Ability to zone an area to increas accuracy in tight spaces
- ignore nodes not in the client's current zone

Improved positioning API

- dead reckoning for improved path accuracy
- better averaging and trilateration
- error circle (accuracy) indication

Self-positioning learning mode

- access points or nodes could be placed into learning mode,
  enabling them to determine their position from other nodes

Bread-crumb mode

- Use Node like breadcrumbs

- can be used to tag rooms or areas and combined with topological
  maps to provide course positioning data

Why SubPos?

- designed to be backwards compatible with vast majority of devices
- fast acquisition time (~2 seconds)
- low location latency (client-side trilateration; no connecting to
  server to work out position)
- no data connectivity required, so no giving your position to big
  data (unless you want to)


Questions:

- data provenance?  authentication?  security?
  - nope.  not much you can do with 31 bytes
  - outliers can be (are) discarded
