use crate::{JubJubAffine, JubJubExtended, JubJubScalar};
use poseidon252::sponge::sponge::sponge_hash;

pub fn hash(p: &JubJubExtended) -> JubJubScalar {
    // The same AffinePoint can have different ExtendedPoint representations,
    // therefore we convert from Extended to Affine before hashing, to ensure
    // deterministic result
    let p = JubJubAffine::from(p);
    JubJubScalar::from_raw(sponge_hash(&[p.get_x(), p.get_y()]).reduce().0)
}
