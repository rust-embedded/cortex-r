# Rust on Arm Cortex-R

This repository contains support libraries for Rust on Arm Cortex-R.

These libraries have been written by Ferrous Systems, and are based on the
[`cortex-m` libraries] from the [Rust Embedded Devices Working Group].

[`cortex-m` libraries]: https://github.com/rust-embedded/cortex-m
[Rust Embedded Devices Working Group]: https://github.com/rust-embedded

There are currently three libraries here:

* [cortex-r](./cortex-r/) - support library for Cortex-R CPUs (like [cortex-m])
* [cortex-r-rt](./cortex-r-rt/) - run-time library for Cortex-R CPUs (like [cortex-m-rt])
* [arm-targets](./arm-targets/) - a helper library for your build.rs that sets various `--cfg` flags according to the current target

There are also example programs for QEMU in the [cortex-r-examples](./cortex-r-examples/) folder.

[cortex-m]: https://crates.io/crates/cortex-m
[cortex-m-rt]: https://crates.io/crates/cortex-m-rt

## Licence

* Copyright (c) Ferrous Systems, 2025
* Copyright (c) The Cortex-M Team <cortex-m@teams.rust-embedded.org>

Licensed under either [MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE) at
your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be licensed as above, without any
additional terms or conditions.
