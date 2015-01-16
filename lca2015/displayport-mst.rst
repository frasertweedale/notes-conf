DisplayPort MST: 4K and laptop dock fail
========================================

David Airlie, Red Hat Brisbane

DisplayPort Multi-stream Transport

Used in:

- 4K streams
- Lots of recent laptop docks

- someone dumped a laptop on desk, "why doesn't this work with two
  monitors when docked?"
- "I'll look at it; can't be that hard"
- "Oh."
- "That'll take about 6mths to fix"

History of display tech

- VGA
- DVI
  - an evolution of VGA
  - digital only, digital + analogue or analogue only (VGA in a
    different physical format)
- HDMI
  - DVI protocol, different physical format
  - driven by entertainment industry
  - licensing of technology, cables cost $75M
- DisplayPort

DP
--

- Main link (1,2,4 lanes)
  - 1 lane = 540MB/s (1.2 HBR), 270MB/s, 162MB/s
  - pretty much carries all the data (mainly video, audio)
- AUXCH
  - Control channel
  - 1MB/s
  - Fast AUXCH 675 MB/s (DP 1.2) (they put USB on it)
  - EDID emulation
- hot plug (HDP)

Single Stream Transport
-----------------------

- DP 1.1 or "SST mode"
- also specifies repeaters, switches; never seen them in real world


Multi-Stream Transport
----------------------

- One tx, multiple rx
- 63 "slots" on the links
  - A device can take multiple slots worth of bandwidth
- Can assign blocks to virtual channels
- Can point virtual channels to receivers
- Layered a whole messaging system on top of the DCPD register space
  - Every device in the tree has a UUID (to avoid loops)


Laptop docks
------------

- Lots of the manufacturers are using MST now, including Dell,
  Lenovo
- Saves them wires between laptop and dock.


MST hubs
--------

- not many out there
- seen one sensible one, and one with two two-port hubs to give
  fanout of three.


4K monitors
-----------

- There are single-stream 4K monitors from Acer and Samsung
  - Clock limits.  "You can't run this monitor on a laptop that...
    isn't heavy."
- Multi-stream 4K monitors
  - Double-panel; two streams.
  - A hub with two (logical; not physical) ports, with one panel on
    each.  They both report EDIDs.
  - Typical behaviour (on Linux and Windows) is it reports as two
    monitors, and usually default with L and R swapped.


Monitor tiling
--------------

- DisplayID v1.3
- Take the DisplayID tiling extension, wrap it in an EDID extension
  and report it with the EDID.
- Can also specify bezel information.


Vendor HW support
-----------------

- Intel have MST support from Haswell onwards
  - ULX can't do high-bitrate DP but can do MST
  - ULT can do high-bitrate DP but can't be used all for one image
  - regular chip can do everything
- Nvidia got onboard with GeForce 9 series
- Radeons can scan out 6 monitors; Nvidia can do 4.


X.org
-----

- There is no nice way to "hide this in the kernel"
- Kernel exposes the tiling info; X.org, GNOME and others have been
  modified; more work to do.


Future fun
----------

- 5K (5120 x 2880)
  - Dell just came out with it.  It's like Gilette -_-
  - There is not even enough bandwidth in DP cable to do it.
  - So the Dell monitor takes two cables!

- DP 1.3 (8.1GBps)
  - "Just make it more faster!"
  - Haven't seen any hardware yet; maybe another 6mths

- FreeSync
  - instead of scanning whole screen at 60Hz, send patches

- USB 3.1
  - DP guys went and put USB over DP, so USB guys are going to put
    DP over USB
