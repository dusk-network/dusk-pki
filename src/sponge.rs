// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use crate::{JubJubExtended, JubJubScalar};
use poseidon252::sponge::sponge::sponge_hash;

/// Hashes a JubJub's ExtendedPoint into a JubJub's Scalar
pub fn hash(p: &JubJubExtended) -> JubJubScalar {
    JubJubScalar::from_raw(sponge_hash(&p.to_hash_inputs()).reduce().0)
}
