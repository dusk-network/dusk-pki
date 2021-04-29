// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use crate::{permutation, JubJubScalar, SecretKey, ViewKey};

use super::public::PublicSpendKey;
use super::stealth::StealthAddress;

#[cfg(feature = "canon")]
use canonical_derive::Canon;

use dusk_bytes::{DeserializableSlice, Error, HexDebug, Serializable};
use dusk_jubjub::GENERATOR_EXTENDED;
use rand_core::{CryptoRng, RngCore};
use subtle::{Choice, ConstantTimeEq};

/// Secret pair of `a` and `b` defining a [`SecretSpendKey`]
#[derive(Clone, Copy, Eq, HexDebug)]
#[cfg_attr(feature = "canon", derive(Canon))]
pub struct SecretSpendKey {
    a: JubJubScalar,
    b: JubJubScalar,
}

impl SecretSpendKey {
    /// This method is used to construct a new `SecretSpendKey` from the given
    /// secret pair of `a` and `b`.
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

    /// Generates a [`SecretKey`] using the [`StealthAddress`] given.
    /// With the formula: `sk_r = H(a · R) + b`
    pub fn sk_r(&self, sa: &StealthAddress) -> SecretKey {
        let aR = sa.R() * self.a;
        let aR = permutation::hash(&aR);

        SecretKey(aR + self.b)
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

impl ConstantTimeEq for SecretSpendKey {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.a.ct_eq(&other.a) & self.b.ct_eq(&other.b)
    }
}

impl PartialEq for SecretSpendKey {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(&other).into()
    }
}

impl Serializable<64> for SecretSpendKey {
    type Error = Error;

    fn to_bytes(&self) -> [u8; 64] {
        let mut bytes = [0u8; 64];
        bytes[..32].copy_from_slice(&self.a.to_bytes());
        bytes[32..].copy_from_slice(&self.b.to_bytes());
        bytes
    }

    fn from_bytes(buf: &[u8; 64]) -> Result<Self, Self::Error> {
        let a = JubJubScalar::from_slice(&buf[..32])?;
        let b = JubJubScalar::from_slice(&buf[32..])?;

        Ok(Self { a, b })
    }
}
