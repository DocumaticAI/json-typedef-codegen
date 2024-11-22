# Changelog

## v0.2.0 (2024-11-22)

### Breaking Changes

- Changed Rust typegen to output `snake_case` field names instead of `camelCase`
- Added following traits to generated Rust types:
  - `PartialEq`
  - `Eq`
  - `PartialOrd`
  - `Ord`
  - `Hash`

## v0.1.0 (2024-11-21)

Initial release of Jetted. Approximately equivalent to `jtd-codegen` v0.5.0.

### Breaking Changes over `jtd-codegen`

- Added `Clone` and `Debug` traits to generated Rust types
