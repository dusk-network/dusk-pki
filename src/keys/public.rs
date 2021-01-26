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
use canonical::Canon;
#[cfg(feature = "canon")]
use canonical_derive::Canon;

/// Structure repesenting a [`PublicKey`]
#[derive(Copy, Clone, PartialEq, HexDebug)]
#[cfg_attr(feature = "canon", derive(Canon))]
pub struct PublicKey(pub(crate) JubJubExtended);

impl From<&SecretKey> for PublicKey {
    fn from(sk: &SecretKey) -> Self {
        let public_key = GENERATOR_EXTENDED * sk.0;

        PublicKey(public_key)
    }
}

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
