# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

- Added `ToFmt` adapter for `core::fmt::Write`.

## 0.6.1 - 2023-11-28

- Handle reading from `FromTokio` with empty buffer, ensuring `Ok(0)` is always returned.
- Use `feature()` on nightly toolchains only. This adds async support for 1.75 beta and stable.

## 0.6.0 - 2023-10-02

- Add support for adapting `BufRead` from `futures` and `tokio`.
- Return an error when a wrapped `std`/`futures`/`tokio` `write()` call returns
  `Ok(0)` to comply with `embedded_io::Write` requirements.

## 0.5.0 - 2023-08-06

- First release
