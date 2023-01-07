# Changelog

## OxChain 0.1.3 (2022-01-04)

### Added

- Added `Hashable` trait.
    - Implemented for `Block` and `Transaction`.
- Added a new `application` module.
- Added a new test case for `find_merkle_root` function.

### Removed

- Removed `calculate_hash` function in `Block`.

## OxChain 0.1.2 (2022-12-29)

### Changed

- Structs moved into their own files.

## OxChain 0.1.1 (2022-12-28)

### Added

- Added `Mempool` to `Blockchain` struct.
- Added `Merkle Root` to the block header.
- Added `Wallet` and `Transaction` structs.
- Added `rustfmt.toml` file the project.
- Added this changelog to track and document changes in the project.

### Changed

- Removed useless `data`, instead add `Transaction` in `Block` struct.

