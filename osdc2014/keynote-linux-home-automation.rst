Linux-based monitoring and control in a sustainable house
=========================================================

Richard Keech:

- redhatter for 10yrs
- first RHCE in world

The talk:

- intersection of tech | creative | sustainability

- 2003: Michael Tiemann (founder Cygnus; original author g++)
  - net gateway
  - control
  - energy mgmt
  - security
  - media / entertainment

- "oh-shit" moments re climate change
- left red hat 2009 and did M.Environment in Energy Efficiency
- now "Energy Efficiency and Renewable Energy Consultant"

Achievemnts

- net zero emission sustainable home retrofit
- important role in important eNGO
- lead author of landmark report on sustainable buildings
- monitoring and control system

Home monitoring system

- temp: seven sensors + BoM
- power: eight sensors + smart meter comms
- water?

Smart meters:

- rolled out in VIC
- connection from meter to power distributor (typically wireless)
  - *Advanced Metering Infrastructure* (AMI)
- *Home Area Network* (ZigBee)

Smart meter data:

- time series; ~30mins
  - consumption & generation
- avail. to cust via web portal
  - graphical / interactive / download CSV

ZigBee HAN

- 2.4GHz ISM band
- 250kbit/s
- In-home display (IHD)
- Reading authoritative info direct from meter
- no CT (current transformer) required
- near real-time data
  ~15-45s old
- also shows solar exports
- messaging / command capability

ZigBee device connection

- Encrypted (128-bit ECC)
- Protection from snooping etc
- Initial device "binding"
  - involves MAC addr and device-specific key / installation code

Linux and ZigBee

- "Raven" ZigBee adapter
- USB device; uses ftdi driver
- XML-based command protocol

Linux and 1-wire
- adapter: DS9490R
  - temp, humidity, moisture, counters, relay control
- ``owfs`` (1-wire file system)
- reliability issued when use through USB hub
- bus length: 10s of metres; tolerates some branching
- multiple adapters are OK
- >14 devices OK - all from USB power

Reading power:

- Hobbyboards pulse counter
- DIN-rail sub-meter module with pulse output
- 1Wh or 10Wh pulses
- direct read pulse count
- infer kWh based on calibration reading

Reading water:

- pulse count from water meter
- pulse probe supported by most meters (in Melb anyway)
  - buy from Reese hardware
- 1 pulse per 0.5L or 5L

Controlling power:

- 1-wire I/O port - relay - 12V control signal
- 240V DIN-rail contactor (12V control)

Controller system:

- Previously: Raspberry Pi w/ Pidora
  - Smaller community - critical mass?
  - Stability issues w/ USB (esp via hub)
  - limited capacity
- Now: NUC with Fedora
  - 3 on-board USB -> no hub needed

On the web and mobile:

- OpenHAB (Home Automation Bus)
- cross-platform, java based
- OSGI pluggable arch
- Open Source
- Broad rance of interfaces
- Monitoring and control

OpenHAB setup:

- mobile-friendly setup
- Android and iOS apps
- Static IP
- Port-forward from router to NUC port 8443
- Split DNS zone
- SSL + password

Future:

- Hot water system control
  - smater algo using weather and human factors
- Notification system
- Internet; device monitoring and control

Questions:

- (Fraser) how much CapEx?
  - ~ 65k
  - now 25k ahead on energy spend during this period of use
  - payback in another 5-7yrs

- (Clinton) what was decision behind getting rid of gas?
  - proportion of gas naturally leaks
  - everything that gas can do, electricity can do better IMO.

