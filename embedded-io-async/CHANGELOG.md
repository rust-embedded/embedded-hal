# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

Add unreleased changes here

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
