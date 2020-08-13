use dusk_pki::{PublicSpendKey, SecretSpendKey, ViewKey};

use std::convert::TryFrom;

#[test]
fn ssk_from_bytes() {
    let bytes = b"some bytes".to_vec();

    let ssk_a = SecretSpendKey::from(&bytes[..]);
    let ssk_b = SecretSpendKey::from(&bytes[..]);

    assert_eq!(ssk_a, ssk_b);
}

#[test]
fn keys_encoding() -> anyhow::Result<()> {
    let bytes = b"some bytes".to_vec();

    let ssk = SecretSpendKey::from(bytes.as_slice());
    let vk = ssk.view_key();
    let psk = ssk.public_key();

    assert_eq!(vk, ViewKey::try_from(format!("{}", vk))?);
    assert_eq!(psk, PublicSpendKey::try_from(format!("{}", psk))?);
    Ok(())
}

#[test]
fn keys_consistency() {
    use dusk_jubjub::{Fr as JubJubScalar, GENERATOR_EXTENDED};

    let r = JubJubScalar::random(&mut rand::thread_rng());
    let ssk = SecretSpendKey::default();
    let psk = ssk.public_key();
    let vk = ssk.view_key();
    let sa = psk.gen_stealth_address(&r);

    assert!(vk.owns(&sa));

    let wrong_ssk = SecretSpendKey::default();
    let wrong_vk = wrong_ssk.view_key();

    assert_ne!(ssk, wrong_ssk);
    assert_ne!(vk, wrong_vk);

    assert!(!wrong_vk.owns(&sa));

    let sk_r = ssk.sk_r(&sa);
    let wrong_sk_r = wrong_ssk.sk_r(&sa);

    assert_eq!(sa.pk_r(), &(GENERATOR_EXTENDED * &sk_r));
    assert_ne!(sa.pk_r(), &(GENERATOR_EXTENDED * &wrong_sk_r));
}
