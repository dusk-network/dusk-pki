// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

//! ![Build Status](https://github.com/dusk-network/dusk-pki/workflows/Continuous%20integration/badge.svg)
//! [![Repository](https://img.shields.io/badge/github-dusk--pki-blueviolet?logo=github)](https://github.com/dusk-network/dusk-pki)
//! [![Documentation](https://img.shields.io/badge/docs-dusk--pki-blue?logo=rust)](https://docs.rs/dusk-pki/)

//!
//! # Dusk Public Key Infrastructure
//!
//! This repository has been created so there's a unique library that holds the
//! types and functions required to perform keys operations.

#![no_std]
#![deny(missing_docs)]
#![allow(non_snake_case)]

/// Public Key
pub use keys::public::PublicKey;
/// Secret Key
pub use keys::secret::SecretKey;
/// Public Spend Key
pub use keys::spend::public::PublicSpendKey;
/// Secret Spend Key
pub use keys::spend::secret::SecretSpendKey;
/// Stealth Address
pub use keys::spend::stealth::{Ownable, StealthAddress};
/// ViewKey
pub use view::ViewKey;

mod keys;
mod permutation;
mod view;

use dusk_jubjub::{JubJubAffine, JubJubExtended, JubJubScalar};
