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

/// Structure repesenting a [`PublicKey`]
#[derive(Copy, Clone, HexDebug)]
#[cfg_attr(feature = "canon", derive(Canon))]
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

mod tests {

    #[test]
    fn partial_eq_test() {
        use super::*;
        use dusk_jubjub::{JubJubAffine, JubJubScalar};
        use rand_core::OsRng;

        let sk1 = PublicKey::from(&SecretKey::random(&mut OsRng));
        let sk2 = PublicKey::from(&SecretKey::random(&mut OsRng));

        assert_eq!(sk1, sk1);
        assert_ne!(sk1, sk2);

        // With all coordinates being different the points are the same ie.
        // equalty holds using this technique.
        let a = JubJubScalar::from(2u64);
        let b = JubJubScalar::from(7u64);
        let c = JubJubScalar::from(4u64);
        let d = JubJubScalar::from(5u64);
        let e = JubJubScalar::from(567758785u64);

        let left: JubJubExtended = dusk_jubjub::GENERATOR_EXTENDED * a
            + dusk_jubjub::GENERATOR_EXTENDED * b;

        let right: JubJubExtended = dusk_jubjub::GENERATOR_EXTENDED * c
            + dusk_jubjub::GENERATOR_EXTENDED * d;

        let wrong: JubJubExtended = dusk_jubjub::GENERATOR_EXTENDED * c
            + dusk_jubjub::GENERATOR_EXTENDED * e;

        // Assert none of the points coordinates actually matches
        assert_ne!(left.get_x(), right.get_x());
        assert_ne!(left.get_y(), right.get_y());
        assert_ne!(left.get_z(), right.get_z());

        assert_eq!(JubJubAffine::from(right), JubJubAffine::from(left));

        assert_eq!(PublicKey::from(left), PublicKey::from(right));
        assert_ne!(PublicKey::from(left), PublicKey::from(wrong))
    }
}
