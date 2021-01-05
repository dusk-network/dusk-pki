// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use crate::Error;
use crate::{permutation, JubJubAffine, JubJubExtended, JubJubScalar, StealthAddress};

use super::secret::SecretSpendKey;

#[cfg(feature = "canon")]
use canonical::Canon;
#[cfg(feature = "canon")]
use canonical_derive::Canon;

use dusk_jubjub::GENERATOR_EXTENDED;
use subtle::{Choice, ConstantTimeEq};

use core::convert::TryFrom;
use core::fmt;

/// Public pair of `a·G` and `b·G`
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "canon", derive(Canon))]
pub struct PublicSpendKey {
    A: JubJubExtended,
    B: JubJubExtended,
}

impl PublicSpendKey {
    /// This method is used to construct a new `PublicSpendKey` from the given public
    /// pair of `a·G` and `b·G`
    pub fn new(A: JubJubExtended, B: JubJubExtended) -> Self {
        Self { A, B }
    }

    /// Gets `A` (`a·G`)
    pub fn A(&self) -> &JubJubExtended {
        &self.A
    }

    /// Gets `B` (`b·G`)
    pub fn B(&self) -> &JubJubExtended {
        &self.B
    }

    /// Generates new `PKr = H(A · r) · G + B` from a given `r`
    pub fn gen_stealth_address(&self, r: &JubJubScalar) -> StealthAddress {
        let G = GENERATOR_EXTENDED;
        let R = G * r;

        let rA = self.A * r;
        let rA = permutation::hash(&rA);
        let rA = G * rA;

        let pk_r = rA + self.B;

        StealthAddress { R, pk_r }
    }
}

impl ConstantTimeEq for PublicSpendKey {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.A.ct_eq(&other.A) & self.B.ct_eq(&other.B)
    }
}

impl PartialEq for PublicSpendKey {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(&other).into()
    }
}

impl Eq for PublicSpendKey {}

impl Default for PublicSpendKey {
    fn default() -> Self {
        SecretSpendKey::default().public_spend_key()
    }
}

impl From<SecretSpendKey> for PublicSpendKey {
    fn from(secret: SecretSpendKey) -> Self {
        secret.public_spend_key()
    }
}

impl From<&SecretSpendKey> for PublicSpendKey {
    fn from(secret: &SecretSpendKey) -> Self {
        secret.public_spend_key()
    }
}

impl From<&PublicSpendKey> for [u8; 64] {
    fn from(pk: &PublicSpendKey) -> [u8; 64] {
        let mut bytes = [0u8; 64];
        bytes[..32].copy_from_slice(&JubJubAffine::from(pk.A).to_bytes()[..]);
        bytes[32..].copy_from_slice(&JubJubAffine::from(pk.B).to_bytes()[..]);
        bytes
    }
}

impl TryFrom<&str> for PublicSpendKey {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        use crate::decode::decode;

        if s.len() != 128 {
            return Err(Error::BadLength {
                found: s.len(),
                expected: 128,
            });
        }

        let A = hex::decode(&s[..64]).map_err(|_| Error::InvalidPoint)?;
        let A = JubJubExtended::from(decode::<JubJubAffine>(&A[..])?);

        let B = hex::decode(&s[64..]).map_err(|_| Error::InvalidPoint)?;
        let B = JubJubExtended::from(decode::<JubJubAffine>(&B[..])?);

        Ok(PublicSpendKey::new(A, B))
    }
}

impl fmt::LowerHex for PublicSpendKey {
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

impl fmt::UpperHex for PublicSpendKey {
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

impl fmt::Display for PublicSpendKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self)
    }
}
