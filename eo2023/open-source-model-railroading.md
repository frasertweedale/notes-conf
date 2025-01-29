# DCC-EX Open Source Model Railroading

- Paul Antoine

- Model railway ground rules:
  - not toys
  - not all railroaders are Sheldon Cooper types (some are)
  - you may not say "choo choo" every time you see a train

- Me
  - model railroader since age 10
  - electronics hobbyist since 12
  - programmer since 13
  - now all have collided

- Opportunities for adding realism include
  - running trains automatically
  - switching trains to various track
  - changing semaphore positions on the signals
  - lighting changes
  - sound effects
  - early on much of this was accomplished with mechanical devices
    and basic circuits
    - op-amps, transistors, etc.

- Open Source/Hardware in railroading
  - sharing circuits for animation
  - open hardware the norm for decades
  - microcontrollers have become so cheap, small and powerful
    it makes sense to use them to move trains, change signals,
    lighting and automation
  - arduino naturally lends itself as an accessible platform for
    beginners
  - now model railroaders are sharing code *and* circuits

- Simpler layouts:
  - oval with passing loops, or two ovals etc.

- How locos work
  - motor in chassis, gears drive wheels
  - voltage applied to rails is picked up by wheels
  - microcontroller in chassis allows reading a digital control
    proto off the rails to change speed, turn on lights, or make
    sounds
  - DCC ("digital command and control") is globally the most popular
    control protocol, a US NMRA open standard invented by Lenz in
    Germany.
  - other protocols include DCS, Märklin Digital, Selectrix, TMCC,
    Hornby Zero 1 and many others...

- NMRA DCC potted history
  - designed by Lenz Elektronik of DE in 1980s
  - US NMRA adopted DCC as standard in 1993
  - DCC puts a 12-20V square wave on the rails

- Commercial DCC products
  - DCC an opportunity for manufacturers to add margin to their
    products
  - DCC equipped locos typically 2x price
  - DCC controllers range in price from AUD $230 to well over $1000.
  - vendor lock-in is a thing :(

- DCC-EX Project concept and history
  - make DCC more affordable and accessible to modelers through
    Arduino platform
  - overcome limitation of commercial DCC designs, esp low-end ones
  - innovate more rapidly
  - leverage mature Arduino platform to provide affordable hardware
    and software base
  - create a supportive and welcoming community of railroader
    electronics an dprogramming geeks and anyone new to
    microcontoller software and hardware.

 - DCC-EX community and ecosystem
   - code on github, community coordination and support on Discord
   - 7 projects, all GPL-3.0.
   - 15–20 volunteers with various hardware and software skills,
     some >70y
   - contributors in US, UK, SE, FR, AT, IN, AU
   - strong sense of community, real time support, brainstorming and
     planning
   - feels a lot like an online model railway club

- EX-CommandStation development
  - DCC Command Station software
  - based on Arduino C++ platform, using Arduino IDE or
    VSCode+PlatformIO
  - started as extension of DCC++ Base Station, bug fixes, etc
  - rewritten from scratch and now ~20kloc
  - originally built on 8-bit AVR controller Arduino UNO
  - quickly ran out of RAM and moved to MEGO2560 controller with 8KB
    RAM.
  - NMRA DCC protocol compliant
  - Ex-CommandStation replaces commercial DCC controllers, for as
    little as AUD $80 fully built, but is far more capable.

- EX-CommandStation features
  - dual DCC *power districts*
  - programs loco DCC decoders
  - WiFi connectivity for up to 4 wireless throttles
  - extensible IO for sensors, actuators and displays - hundreds of
    IO
  - EX-RAIL simple scripting language for animation and automation
  - integration with other open source products like
    - JMRI
    - Engine Driver on Android
    - WiFi throttles

- EX-CommandStation Development challenges
  - Arduino core platform is mature on 8-bit AVR, actively used by
    millions since 2005
  - 3rd party open source Arduino library quality varies greatly.
    Embedded systems like DCC-EX require high reliability!
  - many oen source projects thrive for a while, then enthusiasm
    wanes and projects get abandoned
  - abandoned projects lead to forking - one is then left wondering
    what new bugs have been added
    - we try very hard to not create abandonware ourselves
  - DCC-EX team tends to reimplement critical components/routines
    from scratch, to compensate.

- EX-CommandStation growth challenges
  - being adopted by more users; its features have become appealing
    to a wider range of users
  - design team is thiking of and being prompted to develop more
    features
  - more complex features will require more RAM - and the 8-bit AVR
    platform has nowhere to go
  - DCC-EX dev team needs to look at next generation processors
    - various 32-bit platforms were considered, and prototyped
    - looked at ESP32-WROOM-2, SAMD21, STM32F4xx
    - Arduino core support for these platforms, but vendor
      support varies in quality

- Open Source... legacy issues
  - 25kloc based on Arduino core libs
  - Arduino's AVR arch libs are mature
  - Arduino has moved to Mbed OS to supported 32-bit platforms
  - EX-CommandStation now large enough that porting to MBed OS is a
    substantial task for volunteer team
  - ...

- DCC-EX future
  - team is growing with enthusiastic new devs joining every week
    and building thriving community
  - EX-CommandStation continues to spawn other open source projects
    as part of affordable open source model railroading ecosystem
  - DCC-EX now designing bespoke hardware which will also be open
    source
  - YouTube vloggers like *Little Wicket Railway* have started
    showing their subscribers DCC-EX, bringing new users
  - Users seem excited by low cost of entry, accessibility, existing
    and planned features
  - Release 5.0 imminent will include amazing new feaures

## Questions

- Open hardware projects on the loco side?
  - There are, but it's hard because it must be extremely
    minaturised to fit.  Next level up in engineering.

- Physical layer
  - square wave on the rails, each rail alternates polarity
  - 6(ish) khz
  - designed Lenz 1980
  - people also looking at transporting the DCC wirelessly
  - alternative scheme: "dead rail" - use batteries, no voltage on
    rails.  Voltage on rails is a weak point.  Modern DCC locos have
    "keepalive" using capacitors.

- What is power district?
  - Locos with e.g. sound can consume a lot of power
  - You don't want to put 20A on your rails.  If something goes
    wrong, you will melt things.
  - Our base design puts out 1.5 amps per district.  Same signal,
    separate power suppply.

- Have vendors responded to competition/opportunity from DCC-EX?
  - Mostly not, but one vendor put out a throttle that supports the
    EX wireless throttle protocol.
