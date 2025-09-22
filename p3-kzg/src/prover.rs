// Generates the Commitment
// And the proof as well
use ark_bls12_381::g1::Config;

use ark_bls12_381::g2::Config as cfg;
use ark_bls12_381::{FrConfig, G1Projective};

use ark_ec::PrimeGroup;
use ark_ec::short_weierstrass::Projective;
use ark_ff::UniformRand;
use ark_ff::{Fp, MontBackend, PrimeField};

// [c] = summation pi * [s]1 * i
pub fn generate_commitment(
    polynomial: Vec<Fp<MontBackend<FrConfig, 4>, 4>>,
    powers_of_s: (Vec<Projective<Config>>, Vec<Projective<cfg>>),
) -> Projective<ark_bls12_381::g1::Config> {
    let s1 = powers_of_s.0;
    let mut rng = ark_std::test_rng();
    let mut commitment: Projective<Config> = G1Projective::rand(&mut rng);
    let extra = commitment;
    assert_eq!(polynomial.len(), s1.len());
    for i in 0..polynomial.len() {
        commitment += s1[i].mul_bigint(polynomial[i].into_bigint());
    }
    commitment - extra
}
