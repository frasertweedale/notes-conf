# Embedding Golang in a Kotlin app: how to do it and why you shouldn't

- Charles Korn, Grafana Labs


The magic of C interop

- use C as a bridge between kotlin and go


Java C FFI

- JNI: the original.  It's a pain.
- JNA: like JNI, but no C wrapper to write
- JNR: like JNA, but with better performance
- Panama: like JNR, but built into the runtime and with better
  tooling.  Still in preview for Java 20.


