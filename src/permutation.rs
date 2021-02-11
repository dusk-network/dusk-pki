// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use crate::{JubJubExtended, JubJubScalar};

use dusk_poseidon::sponge;

/// Hashes a JubJub's ExtendedPoint into a JubJub's Scalar
pub fn hash(p: &JubJubExtended) -> JubJubScalar {
    let h = sponge::hash(&p.to_hash_inputs());

    JubJubScalar::from_raw(h.reduce().0)
}
