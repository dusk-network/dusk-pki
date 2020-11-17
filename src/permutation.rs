// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use crate::{JubJubExtended, JubJubScalar};

use dusk_bls12_381::BlsScalar;
use hades252::{ScalarStrategy, Strategy};

use core::cmp;

/// Hashes a JubJub's ExtendedPoint into a JubJub's Scalar
pub fn hash(p: &JubJubExtended) -> JubJubScalar {
    let mut perm = [BlsScalar::zero(); hades252::WIDTH];
    let p = p.to_hash_inputs();

    let n = cmp::min(hades252::WIDTH, p.len());

    perm[0..n].copy_from_slice(&p[0..n]);
    ScalarStrategy::new().perm(&mut perm);

    JubJubScalar::from_raw(perm[1].reduce().0)
}
