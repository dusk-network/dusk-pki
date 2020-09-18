// Copyright (c) DUSK NETWORK. All rights reserved.
// Licensed under the MPL 2.0 license. See LICENSE file in the project root for
// details.

use crate::{decode::decode, Error, JubJubAffine, JubJubExtended};

use std::convert::{TryFrom, TryInto};
use std::fmt;
use subtle::{Choice, ConstantTimeEq};

/// The trait `Ownable` is required by any type that wants to prove its
/// ownership.
pub trait Ownable {
    /// Returns the associated `StealthAddress`
    fn stealth_address(&self) -> &StealthAddress;
}

//. To obfuscate the identity of the participants, we utilizes a Stealth Address
//. system.
/// A `StealthAddress` is composed by a one-time public key (`pk_r`, the actual
// address) and a random point `R`.
#[derive(Debug, Clone, Copy)]
pub struct StealthAddress {
    pub(crate) R: JubJubExtended,
    pub(crate) pk_r: JubJubExtended,
}

impl Ownable for StealthAddress {
    fn stealth_address(&self) -> &StealthAddress {
        &self
    }
}

impl StealthAddress {
    /// Gets the random point `R`
    pub fn R(&self) -> &JubJubExtended {
        &self.R
    }

    /// Gets the `pk_r`
    pub fn pk_r(&self) -> &JubJubExtended {
        &self.pk_r
    }

    /// Alias to `pk_r()` method
    pub fn address(&self) -> &JubJubExtended {
        &self.pk_r
    }

    /// Encode the `StealthAddress` to an array of 64 bytes
    pub fn to_bytes(&self) -> [u8; 64] {
        self.into()
    }

    /// Decode the `StealthAddress` from an array of 64 bytes
    pub fn from_bytes(bytes: &[u8; 64]) -> Result<Self, Error> {
        bytes.try_into()
    }
}

impl ConstantTimeEq for StealthAddress {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.pk_r.ct_eq(&other.pk_r) & self.R.ct_eq(&other.R)
    }
}

impl PartialEq for StealthAddress {
    fn eq(&self, other: &Self) -> bool {
        self.ct_eq(&other).into()
    }
}

impl From<&StealthAddress> for [u8; 64] {
    fn from(sa: &StealthAddress) -> [u8; 64] {
        let mut bytes = [0u8; 64];
        bytes[..32].copy_from_slice(&JubJubAffine::from(sa.R).to_bytes()[..]);
        bytes[32..]
            .copy_from_slice(&JubJubAffine::from(sa.pk_r).to_bytes()[..]);
        bytes
    }
}

impl TryFrom<&[u8; 64]> for StealthAddress {
    type Error = Error;

    fn try_from(bytes: &[u8; 64]) -> Result<Self, Self::Error> {
        let R = JubJubExtended::from(decode::<JubJubAffine>(&bytes[..32])?);
        let pk_r = JubJubExtended::from(decode::<JubJubAffine>(&bytes[32..])?);

        Ok(StealthAddress { R, pk_r })
    }
}

impl TryFrom<String> for StealthAddress {
    type Error = Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.len() != 128 {
            return Err(Error::BadLength {
                found: s.len(),
                expected: 128,
            });
        }

        let s = s.as_str();

        let R = hex::decode(&s[..64]).map_err(|_| Error::InvalidPoint)?;
        let R = JubJubExtended::from(decode::<JubJubAffine>(&R[..])?);

        let pk_r = hex::decode(&s[64..]).map_err(|_| Error::InvalidPoint)?;
        let pk_r = JubJubExtended::from(decode::<JubJubAffine>(&pk_r[..])?);

        Ok(StealthAddress { R, pk_r })
    }
}

impl fmt::LowerHex for StealthAddress {
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

impl fmt::UpperHex for StealthAddress {
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

impl fmt::Display for StealthAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self)
    }
}
