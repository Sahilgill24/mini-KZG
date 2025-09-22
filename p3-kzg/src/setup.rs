// trusted setup

use ark_bls12_381::g1::Config;

use ark_bls12_381::g2::Config as cfg;
use ark_bls12_381::{Fr, FrConfig, G1Affine, G2Affine};

use ark_ec::AffineRepr;
use ark_ec::short_weierstrass::Projective;
use ark_ff::{BigInt, UniformRand};
use ark_ff::{Fp, MontBackend, PrimeField};
use ark_poly::DenseUVPolynomial;
use ark_poly::univariate::DensePolynomial;

/// Secret Generation
/// Currently just using the rand function, but can simulate a MPC also.
/// Obviously this is a scalar
fn generate_secret() -> Fp<MontBackend<FrConfig, 4>, 4> {
    let mut rng = ark_std::test_rng();
    let secret = Fr::rand(&mut rng);
    secret
}

/// Function to generate [s^i]1 , [s^i]2
/// summation s^i * G1 and s^i * G2
/// These will be provided to the prover and verifier.
/// G1 x G2 -> Gt (for BLS_12_381 curve)
/// These are public to the Prover and verifier and not the secret.
/// MPC can be used to generate these group elements after redistribution
pub fn powers_of_s(
    n: i32,
    secret: Fp<MontBackend<FrConfig, 4>, 4>,
) -> (Vec<Projective<Config>>, Vec<Projective<cfg>>) {
    let mut rng = ark_std::test_rng();
    let g1 = G1Affine::rand(&mut rng);
    let g2 = G2Affine::rand(&mut rng);
    let mut store1 = Vec::new();
    let mut store2 = Vec::new();
    let mut temp = Fr::new(BigInt::one());

    for i in 0..n {
        store1.push(g1.mul_bigint(temp.into_bigint()));
        store2.push(g2.mul_bigint(temp.into_bigint()));
        temp = temp * secret;
    }
    // These are two vectors of g1 and g2 elements.
    (store1, store2)
}

/// n is the degree of the polynomial
/// whether to use radix poly's or to use DensePolynomials
///
fn generate_polynomial(n: i32) -> Vec<Fp<MontBackend<FrConfig, 4>, 4>> {
    let mut rng = ark_std::test_rng();
    let poly = DensePolynomial::rand(n as usize, &mut rng);
    poly.to_vec()
}
