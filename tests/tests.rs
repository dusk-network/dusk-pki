// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use dusk_bytes::{DeserializableSlice, ParseHexStr, Serializable};
use dusk_jubjub::{JubJubAffine, JubJubExtended, JubJubScalar};
use dusk_pki::{PublicKey, PublicSpendKey, SecretKey, SecretSpendKey, ViewKey};
use rand_core::OsRng;

#[test]
fn ssk_from_bytes() {
    let ssk_a = SecretSpendKey::random(&mut OsRng);
    let bytes = ssk_a.to_bytes();
    let ssk_b = SecretSpendKey::from_slice(&bytes).expect("Serde error");

    assert_eq!(ssk_a, ssk_b);
}

#[test]
fn keys_encoding() {
    let ssk = SecretSpendKey::random(&mut OsRng);
    let vk = ssk.view_key();
    let psk = ssk.public_spend_key();

    assert_eq!(
        vk,
        ViewKey::from_hex_str(format!("{:x}", vk).as_str()).unwrap()
    );
    assert_eq!(
        psk,
        PublicSpendKey::from_hex_str(format!("{:x}", psk).as_str()).unwrap()
    );
}

#[test]
fn keys_consistency() {
    use dusk_jubjub::{JubJubScalar, GENERATOR_EXTENDED};

    let r = JubJubScalar::random(&mut OsRng);
    let ssk = SecretSpendKey::random(&mut OsRng);
    let psk = ssk.public_spend_key();
    let vk = ssk.view_key();
    let sa = psk.gen_stealth_address(&r);

    assert!(vk.owns(&sa));

    let wrong_ssk = SecretSpendKey::random(&mut OsRng);
    let wrong_vk = wrong_ssk.view_key();

    assert_ne!(ssk, wrong_ssk);
    assert_ne!(vk, wrong_vk);

    assert!(!wrong_vk.owns(&sa));

    let sk_r = ssk.sk_r(&sa);
    let wrong_sk_r = wrong_ssk.sk_r(&sa);

    assert_eq!(sa.address(), &(GENERATOR_EXTENDED * sk_r.as_ref()));
    assert_ne!(sa.address(), &(GENERATOR_EXTENDED * wrong_sk_r.as_ref()));
}

#[test]
#[allow(clippy::eq_op)]
fn partial_eq_pk() {
    let sk1 = SecretKey::random(&mut OsRng);
    let sk2 = SecretKey::random(&mut OsRng);

    assert_ne!(sk1, sk2);

    let pk1 = PublicKey::from(&sk1);
    let pk2 = PublicKey::from(&sk2);

    assert_eq!(pk1, pk1);
    assert_ne!(pk1, pk2);

    // With all coordinates being different the points are the same ie.
    // equalty holds using this technique.
    let s = (
        JubJubScalar::from(2u64),
        JubJubScalar::from(7u64),
        JubJubScalar::from(4u64),
        JubJubScalar::from(5u64),
        JubJubScalar::from(567758785u64),
    );

    let left: JubJubExtended = dusk_jubjub::GENERATOR_EXTENDED * s.0
        + dusk_jubjub::GENERATOR_EXTENDED * s.1;

    let right: JubJubExtended = dusk_jubjub::GENERATOR_EXTENDED * s.2
        + dusk_jubjub::GENERATOR_EXTENDED * s.3;

    let wrong: JubJubExtended = dusk_jubjub::GENERATOR_EXTENDED * s.2
        + dusk_jubjub::GENERATOR_EXTENDED * s.4;

    // Assert none of the points coordinates actually matches
    assert_ne!(left.get_x(), right.get_x());
    assert_ne!(left.get_y(), right.get_y());
    assert_ne!(left.get_z(), right.get_z());

    assert_eq!(JubJubAffine::from(right), JubJubAffine::from(left));

    assert_eq!(PublicKey::from(left), PublicKey::from(right));
    assert_ne!(PublicKey::from(left), PublicKey::from(wrong))
}
