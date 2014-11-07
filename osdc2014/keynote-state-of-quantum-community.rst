State of Quantum Computing, and Related Open Source Projects
============================================================

A/Prof Tom Stace

- ARC Centre for Engineered Quantum Systems, UQ

Using quantum systems:

- 22 years ago: Cray X-MP at CERN
- Today: iPhone
  - sensors: RF, GPS, magnetic field, touch, accelerometer, MEMS
    gyroscope, hi-def video, noise-cancelling mic
- "A lot of room at the bottom" - Richard Feynman
  - number of equations involved grows exponential in number of
    particles
- Current computer power dissipation is 10^10 - 10^12 times greater
  than theoretical limits.

Quantum mechanics:

- physics at short scales
- physics at cold temperatures
- physics at low energies
- how can we use it?

Central challenges:

- establish control of individual quantum components
- integrate quantum systems to produce platforms for new technology
- demonstrate novel application of quantum technology
  - quantum measurement/control
    - computing, communication
  - quantum-enabled sensors and metrology
    - high precision measurement
  - synthetic quantum systems and simulation
    - design new checmicals

Recent past:

- magnetic resonance
- semiconductor devices
  - GoAs Quantum Dots
- optics and photonics
  - cQED
- electrical circuits
- mechanical
  - QEMS
- atom and ion traps

What is quantum mechanics?
--------------------------

The quantum revolution:

- wave-particle duality
  - waves: diffraction and interference
  - particles: discrete
  - Young double-slit experiment
    - light arrives in indivisible packets of energy but
      nevertheless behaves like a wave
  - half-silvered mirror experiement
  - everything can behave like a particle or a wave, depending on
    the experiement
    - electrons, atoms, buckyballs, neutrons, molecules, viruses
  - qubits
    - Polarisation encoding
    - Spatial encoding
    - Path encoding

- entanglement
  - schroedinger's cat: ``|decay,dead> + |no decay, alive>``
  - entanglement suggested to einstein that quantum mechanics was
    incomplete.  Einstein, 1935 (EPR paper).
  - John Bell's inequalities, 1964.  "Hidden variable"
    - Experiment in 1982 showed that either
      - there is something happening faster than light, or
      - correlations exist that cannot be explained with classical
        probability

What is information?

- classical bits: binary digits.  0 or 1.  Three bit register can
  store one number in 0..7
- qubits: single quantum register can simultaneously represent all
  possible states of a computation.
- classical gates: NAND; irreversible.
  - you can build a gate whose output uniquely determines the input
    and vice versa.
- quantum gates: *every* gate *must* be reversable
- to represent a 300-qubit state in a classical computer requires
  2^300 complex numbers.
- killer apps: factoring and simulation

Quantum Advantage:

- factoring: O(n^2 log n log log n)
- search: O(N) vs O(sqrt N)

Error correction:

- DRAM error rates that are orders of magnitude higher than
  previously reports, with 25k - 70k per billion device hours per
  MBit
  - 10^-13 error/bit/s
  - >8% of DIMMs affected by errors per year

- Quantum computers need to be protected from errors
  - ``|v2> = |00> + |11>``
  - Quantum error correcting codes, 2010 paper


Open Source Physics
-------------------

- Matrix Product Toolkit (UQ)
- xmds - Differential eqn solver (Schroedinger) (ANU)
- labscript - laboratory control (Monash)
- physicists don't always have the time and the skills to build the
  software we need, so help out!
