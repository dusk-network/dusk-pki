// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use super::secret::SecretKey;
use crate::{JubJubAffine, JubJubExtended};
use dusk_bytes::{Error, HexDebug, Serializable};
use dusk_jubjub::GENERATOR_EXTENDED;

#[cfg(feature = "canon")]
use canonical_derive::Canon;

#[cfg(feature = "rkyv-impl")]
use rkyv::{Archive, Deserialize, Serialize};

/// Structure repesenting a [`PublicKey`]
#[derive(Copy, Clone, HexDebug)]
#[cfg_attr(feature = "canon", derive(Canon))]
#[cfg_attr(feature = "rkyv-impl", derive(Archive, Serialize, Deserialize))]
pub struct PublicKey(pub(crate) JubJubExtended);

impl From<&SecretKey> for PublicKey {
    fn from(sk: &SecretKey) -> Self {
        let public_key = GENERATOR_EXTENDED * sk.0;

        PublicKey(public_key)
    }
}

impl PartialEq for PublicKey {
    fn eq(&self, other: &Self) -> bool {
        self.0.get_x() * other.0.get_z() == other.0.get_x() * self.0.get_z()
            && self.0.get_y() * other.0.get_z()
                == other.0.get_y() * self.0.get_z()
    }
}

impl Eq for PublicKey {}

impl From<JubJubExtended> for PublicKey {
    fn from(p: JubJubExtended) -> PublicKey {
        PublicKey(p)
    }
}

impl From<&JubJubExtended> for PublicKey {
    fn from(p: &JubJubExtended) -> PublicKey {
        PublicKey(*p)
    }
}

impl AsRef<JubJubExtended> for PublicKey {
    fn as_ref(&self) -> &JubJubExtended {
        &self.0
    }
}

impl Serializable<32> for PublicKey {
    type Error = Error;

    fn to_bytes(&self) -> [u8; 32] {
        JubJubAffine::from(self.0).to_bytes()
    }

    fn from_bytes(bytes: &[u8; 32]) -> Result<Self, Error> {
        Ok(Self(JubJubAffine::from_bytes(bytes)?.into()))
    }
}

impl PublicKey {
    /// Create a public key from its internal parts
    ///
    /// The public keys are generated from a bijective function that takes a
    /// secret keys domain. If keys are generated directly from curve
    /// points, there is no guarantee a secret key exists - in fact, the
    /// discrete logarithm property will guarantee the secret key cannot be
    /// extracted from this public key.
    ///
    /// If you opt to generate the keys manually, be sure you have its secret
    /// counterpart - otherwise this key will be of no use.
    pub const fn from_raw_unchecked(key: JubJubExtended) -> Self {
        Self(key)
    }
}
