Twisted as an IoT Controller
============================

Building a 300+ Seat IFE Controller in Twisted at Digecor
---------------------------------------------------------

David P. Novakovic

- @dpn
- dpn.name
- tech founder at scrunch.co
- more of a data scientist than anything else

Outline:

- what is twisted
- IFE system devices
- IFE system requirements
- code examples

Twisted

- seems to have a bad rep with people who don't use it
- event driven framework in python
- particular focus on network protocols
- from web down to serial ports, tun/tap and udev
- async io - single process can handle thousands of concurrent
  connections on low-end machine
- clean apis for callbacks and deferred
- thread pool with deferred interface
- locking "primitives" - ``DeferredSemaphore``, ``DeferredLock``,
  ``DeferredQueue``.

More twisted:

- application framework
- async unit testing
- integration with most major event loops (wx, gtk, qt, gevent, etc)
- 3rd party libs

Traditional IFE:

- dumb screens, heavy use of streaming content
- very heavy requirements on servers
- very expensive to upgrade
- basically a full rack in an airplane
- ~20k per seat

Glide IFE:

- heavier clients
- system can be upgraded by replacing players
- plane doesn't need rewiring etc
- server can remain fairly unchanged
- more importantly - don't need to be upgraded in lockstep.
- players: embedded debian
- seat disconnect - custom embedded device
- triple-redundant power supplies - switch + power
- cm controller - dual core atom running ubuntu LTS + lots of
  embedded things attached.

Glide Server:

- control whole system from cabin area
- HDDs with content

Requirements:

- multicast commands out to seats
- 300+ seat updates over HTTP every second
- listen to audio stream over multicast (PA, pilot)
- low lat control of players
- telnet into streaming VLC process
- some legacy - sync code needed to run in threads
- respond to and control hardware in the plane (overhead screens
  etc)
- cabin crew inserting HDD with content (lock down USB)
- downloading content from the web (at gate)
- kiosk (lock down control keys/usb ports)
- manhole for debugging a running process
- ssh reverse tunnel for remote access - conch
- tftp - firmware updates to players
- secured via crypto things

Build awesome things:

- digEcor is hiring
  - jobs@digEcor

Qns:

- crypto - what?  what pki needs?  access control? etc
