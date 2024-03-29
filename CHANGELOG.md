# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Added deprecation notice [#79]

## [0.13.0] - 2023-10-12

### Changed

- Update `dusk-poseidon` from `0.30` to `0.31`
- Update `dusk-jubjub` from `0.12` to `0.13`

### Removed

- Remove `canonical` dependencies and features

## [0.12.0] - 2023-06-28

### Changed

- Update `dusk-poseidon` from `0.28` to `0.30`
- Update `rust-toolchain` from `nightly-2022-08-08` to `nightly-2023-05-22`

## [0.11.4] - 2023-04-05

### Added

- Derive `Default` for `StealthAddress`

## [0.11.3] - 2022-12-19

### Added

- Derive `Default` for `PublicKey`

## [0.11.2] - 2022-10-27

### Changed

- Update `dusk-poseidon` from `0.26` to `0.28`

## [0.11.1] - 2022-10-19

### Added

- Add support for `rkyv-impl` under `no_std`

## [0.11.0] - 2022-08-17

### Added

- Add `rkyv` implementations behind feature gate [#66]
- Add `PublicKey::from_raw_unchecked` and `StealthAddress::from_raw_unchecked` [#63]

### Changed

- Update `dusk-jubjub` from `0.10` to `0.12`
- Update `dusk-poseidon` from `0.24.0-rc` to `0.26`
- Update `canonical` from `0.6` to `0.7`
- Update `canonical_derive` from `0.6` to `0.7`
- Update `StealthAddress::R` and `StealthAddress::pk_r` to `const fn` [#63]

## [0.8.0] - 2021-07-27

### Changed

- Change `PartialEq` and `Eq` impls for `PublicKey` [#53]
- Update `dusk-poseidon` from v0.21.0 to v0.22 [#54]

## [0.7.0] - 2021-07-05

### Changed

- Update `canonical` from v0.5.0 to v0.6.0 [#44]
- Update `rand_core` from v0.5.0 to v0.6.0 [#44]
- Update `dusk-jubjub` from v0.8.0 to v0.10.0 [#44]
- Update `dusk-poseidon` from v0.20.0 to v0.21 [#44]
- Update `rand_core` to not use default features [#48]
- Change `permutation::hash` to use poseidon's `truncated::hash` [#50]

### Removed

- Remove `rand` from dev-dependencies [#44]

## [0.6.2] - 2021-04-06

### Changed

- Update dusk-poseidon to `v0.20` [#42]

## [0.6.1] - 2021-02-12

### Added

- Add Cargo.toml fields necessary to be published on crates.io
- Add a short description to README.md

## [0.6.0] - 2021-02-11

### Changed

- Update dusk-poseidon to `v0.18` [#38]

## [0.5.3] - 2021-02-09

### Changed

- Disable `subtle` default-features flag [#36]

## [0.5.2] - 2021-02-09

### Changed

- Disable `jubjub` default-features flag [#34]

## [0.5.1] - 2021-02-01

### Changed

- Bump `poseidon252` to `v0.17.0`

## [0.5.0] - 2021-01-28

### Added

- Add `PublicKey` and `SecretKey` (removed from `schnorr`)
- Add `dusk_bytes::Serializable` trait to structure

### Removed

- Remove manual implementation of `to_bytes` and `from_bytes`
- Remove `Error` enum
- Remove `decode` function

### Changed

- Bump `dusk-jubjub` to `v0.8`
- Bump `poseidon252` to `v0.16.0`
- Bump `canonical` to `v0.5`
- Bump `canonical_derive` `v0.5`
- Update CHANGELOG to ISO 8601

## [0.4.1] - 2020-11-26

### Changed

- Use poseidon252 dependency.

## [0.4.0] - 2020-11-17

### Changed

- No-Std compatibility.

[#79]: https://github.com/dusk-network/dusk-pki/issues/79
[#66]: https://github.com/dusk-network/dusk-pki/issues/66
[#63]: https://github.com/dusk-network/dusk-pki/issues/63
[#60]: https://github.com/dusk-network/dusk-pki/issues/60
[#54]: https://github.com/dusk-network/dusk-pki/issues/54
[#53]: https://github.com/dusk-network/dusk-pki/issues/53
[#50]: https://github.com/dusk-network/dusk-pki/issues/50
[#48]: https://github.com/dusk-network/dusk-pki/issues/48
[#44]: https://github.com/dusk-network/dusk-pki/issues/44
[#42]: https://github.com/dusk-network/dusk-pki/issues/42
[#38]: https://github.com/dusk-network/dusk-pki/issues/38
[#36]: https://github.com/dusk-network/dusk-pki/issues/36
[#34]: https://github.com/dusk-network/dusk-pki/issues/34
[Unreleased]: https://github.com/dusk-network/dusk-pki/compare/v0.13.0...HEAD
[0.13.0]: https://github.com/dusk-network/dusk-pki/compare/v0.12.0...v0.13.0
[0.12.0]: https://github.com/dusk-network/dusk-pki/compare/v0.11.4...v0.12.0
[0.11.4]: https://github.com/dusk-network/dusk-pki/compare/v0.11.3...v0.11.4
[0.11.3]: https://github.com/dusk-network/dusk-pki/compare/v0.11.2...v0.11.3
[0.11.2]: https://github.com/dusk-network/dusk-pki/compare/v0.11.1...v0.11.2
[0.11.1]: https://github.com/dusk-network/dusk-pki/compare/v0.11.0...v0.11.1
[0.11.0]: https://github.com/dusk-network/dusk-pki/compare/v0.8.0...v0.11.0
[0.8.0]: https://github.com/dusk-network/dusk-pki/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/dusk-network/dusk-pki/compare/v0.6.2...v0.7.0
[0.6.2]: https://github.com/dusk-network/dusk-pki/compare/v0.6.1...v0.6.2
[0.6.1]: https://github.com/dusk-network/dusk-pki/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/dusk-network/dusk-pki/compare/v0.5.3...v0.6.0
[0.5.3]: https://github.com/dusk-network/dusk-pki/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/dusk-network/dusk-pki/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/dusk-network/dusk-pki/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/dusk-network/dusk-pki/compare/v0.4.1...v0.5.0
[0.4.1]: https://github.com/dusk-network/dusk-pki/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/dusk-network/dusk-pki/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/dusk-network/dusk-pki/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/dusk-network/dusk-pki/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/dusk-network/dusk-pki/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/dusk-network/dusk-pki/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/dusk-network/dusk-pki/releases/tag/v0.1.0
