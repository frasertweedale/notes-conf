``/sys/class/gpio/Parking Brake``
=================================

Joel Stanley

Mazda CX-5

- MZD Connect in vehicle entertainment unit
- WiFi (for life traffic updates)
- USB ports on the front

Don't brick your car!

- once associated with WiFi... portscan
- open on port 22
  - had to downgrade the ciphers >_<
  - no obvious "admin/admin" account
  - brute forced 3-char passwords
    - I was in!

Specs:

- Linux 3.0.35, armv7l
- NXP (Freescale) i.MX6

Intersting things:

- gstreamer application for overlaying pattern on reversing camera
- bunch of lua apps for audio control
- OpenCar SDK
- openssl 0.9.8k (post-heartbleed, but still horribly out of date)
- terminal app (that can't be killed...?)
- the car has a Ctrl-Alt-Del equivalent.
- USB ethernet dongle... kernel sees it, loads driver :)

Diagnostics:

- plug in a USB drive with files:
  - ``jci-autoupdate``
  - ``dataRetrieval_config.txt``
    - runs anything as root O_O

- SSH was now listening on 3600, and switched to key-based authn
  - ``authorized_keys`` in read-only squashfs partition, thus can't
    be modified

- ticket number references... there is a change control process

- making user-modifiable systems that respect user freedom but are
  also secure by default... is an unsolved problem in embedded
  systems.
  - kinda solved for mobile w/ android

- there's a GPIO called "Parking Brake"
  - engage and disengage emergency brake? :O
  - fortunately it's read-only and can't be used to kill you

Future work:

- log GPS and engine data for analysis
- port OSM viewer to device
- investigate CAN bus bridge and its security
- make it easier to experiment without touching root fs
