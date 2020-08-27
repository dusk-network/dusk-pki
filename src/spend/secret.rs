// Copyright (c) DUSK NETWORK. All rights reserved.
// Licensed under the MPL 2.0 license. See LICENSE file in the project root for details.

use super::public::PublicKey;
use super::stealth::StealthAddress;

use crate::sponge;
use crate::{JubJubScalar, ViewKey};

use dusk_jubjub::GENERATOR_EXTENDED;

use std::fmt;

use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::{CryptoRng, RngCore};
use sha2::{Digest, Sha256};

/// Secret pair of `a` and `b`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SecretKey {
    a: JubJubScalar,
    b: JubJubScalar,
}

impl Default for SecretKey {
    fn default() -> Self {
        SecretKey::random(&mut rand::thread_rng())
    }
}

impl SecretKey {
    /// This method is used to construct a new `SecretKey` from the given secret
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

    /// Generate a `sk_r = H(a · R) + b`
    pub fn sk_r(&self, sa: &StealthAddress) -> JubJubScalar {
        let aR = sa.R() * self.a;
        let aR = sponge::hash(&aR);
        aR + self.b
    }

    /// Deterministically create a new [`SecretKey`] from a random number
    /// generator
    pub fn random<R: RngCore + CryptoRng>(rng: &mut R) -> Self {
        let a = JubJubScalar::random(rng);
        let b = JubJubScalar::random(rng);

        SecretKey::new(a, b)
    }

    /// Derive the secret to deterministically construct a [`PublicKey`]
    pub fn public_key(&self) -> PublicKey {
        let A = GENERATOR_EXTENDED * &self.a;
        let B = GENERATOR_EXTENDED * &self.b;

        PublicKey::new(A, B)
    }

    /// Derive the secret to deterministically construct a [`ViewKey`]
    pub fn view_key(&self) -> ViewKey {
        let B = GENERATOR_EXTENDED * &self.b;

        ViewKey::new(self.a, B)
    }
}

impl From<&SecretKey> for [u8; 64] {
    fn from(pk: &SecretKey) -> Self {
        let mut bytes = [0u8; 64];
        bytes[..32].copy_from_slice(&JubJubScalar::from(pk.a).to_bytes()[..]);
        bytes[32..].copy_from_slice(&JubJubScalar::from(pk.b).to_bytes()[..]);
        bytes
    }
}

impl From<&[u8]> for SecretKey {
    fn from(bytes: &[u8]) -> Self {
        let mut hasher = Sha256::default();
        hasher.input(bytes);
        let bytes = hasher.result();

        let mut seed = [0u8; 32];
        seed.copy_from_slice(&bytes[..32]);

        SecretKey::random(&mut StdRng::from_seed(seed))
    }
}

impl From<String> for SecretKey {
    fn from(s: String) -> Self {
        Self::from(s.into_bytes().as_slice())
    }
}

impl fmt::LowerHex for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes: [u8; 64] = self.into();

        if f.alternate() {
            write!(f, "0x")?
        }

        for byte in &bytes {
            write!(f, "{:02x}", &byte)?
        }
        Ok(())
    }
}

impl fmt::UpperHex for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes: [u8; 64] = self.into();

        if f.alternate() {
            write!(f, "0x")?
        }

        for byte in &bytes {
            write!(f, "{:02X}", &byte)?
        }
        Ok(())
    }
}

impl fmt::Display for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self)
    }
}
