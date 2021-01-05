// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use crate::{permutation, JubJubScalar, ViewKey};

use super::public::PublicSpendKey;
use super::stealth::StealthAddress;

#[cfg(feature = "canon")]
use canonical::Canon;
#[cfg(feature = "canon")]
use canonical_derive::Canon;

use dusk_jubjub::GENERATOR_EXTENDED;
use rand_core::{CryptoRng, RngCore};

use core::fmt;

/// Secret pair of `a` and `b`
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "canon", derive(Canon))]
pub struct SecretSpendKey {
    a: JubJubScalar,
    b: JubJubScalar,
}

impl SecretSpendKey {
    /// This method is used to construct a new `SecretSpendKey` from the given secret
    /// pair of `a` and `b`.
    pub fn new(a: JubJubScalar, b: JubJubScalar) -> Self {
        Self { a, b }
    }

    /// Gets `a`
    pub fn a(&self) -> &JubJubScalar {
        &self.a
    }

    /// Gets `b`
    pub fn b(&self) -> &JubJubScalar {
        &self.b
    }

    /// Deterministically create a new [`SecretSpendKey`] from a random number
    /// generator
    pub fn random<R: RngCore + CryptoRng>(rng: &mut R) -> Self {
        let a = JubJubScalar::random(rng);
        let b = JubJubScalar::random(rng);

        SecretSpendKey::new(a, b)
    }

    /// Generate a `sk_r = H(a · R) + b`
    pub fn sk_r(&self, sa: &StealthAddress) -> JubJubScalar {
        let aR = sa.R() * self.a;
        let aR = permutation::hash(&aR);

        aR + self.b
    }

    /// Derive the secret to deterministically construct a [`PublicSpendKey`]
    pub fn public_spend_key(&self) -> PublicSpendKey {
        let A = GENERATOR_EXTENDED * self.a;
        let B = GENERATOR_EXTENDED * self.b;

        PublicSpendKey::new(A, B)
    }

    /// Derive the secret to deterministically construct a [`ViewKey`]
    pub fn view_key(&self) -> ViewKey {
        let B = GENERATOR_EXTENDED * self.b;

        ViewKey::new(self.a, B)
    }
}

impl From<&SecretSpendKey> for [u8; 64] {
    fn from(pk: &SecretSpendKey) -> Self {
        let mut bytes = [0u8; 64];
        bytes[..32].copy_from_slice(&pk.a.to_bytes()[..]);
        bytes[32..].copy_from_slice(&pk.b.to_bytes()[..]);
        bytes
    }
}

impl fmt::LowerHex for SecretSpendKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes: [u8; 64] = self.into();

        if f.alternate() {
            write!(f, "0x")?
        }

        for byte in &bytes[..] {
            write!(f, "{:02X}", &byte)?
        }

        Ok(())
    }
}

impl fmt::UpperHex for SecretSpendKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes: [u8; 64] = self.into();

        if f.alternate() {
            write!(f, "0x")?
        }

        for byte in &bytes[..] {
            write!(f, "{:02X}", &byte)?
        }

        Ok(())
    }
}

impl fmt::Display for SecretSpendKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self)
    }
}
