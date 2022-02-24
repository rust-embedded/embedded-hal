# Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on all stable Rust versions going back to
the version stated as MSRV in the README. It *might* compile with even older versions but
that may change in any new patch release.

## How the MSRV will be upgraded

For `embedded-hal`, we do not consider upgrading the MSRV a strictly breaking change as defined by
[SemVer](https://semver.org).

We follow these rules when upgrading it:

- We will not update the MSRV on any patch release: \_.\_.*Z*.
- We may upgrade the MSRV on any *major* or *minor* release: *X*.*Y*.\_.
- We may upgrade the MSRV in any preliminary version release (e.g. an `-alpha` release) as
  these serve as preparation for the final release.
- MSRV upgrades will be clearly stated in the changelog.

This applies both to `0._._` releases as well as `>=1._._` releases.

For example:

For a given `x.y.z` release, we may upgrade the MSRV on `x` and `y` releases but not on `z` releases.

If your MSRV upgrade policy differs from this, you are advised to specify the
`embedded-hal` dependency in your `Cargo.toml` accordingly.

See the [Rust Embedded Working Group MSRV RFC](https://github.com/rust-embedded/wg/blob/master/rfcs/0523-msrv-2020.md)
for more background information and reasoning.
