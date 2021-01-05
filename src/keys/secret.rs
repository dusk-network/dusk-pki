// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use crate::{Error, JubJubScalar};
use core::convert::TryFrom;
use core::fmt;
use rand_core::{CryptoRng, RngCore};

#[allow(non_snake_case)]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "canon", derive(Canon))]
/// Structure repesenting a secret key
pub struct SecretKey(JubJubScalar);

impl From<JubJubScalar> for SecretKey {
    fn from(s: JubJubScalar) -> SecretKey {
        SecretKey(s)
    }
}

impl From<&JubJubScalar> for SecretKey {
    fn from(s: &JubJubScalar) -> SecretKey {
        SecretKey(*s)
    }
}

impl AsRef<JubJubScalar> for SecretKey {
    fn as_ref(&self) -> &JubJubScalar {
        &self.0
    }
}

impl SecretKey {
    /// This will create a random [`SecretKey`] from a scalar
    /// of the Field JubJubScalar.
    pub fn new<T>(rand: &mut T) -> SecretKey
    where
        T: RngCore + CryptoRng,
    {
        let fr = JubJubScalar::random(rand);

        SecretKey(fr)
    }

    /// Copies `self` into a new array of 32 bytes.
    pub fn to_bytes(&self) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&self.0.to_bytes());
        bytes
    }

    /// Create a new [`SecretKey`] from an array of 32 bytes.
    pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self, Error> {
        match Option::from(JubJubScalar::from_bytes(bytes)) {
            Some(scalar) => Ok(SecretKey(scalar)),
            _ => Err(Error::InvalidScalar),
        }
    }
}

impl fmt::LowerHex for SecretKey {
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

impl fmt::UpperHex for SecretKey {
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

impl fmt::Display for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self)
    }
}
