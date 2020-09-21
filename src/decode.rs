// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use crate::{Error, JubJubAffine, JubJubScalar};
use subtle::CtOption;

// TODO: move this logic (in this form or another) to jubjub repo, or higher
// repo than `dusk-pki`

pub trait From32Bytes {
    fn from_bytes(bytes: [u8; 32]) -> CtOption<Self>
    where
        Self: Sized;
}

impl From32Bytes for JubJubScalar {
    fn from_bytes(bytes: [u8; 32]) -> CtOption<Self> {
        JubJubScalar::from_bytes(&bytes)
    }
}
impl From32Bytes for JubJubAffine {
    fn from_bytes(bytes: [u8; 32]) -> CtOption<Self> {
        JubJubAffine::from_bytes(bytes)
    }
}

/// DecodesJubJub's AffinePoint and JubJub's Scalar from bytes
pub fn decode<T: From32Bytes>(bytes: &[u8]) -> Result<T, Error> {
    if bytes.len() < 32 {
        return Err(Error::BadLength {
            found: bytes.len(),
            expected: 32,
        });
    }

    let mut array = [0u8; 32];
    array.copy_from_slice(&bytes[..32]);
    let result = T::from_bytes(array);

    if result.is_none().into() {
        return Err(Error::InvalidParameters);
    }
    Ok(result.unwrap())
}
