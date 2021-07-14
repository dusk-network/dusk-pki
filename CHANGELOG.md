# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Update `dusk-poseidon` from v0.21.0 to v0.22 [#54](https://github.com/dusk-network/dusk-pki/issues/54)

## [0.7.0] - 2021-07-05

### Changed

- Update `canonical` from v0.5.0 to v0.6.0 [#44](https://github.com/dusk-network/dusk-pki/issues/44)
- Update `rand_core` from v0.5.0 to v0.6.0 [#44](https://github.com/dusk-network/dusk-pki/issues/44)
- Update `dusk-jubjub` from v0.8.0 to v0.10.0 [#44](https://github.com/dusk-network/dusk-pki/issues/44)
- Update `dusk-poseidon` from v0.20.0 to v0.21 [#44](https://github.com/dusk-network/dusk-pki/issues/44)
- Update `rand_core` to not use default features [#48](https://github.com/dusk-network/dusk-pki/issues/48)
- Change `permutation::hash` to use poseidon's `truncated::hash` [#50](https://github.com/dusk-network/dusk-pki/issues/50)

### Removed

- Remove `rand` from dev-dependencies [#44](https://github.com/dusk-network/dusk-pki/issues/44)

## [0.6.2] - 2021-04-06

### Changed

- Update dusk-poseidon to `v0.20` [#42](https://github.com/dusk-network/dusk-pki/issues/42)

## [0.6.1] - 2021-02-12

### Added

- Add Cargo.toml fields necessary to be published on crates.io
- Add a short description to README.md

## [0.6.0] - 2021-02-11

### Changed

- Update dusk-poseidon to `v0.18` [#38](https://github.com/dusk-network/dusk-pki/issues/38)

## [0.5.3] - 2021-02-09

### Changed

- Disable `subtle` default-features flag [#36](https://github.com/dusk-network/dusk-pki/issues/36)

## [0.5.2] - 2021-02-09

### Changed

- Disable `jubjub` default-features flag [#34](https://github.com/dusk-network/dusk-pki/issues/34)

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
