use crate::{JubJubExtended, JubJubScalar};
use poseidon252::sponge::sponge::sponge_hash;

/// Hashes a JubJub's ExtendedPoint into a JubJub's Scalar
pub fn hash(p: &JubJubExtended) -> JubJubScalar {
    JubJubScalar::from_raw(sponge_hash(&p.to_hash_inputs()).reduce().0)
}
