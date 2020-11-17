// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

#[cfg(feature = "std")]
mod std_tests {
    use core::convert::TryFrom;
    use dusk_pki::{PublicSpendKey, SecretSpendKey, ViewKey};

    #[test]
    fn ssk_from_bytes() {
        let bytes = b"some bytes".to_vec();

        let ssk_a = SecretSpendKey::from(&bytes[..]);
        let ssk_b = SecretSpendKey::from(&bytes[..]);

        assert_eq!(ssk_a, ssk_b);
    }

    #[test]
    fn keys_encoding() {
        let bytes = b"some bytes".to_vec();

        let ssk = SecretSpendKey::from(bytes.as_slice());
        let vk = ssk.view_key();
        let psk = ssk.public_key();

        assert_eq!(vk, ViewKey::try_from(format!("{}", vk)).unwrap());
        assert_eq!(psk, PublicSpendKey::try_from(format!("{}", psk)).unwrap());
    }

    #[test]
    fn keys_consistency() {
        use dusk_jubjub::{JubJubScalar, GENERATOR_EXTENDED};

        let r = JubJubScalar::random(&mut rand::thread_rng());
        let ssk = SecretSpendKey::random(&mut rand::thread_rng());
        let psk = ssk.public_key();
        let vk = ssk.view_key();
        let sa = psk.gen_stealth_address(&r);

        assert!(vk.owns(&sa));

        let wrong_ssk = SecretSpendKey::random(&mut rand::thread_rng());
        let wrong_vk = wrong_ssk.view_key();

        assert_ne!(ssk, wrong_ssk);
        assert_ne!(vk, wrong_vk);

        assert!(!wrong_vk.owns(&sa));

        let sk_r = ssk.sk_r(&sa);
        let wrong_sk_r = wrong_ssk.sk_r(&sa);

        assert_eq!(sa.pk_r(), &(GENERATOR_EXTENDED * &sk_r));
        assert_ne!(sa.pk_r(), &(GENERATOR_EXTENDED * &wrong_sk_r));
    }
}
