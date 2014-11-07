multilingual programming - Nick Coghlan
=======================================

Why change the default?

- Knowing English *shouldn't* be a barrier to entry to programming.

One small part of a much larger pattern:

- making computers better at communicating with people
- instead of humans needing to understand computers, teach the
  computer to better understand human communication
- the long shadow of the telegraph
- some aspects of latest version of Unicode hark back 150 years.


History of encodings; ITA-2, ASCII-7bit, various 8-bit ASCII
encodings.  Codec explosion due to CJK.

Bilingual computing:

- internal encoding
- locale encodings for uesr I/O
- insufficient when dealing with multiple systems with incompatible
  encodings
- how to represent data of *different* encodings within a single
  document, e.g. word processing.  Can be done, but adds complexity.

Multilingual computing:

- *all* languages, *all* the time
- want a way to get from ASCII to a new world order
- Courtesy of Apple and Xerox: unicode!
- started just with languages currently in use (at the time)
  - allowed them to get by with 16-bits in the beginning (if you
    exclude proper nouns!  16-bits was not enough!)

Rise of unicode hasn't ended codec wars!

- UTF-8
  - ASCII compatibility
  - If data is ASCII-heavy, the most efficient Unicode codec
  - Usually a good wire format.
- UTF-16-LE
  - Basic Multilingual Plane (BMP) in two bytes
  - The most common 65k codepoints, ∴ good local storage format.
  - Most efficient for most multilingual text
- UTF-32
  - O(1) random code point access
- UCS2
  - two-bytes; restricted to BMP
  - UCS2 vs UTF-16 is a real source of bugs!
