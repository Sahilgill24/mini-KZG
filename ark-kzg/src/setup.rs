// trusted setup

use ark_bls12_381::g1::Config;

use ark_bls12_381::g2::Config as cfg;
use ark_bls12_381::{Fr, FrConfig, G1Affine, G2Affine};

use ark_ec::AffineRepr;
use ark_ec::short_weierstrass::Projective;
use ark_ff::{BigInt, UniformRand};
use ark_ff::{Fp, MontBackend, PrimeField};
use ark_poly::univariate::DensePolynomial;
use ark_poly::{DenseUVPolynomial, Polynomial};

/// Secret Generation
/// Currently just using the rand function, but can simulate a MPC also.
/// Obviously this is a scalar
pub fn generate_secret() -> Fp<MontBackend<FrConfig, 4>, 4> {
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
    let g1 = G1Affine::generator();
    let g2 = G2Affine::generator();
    let mut store1 = Vec::new();
    let mut store2 = Vec::new();
    let mut temp = Fr::new(BigInt::one());

    for _i in 0..n {
        store1.push(g1.mul_bigint(temp.into_bigint()));
        store2.push(g2.mul_bigint(temp.into_bigint()));
        temp = temp * secret;
    }
    // These are two vectors of g1 and g2 elements.
    (store1, store2)
}

/// n is the degree of the polynomial
/// whether to use radix poly's or to use DensePolynomials

pub fn generate_polynomial(n: i32) -> Vec<Fp<MontBackend<FrConfig, 4>, 4>> {
    let mut rng = ark_std::test_rng();
    let poly = DensePolynomial::rand((n - 1) as usize, &mut rng);
    poly.to_vec()
}

/// evaluate the polynomial at z (convert z to a group element before though)
/// and find the value P(z)
/// q(x) = (p(x) -y )/ (x - z)
pub fn evaluate_polynomial(
    polynomial: Vec<Fp<MontBackend<FrConfig, 4>, 4>>,
    z: i32,
) -> (
    Fp<MontBackend<FrConfig, 4>, 4>,
    Fp<MontBackend<FrConfig, 4>, 4>,
) {
    let poly = DensePolynomial::from_coefficients_vec(polynomial);
    let y = poly.evaluate(&Fr::new(BigInt::from(z as u64)));
    let z_group = Fr::from(BigInt::from(z as u64));
    // z, p(z)
    (z_group, y)
}
