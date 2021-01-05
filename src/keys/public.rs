// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use super::secret::SecretKey;
use crate::{Error, JubJubAffine, JubJubExtended};
use core::convert::TryFrom;
use core::fmt;
use dusk_jubjub::GENERATOR_EXTENDED;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "canon", derive(Canon))]

/// Structure repesenting a [`PublicKey`]
pub struct PublicKey(JubJubExtended);

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

impl PublicKey {
    /// Copies `self` into a new array of 32 bytes.
    pub fn to_bytes(&self) -> [u8; 32] {
        JubJubAffine::from(self.0).to_bytes()
    }

    /// Create a new `PublicKey` from an array of 32 bytes.
    pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self, Error> {
        match Option::<JubJubAffine>::from(JubJubAffine::from_bytes(*bytes)) {
            Some(point) => Ok(PublicKey(JubJubExtended::from(point))),
            _ => Err(Error::InvalidPoint),
        }
    }
}

impl fmt::LowerHex for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.to_bytes();

        if f.alternate() {
            write!(f, "0x")?
        }

        for byte in &bytes[..] {
            write!(f, "{:02X}", &byte)?
        }

        Ok(())
    }
}

impl fmt::UpperHex for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.to_bytes();

        if f.alternate() {
            write!(f, "0x")?
        }

        for byte in &bytes[..] {
            write!(f, "{:02X}", &byte)?
        }

        Ok(())
    }
}

impl fmt::Display for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self)
    }
}
