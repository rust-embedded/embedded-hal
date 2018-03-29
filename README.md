# `embedded-hal`

>  A Hardware Abstraction Layer (HAL) for embedded systems

## [API reference]

[API reference]: https://docs.rs/embedded-hal

## How-to: add a new trait

This is the suggested approach to adding a new trait to `embedded-hal`

### Discussion

Ideally, before proposing a new trait, or set of traits, you should create an issue where the use
cases and requirements of the trait(s) are discussed.

These issues will be labeled as `discussion`s in the issue tracker.

### Proposing a trait

Once there's consensus on the requirements of the trait(s) a new issue, or a PR, with a proposal
should be opened. The proposal should include the actual trait definitions as well as a link to the
issue with previous discussion, if there was one.

If the proposal includes more than one alternative then there should be further discussion to try to
single out the best alternative.

These issues / PRs will be labeled as `proposal`s in the issue tracker.

### Testing period

If there are no objections to the proposal the new trait(s) will land behind the "unproven" Cargo
feature and an issue about the new trait(s) will be created. If the proposal includes several
alternatives and a single one couldn't be chosen as the best then each alternative will land behind
a different Cargo feature, e.g. "alt1" or "alt2".

The traits will undergo a testing period before they move into the set of proven traits. During
this period users are encouraged to try to implement the unproven traits for their platforms and to
build drivers on top of them. Problems implementing the trait(s) as well as successful
implementations should be reported on the corresponding issue.

To leave the unproven state at least *two* implementations of the trait(s) for different platforms
(ideally, the two platforms should be from different vendors) and *one* generic driver built on top
of the trait(s), or alternatively one demo program that exercises the trait (via generic function /
trait object), *should* be demonstrated. If, instead, reports indicate that the proposed trait(s)
can't be implemented for a certain platform then the trait(s) will be removed and we'll go back to
the drawing board.

Issues used to track unproven APIs will be labeled as `unproven-api`s in the issue tracker and they
may also include the labels `needs-impl` and `needs-driver` to signal what's required for them to
move to the set of proven traits.

## Implementations

These are (WIP) implementations of `embedded-hal` for various platforms. Feel free to send a PR
adding yours to the list!

You may be able to find even more implementations by searching for the [`embedded-hal-impl`] keyword
an crates.io. If you publish a `embedded-hal` implementation to crates.io please use that keyword to
let others more easily find your crate!

[`embedded-hal-impl`]: https://crates.io/keywords/embedded-hal-impl

### Linux

- [`linux-embedded-hal`]. For the Raspberry Pi and other SBC that expose pins with embedded
  functionality (SPI, I2C, etc.)

[`linux-embedded-hal`]: https://crates.io/crates/linux-embedded-hal

### Nordic

- [`nrf51-hal`]. Check the [`microbit`] crate for examples that can be run on the [micro:bit]

[`nrf51-hal`]: https://crates.io/crates/nrf51-hal
[`microbit`]: https://crates.io/crates/microbit
[micro:bit]: http://microbit.org/

### NXP

- [`lpc82x-hal`]

[`lpc82x-hal`]: https://github.com/braun-robotics/rust-lpc82x-hal

- [`mkw41z-hal`]. Check the [`frdm-kw41z`] crate for examples that can be run on the NXP [FRDM-KW41Z] boards

[`mkw41z-hal`]: https://crates.io/crates/mkw41z-hal
[`frdm-kw41z`]: https://crates.io/crates/frdm-kw41z
[FRDM-KW41Z]: https://www.nxp.com/products/processors-and-microcontrollers/arm-based-processors-and-mcus/kinetis-cortex-m-mcus/w-serieswireless-conn.m0-plus-m4/freedom-development-kit-for-kinetis-kw41z-31z-21z-mcus:FRDM-KW41Z

### ST Microelectronics

- [`stm32f042-hal`]. Contains examples that can be run on the [Nucleo-F042K6] and similar boards.

[`stm32f042-hal`]: https://crates.io/crates/stm32f042-hal
[Nucleo-F042K6]: http://www.st.com/en/evaluation-tools/nucleo-f042k6.html

- [`stm32f103xx-hal`]. Contains examples that can be run on the [Blue pill] and similar boards.

[`stm32f103xx-hal`]: https://github.com/japaric/stm32f103xx-hal
[Blue pill]: wiki.stm32duino.com/index.php?title=Blue_Pill

- [`stm32f30x-hal`]. Check the [`f3`] crate for examples that can be run on the STM32F3DISCOVERY.

[`stm32f30x-hal`]: https://crates.io/crates/stm32f30x-hal
[`f3`]: https://crates.io/crates/f3

### Texas Instruments

- [`tm4c123x-hal`]

[`tm4c123x-hal`]: https://github.com/thejpster/tm4c123x-hal

## Drivers

These are (WIP) platform agnostic drivers that can be used with any of the above implementations to
interface all sort of external devices like sensors and actuators. Feel free to send a PR adding
yours to the list!

You may be able to find even more implementations by searching for the [`embedded-hal-driver`]
keyword an crates.io. If you publish a driver to crates.io please use that keyword to let others
more easily find your crate!

- [`l3gd20`]. Gyroscope

[`l3gd20`]: https://crates.io/crates/l3gd20

- [`lsm303dlhc`]. Accelerometer + compass

[`lsm303dlhc`]: https://crates.io/crates/lsm303dlhc

- [`mag3110`]. Magnetometer

[`mag3110`]: https://crates.io/crates/mag3110

- [`mfrc522`]. RFID reader / writer

[`mfrc522`]: https://crates.io/crates/mfrc522

- [`motor-driver`]. Motor drivers like the L298N and the TB6612FNG

[`motor-driver`]: https://github.com/japaric/motor-driver

- [`mpu9250`]. Accelerometer + gyroscope + magnetometer IMU

[`mpu9250`]: https://github.com/japaric/mpu9250

- [`si5351`]. Clock generator

[`si5351`]: https://github.com/ilya-epifanov/si5351

- [`si7021`]. Humidity and temperature sensor

[`si7021`]: https://github.com/wose/si7021

- [`mcp3425`]. 16-bit ADC

[`mcp3425`]: https://github.com/dbrgn/mcp3425-rs/

[`embedded-hal-driver`]: https://crates.io/keywords/embedded-hal-driver

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
