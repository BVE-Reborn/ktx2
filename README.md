# ktx2

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/BVE-Reborn/ktx2/build.yml?branch=trunk)
[![Crates.io](https://img.shields.io/crates/v/ktx2)](https://crates.io/crates/ktx2)
[![Documentation](https://docs.rs/ktx2/badge.svg)](https://docs.rs/ktx2)
![License](https://img.shields.io/crates/l/ktx2)

Parser for the [ktx2](https://github.khronos.org/KTX-Specification/ktxspec.v2.html) texture container format.

### Features
- [x] Async reading
- [x] Parsing
- [x] Validating
- [x] [Data format description](https://github.khronos.org/KTX-Specification/ktxspec.v2.html#_data_format_descriptor)
- [x] [Key/value data](https://github.khronos.org/KTX-Specification/ktxspec.v2.html#_keyvalue_data)

### Example
```rust
// Crate instance of reader. This validates the header
let mut reader = ktx2::Reader::new(file).expect("Can't create reader"); // Crate instance of reader.

// Get general texture information.
let header = reader.header();

// Read iterator over slices of each mipmap level.
let levels = reader.levels().collect::<Vec<_>>();
```

### MSRV

The minimum supported Rust version is 1.56. MSRV bumps are treated as breaking changes.

License: Apache-2.0
