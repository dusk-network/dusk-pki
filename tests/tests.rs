// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

mod std_tests {
    use core::convert::TryFrom;
    use dusk_pki::{PublicSpendKey, SecretSpendKey, ViewKey};
    use rand::SeedableRng;

    fn ssk_from_str(s: &str) -> SecretSpendKey {
        use rand::rngs::StdRng;
        use sha2::{Digest, Sha256};

        let bytes = s.as_bytes();

        let mut hasher = Sha256::default();
        hasher.input(bytes);
        let bytes = hasher.result();

        let mut seed = [0u8; 32];
        seed.copy_from_slice(&bytes[..32]);

        SecretSpendKey::random(&mut StdRng::from_seed(seed))
    }

    #[test]
    fn ssk_from_bytes() {
        let ssk_a = ssk_from_str("some bytes");
        let ssk_b = ssk_from_str("some bytes");

        assert_eq!(ssk_a, ssk_b);
    }

    #[test]
    fn keys_encoding() {
        let ssk = ssk_from_str("some bytes");
        let vk = ssk.view_key();
        let psk = ssk.public_key();

        assert_eq!(vk, ViewKey::try_from(format!("{}", vk).as_str()).unwrap());
        assert_eq!(
            psk,
            PublicSpendKey::try_from(format!("{}", psk).as_str()).unwrap()
        );
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
