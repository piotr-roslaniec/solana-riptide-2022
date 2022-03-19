use ark_bls12_381::{Bls12_381, G1Affine, G2Affine};
use ark_ff::{Fp384, QuadExtField};
use ark_groth16::VerifyingKey;
use std::str::FromStr;

pub fn make_verifying_key() -> VerifyingKey<Bls12_381> {
    // TODO: Fill placeholders with generated values
    let alpha_g1 = G1Affine::new(
        Fp384::from_str("0x0").unwrap(),
        Fp384::from_str("0x0").unwrap(),
        false,
    );
    let beta_g2 = G2Affine::new(
        QuadExtField::new(
            Fp384::from_str("0x0").unwrap(),
            Fp384::from_str("0x0").unwrap(),
        ),
        QuadExtField::new(
            Fp384::from_str("0x0").unwrap(),
            Fp384::from_str("0x0").unwrap(),
        ),
        false,
    );
    let gamma_g2 = G2Affine::new(
        QuadExtField::new(
            Fp384::from_str("0x0").unwrap(),
            Fp384::from_str("0x0").unwrap(),
        ),
        QuadExtField::new(
            Fp384::from_str("0x0").unwrap(),
            Fp384::from_str("0x0").unwrap(),
        ),
        false,
    );
    let delta_g2 = G2Affine::new(
        QuadExtField::new(
            Fp384::from_str("0x0").unwrap(),
            Fp384::from_str("0x0").unwrap(),
        ),
        QuadExtField::new(
            Fp384::from_str("0x0").unwrap(),
            Fp384::from_str("0x0").unwrap(),
        ),
        false,
    );
    let mut gamma_abc_g1: Vec<G1Affine> = Vec::new();
    for i in 0..3 {
        gamma_abc_g1.push(G1Affine::new(
            Fp384::from_str("0x0").unwrap(),
            Fp384::from_str("0x0").unwrap(),
            false,
        ))
    }
    VerifyingKey {
        alpha_g1,
        beta_g2,
        gamma_g2,
        delta_g2,
        gamma_abc_g1,
    }
}

pub fn verify() {
    let vk = make_verifying_key();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let vk = make_verifying_key();
        // assert_eq!(add(1, 2), 3);
    }
}
