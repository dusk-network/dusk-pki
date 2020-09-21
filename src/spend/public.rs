// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use super::secret::SecretKey;
use super::stealth::StealthAddress;

use crate::sponge;
use crate::{
    decode::decode, Error, JubJubAffine, JubJubExtended, JubJubScalar,
};

use dusk_jubjub::GENERATOR_EXTENDED;
use std::convert::TryFrom;
use std::fmt;
use subtle::{Choice, ConstantTimeEq};

/// Public pair of `a·G` and `b·G`
#[derive(Debug, Clone, Copy)]
pub struct PublicKey {
    A: JubJubExtended,
    B: JubJubExtended,
}

impl PublicKey {
    /// This method is used to construct a new `PublicKey` from the given public
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
        let rA = sponge::hash(&rA);
        let rA = G * rA;

        let pk_r = rA + self.B;

        StealthAddress { R, pk_r }
    }
}

impl ConstantTimeEq for PublicKey {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.A.ct_eq(&other.A) & self.B.ct_eq(&other.B)
    }
}

impl PartialEq for PublicKey {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(&other).into()
    }
}

impl Eq for PublicKey {}

impl Default for PublicKey {
    fn default() -> Self {
        SecretKey::default().public_key()
    }
}

impl From<SecretKey> for PublicKey {
    fn from(secret: SecretKey) -> Self {
        secret.public_key()
    }
}

impl From<&SecretKey> for PublicKey {
    fn from(secret: &SecretKey) -> Self {
        secret.public_key()
    }
}

impl From<&PublicKey> for [u8; 64] {
    fn from(pk: &PublicKey) -> [u8; 64] {
        let mut bytes = [0u8; 64];
        bytes[..32].copy_from_slice(&JubJubAffine::from(pk.A).to_bytes()[..]);
        bytes[32..].copy_from_slice(&JubJubAffine::from(pk.B).to_bytes()[..]);
        bytes
    }
}

impl TryFrom<String> for PublicKey {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.len() != 128 {
            return Err(Error::BadLength {
                found: s.len(),
                expected: 128,
            });
        }

        let s = s.as_str();

        let A = hex::decode(&s[..64]).map_err(|_| Error::InvalidPoint)?;
        let A = JubJubExtended::from(decode::<JubJubAffine>(&A[..])?);

        let B = hex::decode(&s[64..]).map_err(|_| Error::InvalidPoint)?;
        let B = JubJubExtended::from(decode::<JubJubAffine>(&B[..])?);

        Ok(PublicKey::new(A, B))
    }
}

impl fmt::LowerHex for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes: [u8; 64] = self.into();

        if f.alternate() {
            write!(f, "0x")?
        }

        bytes[..].iter().for_each(|byte| {
            write!(f, "{:02X}", &byte)
                .expect("Unexpected problem while writing bytes.")
        });
        Ok(())
    }
}

impl fmt::UpperHex for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes: [u8; 64] = self.into();

        if f.alternate() {
            write!(f, "0x")?
        }

        bytes[..].iter().for_each(|byte| {
            write!(f, "{:02X}", &byte)
                .expect("Unexpected problem while writing bytes.")
        });
        Ok(())
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self)
    }
}
