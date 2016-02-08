MediaGoblin
===========

Ben Sturmfels


- Simplify publishing pipeline

- Hassle, not much fun to convert media to publishing-suitable
  format.  MediaGoblin does it for you, with sensible defaults

  - Uses GStreamer for transcoding

- HTML5 video player

- Content license is clearly indicated

- Can do pdfs, e.g. of talk slides
  - pdf.js for in-browser rendering

- Content description field - can use Makrdown

- Written in Python

- 3D files (e.g. for 3D printing)
  - Can take a ``.stl`` and render it from a bunch of angles
  - WebGL rendering of the model
  - WAY COOL!
  - Plugin was written when a bounty of a 3D printer was offered!

- ASCII art plugin


Why MediaGoblin?

- Runs on a machine that *you* control

- Maybe it's not for you, if you don't care where things are stored
  and like that generic experience other hosts give you

- MediaGoblin is designed for decentralisation

- Most existing media hosts:
  - Censor
  - Surveille users
  - Broad,  sweeping, *simplistic* copyright infringement detection
    and enforcement
    - Normally up to user to defend fair use
    - Your interests don't really matter in comparison to large
      media companies, Govt, etc.
  - You don't know where your media is actually stored
  - Large for-profit companies are not good custodians of our
    cultural works.

- MediaGoblin is a project for modern, freedom-preserving network
  services
- Can scale down for organisations, communities, etc.
- Can run it on affordable hardware
- Rely less on the network (i.e. Australian internet sucks)
- Choose to host our media in a way that meets our own interests,
  not those of large companies
- YOU decide ToS
- YOU decide what's appropriate or not
- YOUR opinions matter
- YOU decide where things are hosted

The Tech:

- Python web app
- Django-ish
- "front-end Goblins" serve the media
  - serve the web interface and APIs
  - other goblins do the transcoding etc in background

- Built upon
  - Jinja2 templates
  - WTForms
  - Werkzeug URL routing (WSGI)
  - SQLAlchemy
  - PostgreSQL / SQLite
  - Celery and Kombu for background task processing
  - GStreamer 1.0
  - early Python3 support
- AGPLv3+
- pump.io API
  - JSON ActivityStreams 1.0

Command-line upload::

  gmg addmedia ben frog.jpg

  gmg addmedia ben frog.jpg --title X --description Y \
    --license URL --tags "foo,bar" --slog "pobblebonk"


Other features:

- reporting/moderation
- tagging
- collections
- geo-tagged photos
- visual gtheming
- per-user license preference
- Persona/OpenID/LDAP support


Future:

- GMG is a young but very ambitious project
- Federation
- Finish 1.0
- Privacy features

Federation:

- What it means: a social experience for MG
  - can follow / comment etc without having account of a particular
    MG instance
  - server-to-server technology

MediaGoblin is not just about the tech, but also the social process

- We are working hard to bring in standards
  - W3C social working group
  - ActivityStreams 2.0

Android app:

- *Goblinoid*
- Interesting technical challenges
- F-Droid

Deployment:

- Python deployment is still a Hard Thing
- Distro packaging
- Guix is looking to be a good solution

MediaGoblin is turning your money into free software:

- Stewarded by FSF
- Legally obliged to act in public interest
- Please support us!
- Media hosting for artists


QUESTIONS:

- Handling of composite images?
  - i.e. huge images consisting of a collections of existing images
  - Answer: it is possible to write a plugin

- Is there a bounty market for plugins?
  - There haven't be many but it would be an effective way to spur
    development.

- Account mgmt, SSO etc?
  - Default is usernames / password hashes in DB (i.e. identity silo)
  - Not aware of organisations deploying internally
  - You can deploy behind a webserver (Apache, Nginx, ...)
  - Wasn't sure if it observes external authn but if not, it
    would no be hard to make it do so
