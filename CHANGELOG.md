# Changelog

All notable changes to this project will be documented in this file.

The format is loosely based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to cargo's version of [Semantic Versioning](https://doc.rust-lang.org/cargo/reference/semver.html).

Per Keep a Changelog there are 6 main categories of changes:
- Added
- Changed
- Deprecated
- Removed
- Fixed
- Security

#### Table of Contents

- [Unreleased](#unreleased)
- [v0.3.0](#v030)
- [Diffs](#diffs)

## Unreleased

- Added a `key_value_data` function to the reader that returns an iterator over key-value pairs (by @expenses).
- `Reader::levels` now returns an iterator over `Level` structs, which contain the bytes of the level as well as the uncompressed length (by @expenses).
- Added `Header::from_bytes`, `Header::as_bytes`, `LevelIndex::from_bytes` and `LevelIndex::as_bytes` (by @expenses).
- Made the following fields public (by @expenses):
  - `Header::LENGTH`
  - `Header::index`
  - `LevelIndex::LENGTH`
  - `LevelIndex::byte_offset`
  - `LevelIndex::byte_length`
  - `LevelIndex::uncompressed_byte_length`
  - `Level::data`
  - `Level::uncompressed_byte_length`
- Moved header data in `BasicDataFormatDescriptor` into `BasicDataFormatDescriptorHeader`.
- Add `ASTC_n_SFLOAT_BLOCK` variants to `Format`.

## v0.3.0

Released 2022-02-03

Initial release under new ownership.
- Added support for Data Format Descriptor parsing (Rob Swain [@superdump](https://github.com/superdump))

### Changed
- Cleaned up a signifigant portion of the crate.

## Diffs

- [Unreleased](https://github.com/BVE-Reborn/ktx2/compare/v0.3.0...HEAD)
