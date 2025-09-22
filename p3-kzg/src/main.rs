#![allow(unused)]
use ark_bls12_381::{Bls12_381, Fr, FrConfig, G1Affine};
use ark_ec::pairing;
use ark_ff::UniformRand;
use ark_ff::{Fp, MontBackend, PrimeField};
use ark_poly::{EvaluationDomain, Radix2EvaluationDomain, DenseUVPolynomial};

fn main() {
    let secret = generate_polynomial(12);
    println!("{:?}", secret);
    let mut rng = ark_std::test_rng();
    let val = Fr::rand(&mut rng);

}

/// Represents the group elements for commitment and proof.
struct KZGparams {
    commitment: G1Affine,
    proof: G1Affine,
}

fn verification() {}

/// Function to generate [s^i]1 , [s^i]2
/// These will be provided to the prover and verifier.
/// G1 x G2 -> Gt (for BLS_12_381 curve)
/// MPC can be used to generate these group elements after redistribution
fn group_elements(n: i32) {}

/// n is the degree of the polynomial
fn generate_polynomial(n: i32) -> Vec<Fp<MontBackend<FrConfig, 4>, 4>> {
    let mut rng = ark_std::test_rng();
    let mut store = Vec::new();
    let domain = Radix2EvaluationDomain::<Fr>::new(n as usize).unwrap();
    for _i in 0..n {
        store.push(Fr::rand(&mut rng));
    }
    let poly = domain.fft(&mut store);
    poly
}

// understanding the need to use lagrange basis for everything here, also
// and no need to take .fft here ig 


/// Secret Generation
/// Currently just using the rand function, but can simulate a MPC also.
fn generate_secret() -> Fp<MontBackend<FrConfig, 4>, 4> {
    let mut rng = ark_std::test_rng();
    let secret = Fr::rand(&mut rng);
    secret
}
