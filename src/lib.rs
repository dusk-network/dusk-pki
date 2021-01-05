// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

//! ![Build Status](https://travis-ci.com/dusk-network/dusk-pki.svg?branch=master)](https://travis-ci.com/dusk-network/dusk-pki)
//! ![Repository](https://dusk-network.github.io/dusk-pki/repo-badge.svg)](https://github.com/dusk-network/dusk-pki)
//! ![Documentation](https://dusk-network.github.io/dusk-pki/badge.svg)](https://dusk-network.github.io/dusk-pki/index.html)
//!
//! # Dusk Public Key Infrastructure
//!
//! This repository has been created so there's a unique library that holds the
//! types and functions required to perform keys operations.

#![no_std]
#![deny(missing_docs)]
#![allow(non_snake_case)]

pub use decode::decode as jubjub_decode;

/// PKI Errors
pub use errors::Error;
/// Public Spend Key
pub use spend::public::PublicKey as PublicSpendKey;
/// Secret Spend Key
pub use spend::secret::SecretKey as SecretSpendKey;
/// Stealth Address
pub use spend::stealth::{Ownable, StealthAddress};
/// ViewKey
pub use view::ViewKey;

mod decode;
mod errors;
mod permutation;
mod spend;
mod view;

use dusk_jubjub::{JubJubAffine, JubJubExtended, JubJubScalar};
