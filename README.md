# JP's experimental RP2350 HAL

**NOTE:** This is an experimental fork of the rp2040-hal with changes to support
the RP235x family. You should not use this repo, and instead you should wait
until patches are applied to the upstream repo at <https://github.com/rp-rs>.
Whilst some things here have been tested, there are likely many broken things
here.

The Raspberry Pi RP2040 was the first chip in the RP2x family, and was released
in 2021. It has two Cortex-M0+ processor cores.

The Raspberry Pi RP2350A and RP2350B came next in the RP2x family, and were
released in 2024. We refer to them as *RP235x* as they are functionally very
similar - the letter suffix indicates the package (QFN60 or QFN80). The *RP235x*
microcontrollers all have two Arm Cortex-M33 processor cores and two Hazard3
RISC-V (riscv32imac) processor cores. Support for *RP235x* is TBC. See the
[Programming](#programming) section for more details.

In addition, we support the RP2354A and RP2354B. These are basically the same,
except they include 2 MiB of internal QSPI flash. You can treat them just like
an RP2350x in most respects.

Before trying any of the examples, please ensure you have the latest stable
version of Rust installed, along with the right target support:

```sh
rustup self update
rustup update stable
rustup target add thumbv8m.main-none-eabihf
rustup target add riscv32imac-unknown-none-elf
```

**Note:** probe-rs does not yet work with RP2350. You should use Raspberry Pi's
`picotool` instead. You'll have to compile that from source using Raspberry Pi's
picotool repository.

An *RP235x* contains two Arm Cortex-M33 processors, which execute Thumb-2
encoded instructions from the ARMv8-M architecture, including both the
*Baseline* instructions and those from the *Main Extension*. There are no
operating-specific features in the binaries produced - they are for 'bare-metal'
systems. For compatibility with other Arm code (e.g. as produced by GCC), Rust
uses the *Arm Embedded-Application Binary Interface* standard or EABI.
Therefore, any Rust code for an RP235x in Arm mode should be compiled with the
target *`thumbv8m.main-none-eabi`*.

An *RP235x* also contains two Hazard3 RISC-V processors, which execute riscv32imac
instructions in *Machine Mode*. Again, there are no operating-specific features
in the binaries produced - they are for 'bare-metal' systems. For compatibility
with other RISC-V code (e.g. as produced by GCC), Rust uses the RISC-V ELF
specification. Therefore, any Rust code for an *RP235x* in RISC-V mode should be
compiled with the target *`riscv32imac-unknown-none-elf`*.

You can only enable two CPUs at any one time, as each Hazard3 shares a bus
interface with its associated Cortex-M33. It effectively therefore has two CPUs,
and each CPU can reboot between Arm mode and RISC-V mode.

## Linker flags

Besides the correct target, which mainly defines the instruction set, it's also
necessary to use a certain memory layout compatible with the particular RP2x
microcontroller. To achieve that, `rustc` must be called with appropriate linker
flags.

When the RP2350 is in Arm mode the build uses a `memory.x` file as before. When
in RISC-V mode, you need the `rp235x_riscv.x` file instead. Currently interrupts
don't work in RISC-V mode because we have yet to write a software equivalent of
the Arm NVIC to read from the RISC-V mode interrupt peripheral and dispatch the
interrupt to the appropriate registered interrupt handler.

<!-- CODE OF CONDUCT -->
## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], and the maintainer of this crate, the [rp-rs team], promises
to intervene to uphold that code of conduct.

[CoC]: CODE_OF_CONDUCT.md
[rp-rs team]: https://github.com/orgs/rp-rs/people

<!-- LICENSE -->
## License

The contents of this repository are dual-licensed under the *MIT OR Apache 2.0*
License. That means you can choose either the MIT license or the Apache-2.0
license when you re-use this code. See `MIT` or `APACHE2.0` for more information
on each specific license.

Any submissions to this project (e.g. as Pull Requests) must be made available
under these terms.

<!-- ACKNOWLEDGEMENTS -->
## Acknowledgements

* [Othneil Drew's README template](https://github.com/othneildrew)
* [Rust Embedded Working Group](https://github.com/rust-embedded)
* [Raspberry Pi](https://raspberrypi.org) and the [Pico SDK](https://github.com/raspberrypi/pico-sdk)
