// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use crate::{JubJubAffine, JubJubExtended, PublicKey};

#[cfg(feature = "canon")]
use canonical_derive::Canon;
use dusk_bytes::{DeserializableSlice, Error, HexDebug, Serializable};

use subtle::{Choice, ConstantTimeEq};

/// To obfuscate the identity of the participants, we utilizes a Stealth Address
/// system.
/// A `StealthAddress` is composed by a one-time public key (`pk_r`, the actual
// address) and a random point `R`.
#[derive(HexDebug, Clone, Copy)]
#[cfg_attr(feature = "canon", derive(Canon))]
pub struct StealthAddress {
    pub(crate) R: JubJubExtended,
    pub(crate) pk_r: PublicKey,
}

/// The trait `Ownable` is required by any type that wants to prove its
/// ownership.
pub trait Ownable {
    /// Returns the associated `StealthAddress`
    fn stealth_address(&self) -> &StealthAddress;
}

impl StealthAddress {
    /// Gets the random point `R`
    pub fn R(&self) -> &JubJubExtended {
        &self.R
    }

    /// Gets the `pk_r`
    pub fn pk_r(&self) -> &PublicKey {
        &self.pk_r
    }

    /// Gets the underline `JubJubExtended` point of `pk_r`
    pub fn address(&self) -> &JubJubExtended {
        &self.pk_r.as_ref()
    }
}

impl ConstantTimeEq for StealthAddress {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.pk_r.as_ref().ct_eq(&other.pk_r.as_ref()) & self.R.ct_eq(&other.R)
    }
}

impl PartialEq for StealthAddress {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(&other).into()
    }
}

impl Ownable for StealthAddress {
    fn stealth_address(&self) -> &StealthAddress {
        &self
    }
}

impl Serializable<64> for StealthAddress {
    type Error = Error;
    /// Encode the `StealthAddress` to an array of 64 bytes
    fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut bytes = [0u8; Self::SIZE];
        bytes[..32].copy_from_slice(&JubJubAffine::from(self.R).to_bytes());
        bytes[32..].copy_from_slice(
            &JubJubAffine::from(self.pk_r.as_ref()).to_bytes(),
        );
        bytes
    }

    /// Decode the `StealthAddress` from an array of 64 bytes
    fn from_bytes(bytes: &[u8; Self::SIZE]) -> Result<Self, Error> {
        let R = JubJubExtended::from(JubJubAffine::from_slice(&bytes[..32])?);
        let pk_r =
            JubJubExtended::from(JubJubAffine::from_slice(&bytes[32..])?);
        let pk_r = PublicKey(pk_r);

        Ok(StealthAddress { R, pk_r })
    }
}
