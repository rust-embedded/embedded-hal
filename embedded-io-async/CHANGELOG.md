# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.7.0 - 2025-09-30

- Make `Write::flush()` a required method, aligning with std and embedded-io
- Update to and align with embedded-io 0.7:
  - Increase MSRV to 1.81
  - Error type is updated to include core::Error
  - Update `defmt` dependency to 1.0; rename feature from `defmt_03` to `defmt`
  - Require `Read` and `Write` to be implemented for various Read and Write traits
  - Fix missing method forwardings for blanket implementations
  - Implement `Read`, `ReadReady`, `BufRead`, `Write`, and `WriteReady` for `VecDeque<u8>`
  - Documentation updates

## 0.6.1 - 2023-11-28

- Use `feature()` on nightly toolchains only. This adds support for 1.75 beta and stable.

## 0.6.0 - 2023-10-02

- Prohibit `Write::write` implementations returning `Ok(0)` unless there is no
  data to write; consequently remove `WriteAllError`.
  Update the `&mut [u8]` impl to possibly return
  a new `SliceWriteError` if the slice is full instead of `Ok(0)`.
- Add `WriteZero` variant to `ErrorKind` for implementations that previously
  may have returned `Ok(0)` to indicate no further data could be written.
- `Write::write_all` now panics if the `write()` implementation returns `Ok(0)`.

## 0.5.0 - 2023-08-06

- First release
