use core::num;

// Generates the Commitment
// And the proof as well
use ark_bls12_381::g1::Config;

use ark_bls12_381::g2::Config as cfg;
use ark_bls12_381::{Fr, FrConfig, G1Projective};

use ark_ec::PrimeGroup;
use ark_ec::short_weierstrass::Projective;
use ark_ff::{Field, UniformRand};
use ark_ff::{Fp, MontBackend, PrimeField};
use ark_poly::DenseUVPolynomial;
use ark_poly::univariate::{DenseOrSparsePolynomial, DensePolynomial};

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

/// [q]1 this is the kzg proof.
/// sigma qi.[si]
pub fn generate_proof(
    polynomial: Vec<Fp<MontBackend<FrConfig, 4>, 4>>,
    powers_of_s: (Vec<Projective<Config>>, Vec<Projective<cfg>>),
    evaluation_points: (
        Fp<MontBackend<FrConfig, 4>, 4>,
        Fp<MontBackend<FrConfig, 4>, 4>,
    ),
) -> Projective<ark_bls12_381::g1::Config> {
    let mut poly = polynomial;
    let s1 = powers_of_s.0;
    let z = evaluation_points.0;
    let y = evaluation_points.1;
    poly[0 as usize] = poly[0 as usize] - y;
    let one = Fr::ONE;
    let store2 = vec![-z, one];
    let denominator = DensePolynomial::from_coefficients_vec(store2);
    let numerator = DensePolynomial::from_coefficients_vec(poly);
    // now q(x) = numerator/ denominator simply
    let q = DenseOrSparsePolynomial::divide_with_q_and_r(&numerator.into(), &denominator.into());
    let quotient = q.unwrap().0.to_vec();
    let mut rng = ark_std::test_rng();
    let mut proof: Projective<Config> = G1Projective::rand(&mut rng);
    let extra = proof;
    assert_eq!(s1.len(), quotient.len());
    for i in 0..quotient.len() {
        proof += s1[i].mul_bigint(quotient[i].into_bigint());
    }
    proof - extra
}
