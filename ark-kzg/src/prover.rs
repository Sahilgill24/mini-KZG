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

/// [c] = summation pi * [s]1 * i
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
/// q(x) = ( P(x) - P(z) )/ (x - z )
pub fn generate_proof(
    polynomial: Vec<Fp<MontBackend<FrConfig, 4>, 4>>,
    powers_of_s: (Vec<Projective<Config>>, Vec<Projective<cfg>>),
    evaluation_points: (
        Fp<MontBackend<FrConfig, 4>, 4>,
        Fp<MontBackend<FrConfig, 4>, 4>,
    ),
) -> Projective<ark_bls12_381::g1::Config> {
    // p(x)
    let mut poly = polynomial;
    // [s]1
    let s1 = powers_of_s.0;
    // z
    let z = evaluation_points.0;
    // y = p(z)
    let y = evaluation_points.1;
    poly[0 as usize] = poly[0 as usize] - y;
    let one = Fr::ONE;
    let store2 = vec![-z, one];
    // X - z
    let denominator = DensePolynomial::from_coefficients_vec(store2);
    // P(x) - P(z)
    let numerator = DensePolynomial::from_coefficients_vec(poly);
    // now q(x) = numerator/ denominator simply
    let q = DenseOrSparsePolynomial::divide_with_q_and_r(&numerator.into(), &denominator.into());
    let quotient = q.unwrap().0.to_vec();
    // rng
    let mut rng = ark_std::test_rng();
    // proof
    let mut proof: Projective<Config> = G1Projective::rand(&mut rng);
    let extra = proof;
    for i in 0..quotient.len() {
        // s
        proof += s1[i].mul_bigint(quotient[i].into_bigint());
    }
    proof - extra
}
