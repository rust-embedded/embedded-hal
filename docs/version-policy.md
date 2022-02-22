# Version policy

At the moment we are working towards a `1.0.0` release (see [#177]). During this process we will
release alpha versions like `1.0.0-alpha.1` and `1.0.0-alpha.2`.
Alpha releases are **not guaranteed** to be compatible with each other.
They are provided as early previews for community testing and preparation for the final release.
If you use an alpha release, we recommend you choose an exact version specification in your
`Cargo.toml` like: `embedded-hal = "=1.0.0-alpha.2"`

See below for a way to implement both an `embedded-hal` `0.2.x` version and an `-alpha` version
side by side in a HAL.

[#177]: https://github.com/rust-embedded/embedded-hal/issues/177

## Supporting different (alpha and non-alpha) HALs

[embedded-hal-compat](https://github.com/ryankurte/embedded-hal-compat) provides shims
to support interoperability between the latest `0.2.x` and `1.0.0-alpha.N` HALs, allowing one to use
incompatible HAL components (generally) without alteration.
See the [docs](https://docs.rs/embedded-hal-compat/) for examples.

It is also possible for HAL implementations to support both the latest `0.2.x` and `1.0.0-alpha.N` versions
side by side, for an example see [LPC8xx HAL](https://github.com/lpc-rs/lpc8xx-hal).

Note that `embedded-hal` `-alpha` versions are a moving target and _not guaranteed_ to be compatible.
Because of this we only aim to support the latest `-alpha`.
