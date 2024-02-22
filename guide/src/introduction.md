# Introduction

The GMT NGAO model is build with the [GMT DOS Actors](https://rconan.github.io/dos-actors) integrated modeling framework.
It is leveraging 2 crates: [crseo] and [gmt_dos-clients_crseo].

[crseo] is a Rust wrapper for the GMT optical ray tracing and Fourier propagation software [CEO].
[gmt_dos-clients_crseo] is a higher level API build on top on [crseo] and it also implements the [interface] for DOS [actors]

[crseo]: https://crates.io/crates/crseo
[CEO]: https://github.com/rconan/CEO
[gmt_dos-clients_crseo]: https://crates.io/crates/gmt_dos-clients_crseo
[interface]: https://crates.io/crates/gmt_dos_actors-clients_interface
[actors]: https://crates.io/crates/gmt_dos_actors
